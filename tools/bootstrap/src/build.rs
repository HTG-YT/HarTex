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

use crate::builder::Builder;
use crate::config::Config;

/// Represents a build session.
pub struct Build {
    /// The configuration used for the session.
    pub config: Config,
}

impl Build {
    /// Construct a new build session.
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Run the build.
    pub fn build(&self) {
        Builder::new(self).run_cli();
    }
}
