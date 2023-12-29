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

//! # Cache Updaters
//!
//! Callbacks that are invoked when a certain event is received to update the cache accordingly.

#![allow(incomplete_features)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use hartex_discord_entitycache_core::error::CacheResult;

pub mod guild_create;
pub mod guild_member_chunk;

/// A trait for all cache updaters to implement.
pub trait CacheUpdater {
    /// Update the cache.
    #[allow(async_fn_in_trait)]
    async fn update(&self) -> CacheResult<()>;
}
