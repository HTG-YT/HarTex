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

//! # Localization Infrastructure Core
//!
//! The localization-core crate provides core infrastructure for the implementation of
//! localization.

#![allow(incomplete_features)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(let_chains)]

use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use fluent_bundle::FluentError;
use fluent_bundle::FluentResource;
use miette::IntoDiagnostic;
use miette::Report;
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

pub mod types;

/// Create a localization bundle from its path.
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub fn create_bundle(
    requested: Option<LanguageIdentifier>,
    path: &[&str],
) -> miette::Result<types::LocalizationBundle> {
    let fallback = langid!("en-GB");
    let locale = requested.unwrap_or(fallback.clone());
    let mut bundle = types::LocalizationBundle::new_concurrent(vec![locale.clone()]);
    // fix Discord timestamp formatting
    bundle.set_use_isolating(false);

    let mut localizations_root = PathBuf::from("../localization/locales");
    localizations_root.push(locale.to_string());

    if !localizations_root.exists() {
        bundle.locales = vec![fallback];
        localizations_root = PathBuf::from("../localization/locales/en-GB");
    }

    path.iter()
        .for_each(|segment| localizations_root.push(segment));

    if !localizations_root.is_dir() {
        return Err(Report::msg(format!(
            "localization root is not a directory: {}",
            localizations_root.to_string_lossy()
        )));
    }

    for result in localizations_root.read_dir().into_diagnostic()? {
        let entry = result.into_diagnostic()?;
        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) != Some("ftl") {
            continue;
        }

        let resource_string = fs::read_to_string(path).into_diagnostic()?;
        let resource = FluentResource::try_new(resource_string)
            .map_err(|(_, errors)| errors.last().unwrap().clone())
            .into_diagnostic()?;
        bundle
            .add_resource(resource)
            .map_err(|errors| errors.last().unwrap().clone())
            .into_diagnostic()?;
    }

    Ok(bundle)
}

/// Handle errors returned from fluent.
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::needless_pass_by_value)]
pub fn handle_errors(errors: Vec<FluentError>) -> miette::Result<()> {
    if errors.is_empty() {
        return Ok(());
    }

    Err(errors[0].clone()).into_diagnostic()
}
