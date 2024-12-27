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

//! # The Info User Subcommand
//!
//! This command returns informatiomn about a user.

use hartex_discord_cdn::Cdn;
use hartex_discord_core::discord::http::client::InteractionClient;
use hartex_discord_core::discord::mention::Mention;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::application::interaction::application_command::CommandDataOption;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::embed::ImageSource;
use hartex_discord_core::discord::util::snowflake::Snowflake;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_repositories::member::CachedMemberRepository;
use hartex_discord_entitycache_repositories::user::CachedUserRepository;
use hartex_discord_utils::commands::CommandDataOptionExt;
use hartex_discord_utils::commands::CommandDataOptionsExt;
use hartex_discord_utils::interaction::embed_response;
use hartex_discord_utils::markdown::MarkdownStyle;
use hartex_localization_core::Localizer;
use miette::IntoDiagnostic;
use rand::seq::IndexedRandom;
use rand::thread_rng;

/// Executes the `info user` command.
#[allow(clippy::too_many_lines)]
pub async fn execute(
    interaction: Interaction,
    interaction_client: &InteractionClient<'_>,
    option: CommandDataOption,
    localizer: Localizer<'_>,
) -> miette::Result<()> {
    let options = option.assume_subcommand();

    let user_id = options.user_value_of("user");

    let user = CachedUserRepository.get(user_id).await.into_diagnostic()?;

    let userinfo_embed_generalinfo_field_name =
        localizer.utilities_plugin_userinfo_embed_generalinfo_field_name()?;
    let userinfo_embed_generalinfo_id_subfield_name =
        localizer.utilities_plugin_userinfo_embed_generalinfo_id_subfield_name()?;
    let userinfo_embed_generalinfo_name_subfield_name =
        localizer.utilities_plugin_userinfo_embed_generalinfo_name_subfield_name()?;
    let userinfo_embed_generalinfo_created_subfield_name =
        localizer.utilities_plugin_userinfo_embed_generalinfo_created_subfield_name()?;
    let userinfo_embed_serverpresence_field_name =
        localizer.utilities_plugin_userinfo_embed_serverpresence_field_name()?;
    let userinfo_embed_serverpresence_nickname_subfield_name =
        localizer.utilities_plugin_userinfo_embed_serverpresence_nickname_subfield_name()?;
    let userinfo_embed_serverpresence_joined_subfield_name =
        localizer.utilities_plugin_userinfo_embed_serverpresence_joinedat_subfield_name()?;
    let userinfo_embed_serverpresence_roles_subfield_name =
        localizer.utilities_plugin_userinfo_embed_serverpresence_roles_subfield_name()?;
    let userinfo_embed_serverpresence_flags_subfield_name =
        localizer.utilities_plugin_userinfo_embed_serverpresence_flags_subfield_name()?;

    let mut builder = EmbedBuilder::new()
        .color(0x41_A0_DE)
        .field(EmbedFieldBuilder::new(
            userinfo_embed_generalinfo_field_name,
            format!(
                "{} {}\n{} {}\n{} {}",
                userinfo_embed_generalinfo_id_subfield_name,
                user_id.to_string().discord_inline_code(),
                userinfo_embed_generalinfo_name_subfield_name,
                user.global_name
                    .clone()
                    .unwrap_or(String::from("<not set>")),
                userinfo_embed_generalinfo_created_subfield_name,
                (user.id.timestamp() / 1000)
                    .to_string()
                    .discord_relative_timestamp(),
            ),
        ));

    if let Some(guild_id) = interaction.guild_id {
        let member = CachedMemberRepository
            .get((guild_id, user_id))
            .await
            .into_diagnostic()?;

        let flags = member
            .flags
            .iter_names()
            .map(|(name, _)| name)
            .collect::<Vec<_>>();
        let flags_display = if flags.is_empty() {
            "None".to_string()
        } else {
            flags.join(", ")
        };

        builder = builder
            .field(EmbedFieldBuilder::new(
                userinfo_embed_serverpresence_field_name,
                format!(
                    "{} {}\n{} {}\n{} {}\n{} {}",
                    userinfo_embed_serverpresence_nickname_subfield_name,
                    member.nick.unwrap_or(String::from("<not set>")),
                    userinfo_embed_serverpresence_joined_subfield_name,
                    member
                        .joined_at
                        .map_or(String::from("unknown"), |timestamp| timestamp
                            .as_secs()
                            .to_string()
                            .discord_relative_timestamp()),
                    userinfo_embed_serverpresence_roles_subfield_name,
                    member
                        .roles
                        .choose_multiple(&mut thread_rng(), 10)
                        .map(|id| id.mention().to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    userinfo_embed_serverpresence_flags_subfield_name,
                    flags_display,
                ),
            ))
            .title(user.name);
    }

    builder = if let Some(avatar) = user.avatar {
        builder.thumbnail(ImageSource::url(Cdn::user_avatar(user_id, avatar)).into_diagnostic()?)
    } else if user.global_name.is_some() {
        builder.thumbnail(
            ImageSource::url(Cdn::default_user_avatar(Some(user_id), None)).into_diagnostic()?,
        )
    } else {
        builder.thumbnail(
            ImageSource::url(Cdn::default_user_avatar(None, Some(user.discriminator)))
                .into_diagnostic()?,
        )
    };

    let embed = builder.validate().into_diagnostic()?.build();

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
