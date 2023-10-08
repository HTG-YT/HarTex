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

use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use owo_colors::OwoColorize;
use serde::Deserialize;
use serde::Serialize;

/// Command option choice,
///
/// Refer to the corresponding API documentation on discord official website.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize)]
pub struct CommandManagerCommandOptionChoice {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: CommandManagerCommandOptionChoiceValue,
}

impl CommandManagerCommandOptionChoice {
    /// Display command option choice.
    pub fn display(&self, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
        writeln!(
            f,
            "{}- {}{}",
            "    ".repeat(depth),
            "Command Option Choice Name: ".bold(),
            self.name.bright_cyan()
        )?;

        write!(
            f,
            "{}  {}",
            "    ".repeat(depth),
            "Command Option Choice Name Localizations: ".bold()
        )?;
        if self.name_localizations.is_some() {
            super::super::print_localizations(
                f,
                self.name_localizations.as_ref().unwrap(),
                depth + 1,
            )?;
        } else {
            writeln!(f, "{}", "None".truecolor(107, 107, 107))?;
        }

        self.value.display(f, depth)
    }
}

/// Command option choice value.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CommandManagerCommandOptionChoiceValue {
    String(String),
    Integer(i64),
    Number(f64),
}

impl CommandManagerCommandOptionChoiceValue {
    /// Display command option choice value.
    pub fn display(&self, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
        write!(
            f,
            "{}  {}",
            "    ".repeat(depth),
            "Command Option Choice Value: ".bold()
        )?;

        match self {
            Self::String(string) => writeln!(f, "{}", string.bright_cyan()),
            Self::Integer(integer) => writeln!(f, "{}", integer.bright_cyan()),
            Self::Number(number) => writeln!(f, "{}", number.to_string().bright_cyan()),
        }
    }
}
