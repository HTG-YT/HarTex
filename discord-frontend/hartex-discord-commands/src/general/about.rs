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

//! # The About Command
//!
//! This command returns brief information about the bot, like its description and GitHub
//! repository.

use async_trait::async_trait;
use hartex_discord_commands_core::command;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_core::discord::http::client::InteractionClient;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::util::builder::embed::EmbedAuthorBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFooterBuilder;
use hartex_discord_core::discord::util::builder::embed::ImageSource;
use hartex_discord_utils::interaction::embed_response;
use hartex_localization_core::Localizer;
use miette::IntoDiagnostic;

use crate::general::General;

/// The `about` command declaration.
#[command(name = "about", plugin = General)]
pub struct About;

#[async_trait]
impl Command for About {
    async fn execute(
        &self,
        interaction: Interaction,
        interaction_client: &InteractionClient<'_>,
        localizer: Localizer<'_>,
    ) -> miette::Result<()> {
        let about_embed_title = localizer.general_plugin_about_embed_title()?;
        let about_embed_description = localizer.general_plugin_about_embed_description()?;
        let about_embed_github_repo_field_name =
            localizer.general_plugin_about_embed_github_repo_field_name()?;
        let about_embed_footer =
            localizer.general_plugin_about_embed_footer("https://discord.gg/Xu8453VBAv")?;
        let embed = EmbedBuilder::new()
            .author(
                EmbedAuthorBuilder::new(about_embed_title)
                    .icon_url(ImageSource::url("https://cdn.discordapp.com/avatars/936431574310879332/9a46b39c031ca84e8351ee97867afc96.png").into_diagnostic()?)
                    .build()
            )
            .color(0x41_A0_DE)
            .description(about_embed_description)
            .field(EmbedFieldBuilder::new(about_embed_github_repo_field_name, "https://github.com/TeamHarTex/HarTex").build())
            .footer(EmbedFooterBuilder::new(about_embed_footer).build())
            .validate()
            .into_diagnostic()?
            .build();

        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &embed_response(vec![embed]),
            )
            .await
            .into_diagnostic()?;

        Ok(())
    }
}
