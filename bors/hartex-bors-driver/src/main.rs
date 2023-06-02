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

//! # Bors for HarTex
//!
//! A reimplementation of Bors in Rust for usage in the HarTex repository.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use hartex_bors_database::client::SeaORMDatabaseClient;
use hartex_bors_database::migration::Migrator;
use hartex_bors_github::GithubBorsState;
use hartex_log::log;
use sea_orm::DatabaseConnection;
use sea_orm::SqlxSqliteConnector;
use sea_orm_migration::prelude::MigratorTrait;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use tokio::runtime::Builder;

mod event;
mod process;
mod workflows;

/// Entry point.
pub fn main() -> hartex_eyre::Result<()> {
    hartex_eyre::initialize()?;
    hartex_log::initialize();

    actual_main()?;

    Ok(())
}

/// Actual entry point, building the runtime and other stuff.
fn actual_main() -> hartex_eyre::Result<()> {
    dotenvy::dotenv()?;

    log::trace!("constructing runtime");
    let runtime = Builder::new_multi_thread().enable_all().build()?;

    log::trace!("loading github application state");
    let app_id = env::var("APP_ID")?.parse::<u64>()?;

    log::trace!("initializing sqlite database");
    let database = runtime.block_on(initialize_database())?;

    let mut private_key_file = File::open("../bors-private-key.pem")?;
    let mut private_key = String::new();
    private_key_file.read_to_string(&mut private_key)?;

    let state = runtime.block_on(GithubBorsState::load(
        app_id.into(),
        SeaORMDatabaseClient::new(database),
        private_key.into_bytes().into(),
    ))?;

    let future = process::bors_process(state);

    runtime.block_on(future);

    Ok(())
}

async fn initialize_database() -> hartex_eyre::Result<DatabaseConnection> {
    let database = SqlxSqliteConnector::from_sqlx_sqlite_pool(
        SqlitePool::connect_with(
            SqliteConnectOptions::from_str("sqlite:bors-data/data.db")?.create_if_missing(true),
        )
        .await?,
    );
    Migrator::up(&database, None).await?;

    Ok(database)
}
