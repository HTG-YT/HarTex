/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # Localization Bindings
//!
//! This crate provides a macro for generating localization bindings from Fluent files.

#![feature(proc_macro_diagnostic)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use fluent_bundle::FluentResource;
use fluent_syntax::ast::Entry;
use fluent_syntax::ast::Expression;
use fluent_syntax::ast::InlineExpression;
use fluent_syntax::ast::PatternElement;
use hartex_localization_loader::env::base_path;
use hartex_localization_loader::load_resources;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::TokenStreamExt;
use quote::quote;
use syn::GenericParam;
use syn::Ident;
use syn::LitStr;
use syn::TypeParam;

/// A localization node.
struct LocalizationNode<'a> {
    category: &'a str,
    name: &'a str,
    variables: HashSet<&'a str>,
    dependencies: HashSet<&'a str>,
    term: bool,
}

impl<'a> LocalizationNode<'a> {
    /// Constructs a new localization node.
    pub fn new(category: &'a str, name: &'a str, term: bool) -> Self {
        Self {
            category,
            name,
            variables: HashSet::new(),
            dependencies: HashSet::new(),
            term,
        }
    }
}

/// Generate the localization bindings.
#[proc_macro]
pub fn generate_bindings(_: TokenStream) -> TokenStream {
    let mut base_dir = base_path();
    base_dir.push("en-GB"); // todo: may not want to assume en-GB as default?

    let resources = load_resources(base_dir.clone()).unwrap_or_else(|_| {
        panic!("failed to load localization resources from folder: {base_dir:?}")
    });

    let mut nodes = (resources.iter())
        .flat_map(|resource| generate_nodes_for_resource(&resource.name, &resource.resource))
        .map(|node| (node.name.to_string(), node))
        .collect::<HashMap<_, _>>();

    let messages = (nodes.iter().filter(|(_, node)| !node.term))
        .map(|(name, _)| LitStr::new(name, Span::call_site()))
        .collect::<Vec<_>>();
    let message_count = messages.len();

    let terms = (nodes.iter().filter(|(_, node)| node.term))
        .map(|(name, _)| LitStr::new(name, Span::call_site()))
        .collect::<Vec<_>>();
    let term_count = terms.len();

    while let Some(dependency_name) =
        (nodes.iter()).find_map(|(_, node)| Some(node.dependencies.iter().next()?.to_string()))
    {
        let node = nodes
            .get(&*dependency_name).unwrap_or_else(|| panic!(
                "encountered a dependency on localization node `{dependency_name}` but no such node was loaded"
            ));
        let (variables, dependencies) = (node.variables.clone(), node.dependencies.clone());

        for (name, node) in
            (nodes.iter_mut()).filter(|(_, node)| node.dependencies.contains(&*dependency_name))
        {
            if name == &*dependency_name {
                panic!("cyclic localization loop detected at node {name}");
            }

            node.dependencies.remove(&*dependency_name);
            node.variables.extend(variables.iter());
            node.dependencies.extend(dependencies.iter());
        }
    }

    let mut stream = quote! {
        pub const MESSAGES: [&str; #message_count] = [#(#messages,)*];
        pub const TERMS: [&str; #term_count] = [#(#terms,)*];

        pub struct Localizer<'a> {
            localizations: &'a hartex_localization_loader::LocalizationBundleHolder,
            language: &'a str,
        }

        impl<'a> Localizer<'a> {
            pub fn new(holder: &'a hartex_localization_loader::LocalizationBundleHolder, language: &'a str) -> Localizer<'a> {
                Self {
                    localizations: holder,
                    language,
                }
            }

            pub fn validate_completeness_of_default_bundle() -> miette::Result<()> {
                let mut base_dir = hartex_localization_loader::env::base_path();
                base_dir.push("en-GB");

                let resources = hartex_localization_loader::load_resources(base_dir)?;

                let mut found_messages = std::collections::HashSet::<String>::new();
                let mut found_terms = std::collections::HashSet::<String>::new();

                resources.iter()
                    .flat_map(|resource| resource.resource.entries())
                    .for_each(|entry| {
                        match entry {
                            fluent_syntax::ast::Entry::Message(message) if message.value.is_some() => {
                                found_messages.insert(message.id.name.to_string());
                            }
                            fluent_syntax::ast::Entry::Term(term) => {
                                found_terms.insert(term.id.name.to_string());
                            }
                            _ => ()
                    }
                });

                let missing_messages = MESSAGES.into_iter().filter(|name| !found_messages.contains(&name.to_string())).collect::<Vec<_>>();
                let missing_terms = TERMS.into_iter().filter(|name| !found_terms.contains(&name.to_string())).collect::<Vec<_>>();

                if missing_messages.is_empty() && missing_terms.is_empty()  {
                    Ok(())
                } else {
                    Err(miette::Report::msg(format!("messages {} and terms {} are missing", missing_messages.join(","), missing_terms.join(","))))
                }
            }

            fn localize(&self, name: &str, arguments: Option<fluent_bundle::FluentArgs<'a>>) -> miette::Result<String> {
                let bundle = self.localizations.get_bundle(self.language);

                let message = bundle.get_message(name).unwrap();
                let mut errors = Vec::new();
                let localized = bundle.format_pattern(message.value().unwrap(), arguments.as_ref(), &mut errors);

                if errors.is_empty() {
                    return Ok(localized.to_string());
                }

                let errors = errors.iter().map(ToString::to_string).collect::<Vec<_>>();
                Err(miette::Report::msg(format!("errors found when localizing message: {}", errors.join(","))))
            }
        }
    };

    let no_generics_functions = (nodes.iter())
        .filter(|(_, node)| node.variables.is_empty() && !node.term)
        .map(|(name, node)| {
            let category = sanitize_name(node.category);
            let sanitized_name = sanitize_name(name);
            let ident = quote::format_ident!("{category}_{sanitized_name}");
            let name_lit = LitStr::new(name, Span::call_site());

            quote::quote! {
                #[inline]
                pub fn #ident(&self) -> miette::Result<String> {
                    self.localize(#name_lit, None)
                }
            }
        });
    let no_generics = quote::quote! {
        impl<'a> Localizer<'a> {
            #(#no_generics_functions)*
        }
    };
    stream.append_all(no_generics);

    let generics_functions = (nodes.iter())
        .filter(|(_, node)| !node.variables.is_empty() && !node.term)
        .map(|(name, node)| {
            let category = sanitize_name(node.category);
            let sanitized_name = sanitize_name(name);
            let ident = quote::format_ident!("{category}_{sanitized_name}");
            let name_lit = LitStr::new(name, Span::call_site());

            // todo: assumed that maximum 26 parameters are used
            let letters = ('A'..='Z').take(node.variables.len()).collect::<Vec<_>>();
            let generic_parameters = letters.iter().map(|letter| {
                GenericParam::Type(TypeParam::from(Ident::new(
                    &letter.to_string(),
                    Span::call_site(),
                )))
            });
            let generics = quote::quote! {
                <#(#generic_parameters),*>
            };

            let mut variables = node.variables.iter().collect::<Vec<_>>();
            variables.sort_unstable_by_key(|value| value.to_lowercase());

            let (extra_arguments, argument_insertion): (Vec<_>, Vec<_>) =
                (variables.iter().enumerate())
                    .map(|(index, name)| {
                        let sanitized_name = sanitize_name(name);
                        let ident = Ident::new(&sanitized_name, Span::call_site());
                        let corresponding_generic =
                            Ident::new(&letters[index].to_string(), Span::call_site());
                        let name_lit = LitStr::new(name, Span::call_site());

                        (
                            quote::quote! {
                                #ident: #corresponding_generic
                            },
                            quote::quote! {
                                arguments.set(#name_lit, #ident.into());
                            },
                        )
                    })
                    .unzip();
            let where_clauses = letters.iter().map(|letter| {
                let ident = Ident::new(&letter.to_string(), Span::call_site());

                quote::quote! {
                    #ident: Into<fluent_bundle::FluentValue<'a>>
                }
            });

            quote::quote! {
                #[inline]
                pub fn #ident #generics(&self, #(#extra_arguments),*) -> miette::Result<String>
                where #(#where_clauses),*
                {
                    let mut arguments = fluent_bundle::FluentArgs::new();
                    #(#argument_insertion)*

                    self.localize(#name_lit, Some(arguments))
                }
            }
        });
    let generics = quote::quote! {
        impl<'a> Localizer<'a> {
            #(#generics_functions)*
        }
    };
    stream.append_all(generics);

    stream.into()
}

/// Generate nodes for a given Fluent resources
fn generate_nodes_for_resource<'a>(
    parent: &'a str,
    resource: &'a Arc<FluentResource>,
) -> Vec<LocalizationNode<'a>> {
    let nodes = resource.entries().filter_map(|entry| {
        let (name, pattern, term) = match entry {
            Entry::Message(message) => (message.id.name, message.value.as_ref()?, false),
            Entry::Term(term) => (term.id.name, &term.value, true),
            _ => return None,
        };

        let mut node = LocalizationNode::new(parent, name, term);
        process_pattern_elements(&pattern.elements, &mut node);
        Some(node)
    });
    nodes.collect()
}

