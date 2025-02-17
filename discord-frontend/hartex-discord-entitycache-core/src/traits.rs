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

//! # Entity Cache Traits

use crate::error::CacheResult;

/// A cache entity.
pub trait Entity {
    /// The identifier of the entity.
    type Id;

    /// Returns the entity identifier.
    fn id(&self) -> Self::Id;
}

/// A cache repository holding entities.
pub trait Repository<T: Entity> {
    /// Retrieves an entity from the repository.
    #[allow(async_fn_in_trait)]
    async fn get(&self, entity_id: T::Id) -> CacheResult<T>;

    /// Upserts an entity into the repository.
    #[allow(async_fn_in_trait)]
    async fn upsert(&self, entity: T) -> CacheResult<()>;
}
