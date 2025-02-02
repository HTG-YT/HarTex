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

use std::process::Command;
use std::process::exit;
use owo_colors::OwoColorize;
use crate::builder::Builder;
use crate::builder::RunConfig;
use crate::builder::Step;

/// Step for compiling the test suite.
pub struct BuildTestsuiteTool;

impl Step for BuildTestsuiteTool {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        build_testsuite_tool(builder);
    }

    fn run_config(run: RunConfig<'_>) {
        run.builder.run_step(BuildTestsuiteTool);
    }
}

/// Utility function for building the test suite tool.
#[allow(clippy::missing_panics_doc)]
fn build_testsuite_tool(builder: &Builder<'_>) {
    let pwd = builder.config.root.join("tools/testsuite");

    let mut command = Command::new("cargo");
    command.arg("build");

    let mut rustflags = format!("-C opt-level={}", builder.config.opt_level);

    if builder.config.debug {
        rustflags.push_str(" -g");
    }

    command.current_dir(pwd);
    command.env(
        "CARGO_TARGET_DIR",
        builder
            .config
            .root
            .join(builder.config.output_dir.clone())
            .join("testsuite"),
    );
    command.env("RUSTFLAGS", rustflags);

    println!("{} Building testsuite tool before running tests", "info:".bold());
    let status = command.status().expect("failed to get status");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

/// Step for running the test suite.
pub struct RunUiTests;

impl Step for RunUiTests {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        run_ui_tests(builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("--ui"))
        {
            run.builder.run_step(RunUiTests);
        }
    }
}

/// Utility function for running the test suite.
#[allow(clippy::missing_panics_doc)]
fn run_ui_tests(builder: &Builder<'_>) {
    let mut command = Command::new("./build/testsuite/debug/testsuite");
    command.args(&builder.config.subcommand_args);
    command.current_dir(&builder.config.root);

    println!("{} Running testsuite tool", "info:".bold());
    let status = command.status().expect("failed to get status");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}
