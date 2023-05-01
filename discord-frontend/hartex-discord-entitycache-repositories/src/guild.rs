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

//! # Guild Repository

use std::env;

use hartex_discord_core::discord::model::id::Id;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::guild::GuildEntity;
use redis::AsyncCommands;
use redis::Client;

/// Repository for guild entities.
pub struct CachedGuildRepository;

impl Repository<GuildEntity> for CachedGuildRepository {
    async fn get(&self, id: <GuildEntity as Entity>::Id) -> CacheResult<GuildEntity> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;

        let id = connection
            .get::<String, u64>(format!("guild:{id}:id"))
            .await?;

        Ok(GuildEntity {
            id: Id::new_checked(id).expect("id is zero (unexpected and unreachable)"),
        })
    }

    #[allow(clippy::unused_async)]
    async fn upsert(&self, entity: GuildEntity) -> CacheResult<()> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;

        connection
            .set(format!("guild:{}:id", entity.id), entity.id.get())
            .await?;

        Ok(())
    }
}
