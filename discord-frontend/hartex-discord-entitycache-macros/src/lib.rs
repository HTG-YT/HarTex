/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

//! # Entity Cache Macros
//!
//! Useful macros for the entity cache.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(deprecated)]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

mod entity;

/// Macro to derive the `Entity` trait.
#[deprecated(since = "0.4.0")]
#[proc_macro_derive(Entity, attributes(entity))]
pub fn derive_entity_trait(tokens: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(tokens as DeriveInput);
    entity::expand_entity_derivation(&mut input)
        .unwrap_or_default()
        .into()
}