/// Process a Fluent expression.
fn process_expression<'a>(expression: &'a Expression<&'a str>, node: &mut LocalizationNode<'a>) {
    match expression {
        Expression::Inline(expression) => process_inline_expression(expression, node),
        Expression::Select { selector, variants } => {
            process_inline_expression(selector, node);
            (variants.iter()).for_each(|v| process_pattern_elements(&v.value.elements, node));
        }
    }
}

/// Process an inline Fluent expression.
fn process_inline_expression<'a>(
    expression: &'a InlineExpression<&'a str>,
    node: &mut LocalizationNode<'a>,
) {
    match expression {
        InlineExpression::FunctionReference { .. } => unimplemented!(),
        InlineExpression::MessageReference { id, .. } => {
            node.dependencies.insert(id.name);
        }
        InlineExpression::TermReference { id, .. } => {
            node.dependencies.insert(id.name);
        }
        InlineExpression::VariableReference { id } => {
            node.variables.insert(id.name);
        }
        InlineExpression::Placeable { expression } => process_expression(expression, node),
        InlineExpression::StringLiteral { .. } | InlineExpression::NumberLiteral { .. } => (),
    };
}

/// Process the pattern elements.
fn process_pattern_elements<'a>(
    elements: &'a Vec<PatternElement<&'a str>>,
    node: &mut LocalizationNode<'a>,
) {
    for element in elements {
        if let PatternElement::Placeable { expression } = element {
            process_expression(expression, node)
        }
    }
}

/// Sanitize Fluent names.
fn sanitize_name(unsanitized: &str) -> String {
    unsanitized.replace('-', "_").to_lowercase()
}
