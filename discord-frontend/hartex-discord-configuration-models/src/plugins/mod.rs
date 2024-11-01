/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

//! # Plugins Configuration Object

use mlua::Error;
use mlua::FromLua;
use mlua::Lua;
use mlua::Value;
use serde::Serialize;

pub mod management;
pub mod modlog;
pub mod utilities;

/// The plugins configuration object.
#[derive(Debug, Serialize)]
pub struct Plugins {
    /// Optional configuration object for the management plugin.
    pub management: Option<management::ManagementPlugin>,
    /// Optional configuration object for the modlog plugin.
    pub modlog: Option<modlog::ModlogPlugin>,
    /// Optional configuration object for the utilities plugin.
    pub utilities: Option<utilities::UtilitiesPlugin>,
}

impl FromLua for Plugins {
    fn from_lua(lua_value: Value, _: &Lua) -> mlua::Result<Self> {
        let Value::Table(table) = lua_value.clone() else {
            return Err(Error::RuntimeError(format!(
                "Dashboard: mismatched value type, expected table, found: {}",
                lua_value.type_name()
            )));
        };

        let management = table.get("management")?;
        let modlog = table.get("modlog")?;
        let utilities = table.get("utilities")?;

        Ok(Self {
            management,
            modlog,
            utilities,
        })
    }
}
