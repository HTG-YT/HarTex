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

//! # Backend Driver
//!
//! This is the driver crate of the API backend for HarTex, uniting all components of the backend
//! and contains the core routing logic that routes requests to the corresponding request handlers.
//!
//! The driver also registers certain useful error catchers that return custom JSON payloads.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use hartex_backend_routes_v1::bors::v1_repositories_repository_permissions_permissions;
use hartex_backend_routes_v1::uptime::v1_post_uptime;
use hartex_log::log;
use rocket::catchers;
use rocket::routes;

mod catchers;

/// # Entry Point
///
/// This is the entry point of the API backend for HarTex. This does the heavy lifting of building
/// a Rocket server, igniting, and launching it.
#[rocket::main]
pub async fn main() -> hartex_eyre::Result<()> {
    hartex_eyre::initialize()?;
    hartex_log::initialize();

    log::trace!("loading environment variables");
    dotenvy::dotenv()?;

    log::debug!("igniting rocket");
    let rocket = rocket::build()
        .mount(
            "/api/v1",
            routes![
                v1_post_uptime,
                v1_repositories_repository_permissions_permissions,
            ],
        )
        .register(
            "/",
            catchers![catchers::not_found, catchers::too_many_requests],
        )
        .ignite()
        .await?;

    log::debug!("launching rocket");
    rocket.launch().await?;

    Ok(())
}
