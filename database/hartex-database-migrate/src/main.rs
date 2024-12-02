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

//! # Database Migration Binary
//!
//! This utility is used for migrating to certain revisions of the PostgreSQL database
//! used by HarTex.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::env;

use miette::IntoDiagnostic;
use hartex_log::log;
use tokio_postgres::NoTls;

mod api_backend {
    refinery::embed_migrations!("api-backend-migrations");
}

mod discord_frontend {
    refinery::embed_migrations!("discord-frontend-migrations");
}

/// The entry point of the migration utility program.
#[tokio::main]
pub async fn main() -> miette::Result<()> {
    hartex_log::initialize();

    log::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    log::trace!("establishing database connection: Discord Frontend Migrations");
    let url = env::var("HARTEX_NIGHTLY_PGSQL_URL").unwrap();
    let (mut client, connection) =
        tokio_postgres::connect(&url, NoTls).await.into_diagnostic()?;

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            log::error!("postgres connection error: {error}");
        }
    });

    log::trace!("running migrations: Discord Frontend Migrations");
    discord_frontend::migrations::runner().run_async(&mut client).await.into_diagnostic()?;

    log::trace!("establishing database connection: API Backend Migrations");
    let url2 = env::var("API_PGSQL_URL").unwrap();
    let (mut client2, connection2) =
        tokio_postgres::connect(&url2, NoTls).await.into_diagnostic()?;

    tokio::spawn(async move {
        if let Err(error) = connection2.await {
            log::error!("postgres connection error: {error}");
        }
    });

    log::trace!("establishing database connection: API Backend Migrations");
    api_backend::migrations::runner().run_async(&mut client2).await.into_diagnostic()?;

    Ok(())
}
