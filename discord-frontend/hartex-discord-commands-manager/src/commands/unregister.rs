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

use std::env;

use clap::ArgMatches;
use hartex_discord_core::dotenvy;
use hartex_discord_core::tokio::task::spawn;
use hartex_discord_utils::hyper::tls_stream;
use hartex_log::log;
use http_body_util::Empty;
use hyper::Method;
use hyper::Request;
use hyper::body::Bytes;
use hyper::client::conn::http1::handshake;
use hyper::header::ACCEPT;
use hyper::header::AUTHORIZATION;
use hyper::header::CONTENT_LENGTH;
use hyper::header::HOST;
use hyper::header::USER_AGENT;
use hyper_util::rt::TokioIo;
use miette::IntoDiagnostic;

/// Unregistering a command.
#[allow(clippy::module_name_repetitions)]
pub async fn unregister_command(matches: ArgMatches) -> miette::Result<()> {
    log::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    let command_id = matches.get_one::<String>("command-id").unwrap().clone();

    log::trace!("making tcp connection");
    let stream = tls_stream().await?;
    let (mut sender, connection) = handshake(TokioIo::new(stream)).await.into_diagnostic()?;

    spawn(async move {
        if let Err(err) = connection.await {
            log::error!("TCP connection failed: {:?}", err);
        }
    });

    let application_id = env::var("APPLICATION_ID").into_diagnostic()?;

    let mut token = env::var("BOT_TOKEN").into_diagnostic()?;
    if !token.starts_with("Bot ") {
        token.insert_str(0, "Bot ");
    }

    log::trace!("sending request");
    let request = Request::builder()
        .uri(format!(
            "/api/v10/applications/{application_id}/commands/{command_id}"
        ))
        .method(Method::DELETE)
        .header(HOST, "discord.com")
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, token)
        .header(CONTENT_LENGTH, 0)
        .header(
            USER_AGENT,
            "DiscordBot (https://github.com/TeamHarTex/HarTex, v0.6.0) CommandsManager",
        )
        .body(Empty::<Bytes>::new())
        .into_diagnostic()?;

    let result = sender.send_request(request).await.into_diagnostic()?;
    log::info!("received response with status {}", result.status());

    Ok(())
}
