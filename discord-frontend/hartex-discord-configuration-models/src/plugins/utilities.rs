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

//! # Utilities Plugin Configuration Object

use mlua::Error;
use mlua::FromLua;
use mlua::Lua;
use mlua::Value;
use serde::Serialize;

/// The utilities plugin configuration object.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize)]
pub struct UtilitiesPlugin {
    /// Sets whether the utilities plugin is enabled.
    pub enabled: bool,
}

impl FromLua for UtilitiesPlugin {
    fn from_lua(lua_value: Value, _: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = lua_value.clone() else {
            return Err(Error::RuntimeError(format!(
                "UtilitiesPlugin: mismatched value type, expected table, found: {}",
                lua_value.type_name()
            )));
        };

        let enabled = table.get("enabled")?;

        Ok(Self { enabled })
    }
}
