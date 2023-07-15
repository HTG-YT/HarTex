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

//! # Project Models

use std::env;
use std::process::Command;

use miette::IntoDiagnostic;
use miette::Report;
use serde::Deserialize;

/// A project.
#[derive(Debug, Deserialize)]
#[serde(rename = "kebab-case")]
pub struct Project {
    /// The type of the project.
    pub r#type: ProjectType,
    /// The build profile for the project.
    pub profile: Option<RustBuildProfile>,
    /// Whether to include debug information when building the project.
    pub include_debug_info: Option<bool>,
}

impl Project {
    /// Build a project with its name.
    pub fn build(&self, name: String) -> miette::Result<()> {
        let mut pwd = env::current_dir().into_diagnostic()?;
        pwd.push(name);

        let result = match self.r#type {
            ProjectType::JsTs => {
                let mut command = Command::new("pwsh");
                command.current_dir(pwd).arg("-c").arg("yarn build");
                command.status().into_diagnostic()?.exit_ok()
            }
            ProjectType::Rust => {
                let mut command = Command::new("cargo");
                command.arg("build").current_dir(pwd);

                if let Some(profile) = self.profile.clone() && profile == RustBuildProfile::Release {
                    command.arg("--release");
                }

                if let Some(include_debug_info) = self.include_debug_info && include_debug_info {
                    command.env("RUSTFLAGS", "-g");
                }

                command.status().into_diagnostic()?.exit_ok()
            }
        };

        result.map_err(|error| Report::msg(format!("abnormal termination: {error}")))
    }

    /// Runs linting on a project with its name.
    pub fn lint(&self, name: String) -> miette::Result<()> {
        let mut pwd = env::current_dir().into_diagnostic()?;
        pwd.push(name);

        let result = match self.r#type {
            ProjectType::JsTs => {
                let mut command = Command::new("pwsh");
                command.current_dir(pwd).arg("-c").arg("yarn eslint");
                command.status().into_diagnostic()?.exit_ok()
            }
            ProjectType::Rust => {
                let mut command = Command::new("cargo");
                command.arg("clippy").current_dir(pwd.clone());
                command.status().into_diagnostic()?.exit_ok().into_diagnostic()?;

                let mut command = Command::new("cargo");
                command.arg("fmt").current_dir(pwd);
                command.status().into_diagnostic()?.exit_ok()
            }
        };

        result.map_err(|error| Report::msg(format!("abnormal termination: {error}")))
    }

    /// Runs a test suite on a project with its name.
    pub fn test(&self, name: String) -> miette::Result<()> {
        let mut pwd = env::current_dir().into_diagnostic()?;
        pwd.push(name);

        let result = match self.r#type {
            ProjectType::JsTs => {
                return Err(Report::msg("testing currently not supported for jsts projects"));
            }
            ProjectType::Rust => {
                let mut command = Command::new("cargo");
                command.arg("nextest").arg("run").current_dir(pwd.clone());
                command.status().into_diagnostic()?.exit_ok()
            }
        };

        result.map_err(|error| Report::msg(format!("abnormal termination: {error}")))
    }
}

/// The project type.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    /// JavaScript / TypeScript project.
    JsTs,
    /// Rust project.
    Rust,
}

/// The Rust build profile for a Rust project.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RustBuildProfile {
    /// Release profile.
    Release,
}
