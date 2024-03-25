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

#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub generic_params: Vec<GenericParameter>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub vis: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct GenericParameter {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub generic_params: Vec<GenericParameter>,
    pub fields: Vec<Field>,
}
