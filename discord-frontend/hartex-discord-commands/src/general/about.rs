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

//! # The About Command

use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::CommandMetadata;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedAuthorBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFooterBuilder;
use hartex_discord_core::discord::util::builder::embed::ImageSource;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::create_bundle;
use hartex_localization_core::handle_errors;
use hartex_localization_macros::bundle_get;
use hartex_localization_macros::bundle_get_args;

#[derive(CommandMetadata)]
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(minimum_level = 0)]
#[metadata(name = "about")]
pub struct About;

impl Command for About {
    #[allow(clippy::unused_async)]
    async fn execute(&self, interaction: Interaction) -> hartex_eyre::Result<()> {
        let interaction_client = CLIENT.interaction(interaction.application_id);
        let bundle = create_bundle(
            interaction.locale.and_then(|locale| locale.parse().ok()),
            &["discord-frontend", "commands"],
        )?;

        bundle_get!(bundle."about-embed-description": message, out [about_embed_description, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."about-embed-github-repo-field-name": message, out [about_embed_github_repo_field_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."about-embed-title": message, out [about_embed_title, errors]);
        handle_errors(errors)?;

        let invite_link = "https://discord.gg/Xu8453VBAv";
        bundle_get_args!(bundle."about-embed-footer": message, out [about_embed_footer, errors], args ["inviteLink" to invite_link]);

        let embed = EmbedBuilder::new()
            .author(
                EmbedAuthorBuilder::new(about_embed_title)
                    .icon_url(ImageSource::url("https://cdn.discordapp.com/avatars/936431574310879332/9a46b39c031ca84e8351ee97867afc96.png")?)
                    .build()
            )
            .color(0x41_A0_DE)
            .description(about_embed_description)
            .field(EmbedFieldBuilder::new(about_embed_github_repo_field_name, "https://github.com/TeamHarTex/HarTex").build())
            .footer(EmbedFooterBuilder::new(about_embed_footer).build())
            .validate()?
            .build();

        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .embeds(vec![embed])
                            .build(),
                    ),
                },
            )
            .await?;

        Ok(())
    }
}
