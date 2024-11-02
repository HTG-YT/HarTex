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

//! # Backend Driver
//!
//! This is the driver crate of the API backend for HarTex, uniting all components of the backend
//! and contains the core routing logic that routes requests to the corresponding request handlers.
//!
//! The driver also registers certain useful error catchers that return custom JSON payloads.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::env;
#[cfg(not(unix))]
use std::future;
use std::time::Duration;

use bb8_postgres::bb8::Pool;
use bb8_postgres::tokio_postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;
use dotenvy::Error;
use hartex_errors::dotenv;
use hartex_log::log;
use miette::IntoDiagnostic;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_redoc::Redoc;
use utoipa_redoc::Servable;

/// # Entry Point
///
/// This is the entry point of the API backend for HarTex. This does the heavy lifting of building
/// an Axum server and starting it.
#[allow(clippy::ignored_unit_patterns)]
#[allow(clippy::no_effect_underscore_binding)]
#[tokio::main]
pub async fn main() -> miette::Result<()> {
    hartex_log::initialize();

    log::trace!("loading environment variables");
    if let Err(error) = dotenvy::dotenv() {
        match error {
            Error::LineParse(content, index) => Err(dotenv::LineParseError {
                src: content,
                err_span: (index - 1, 1).into(),
            })?,
            _ => todo!(),
        }
    }

    let api_pgsql_url = env::var("API_PGSQL_URL").into_diagnostic()?;

    log::debug!("building database connection pool");
    let manager = PostgresConnectionManager::new_from_stringlike(api_pgsql_url, NoTls).into_diagnostic()?;
    let pool = Pool::builder().build(manager).await.into_diagnostic()?;

    log::debug!("starting axum server");
    let (app, openapi) = OpenApiRouter::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .routes(routes!(
            hartex_backend_routes::uptime::get_uptime
        ))
        .with_state(pool)
        .split_for_parts();

    let router = app.merge(Redoc::with_url("/openapi", openapi));

    let domain = env::var("API_DOMAIN").into_diagnostic()?;
    let listener = TcpListener::bind(&domain).await.into_diagnostic()?;
    log::debug!("listening on {domain}");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown())
        .await
        .into_diagnostic()?;

    Ok(())
}

/// Creates a shutdown signal future for the Axum server to wait for in graceful shutdown.
///
/// This listens for both CTRL+C and SIGTERM (Unix-specific).
#[allow(clippy::ignored_unit_patterns)]
async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
