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

//! # Bors GitHub API Interaction

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use hartex_bors_core::models::GithubRepositoryName;
use hartex_bors_core::BorsState;
use hartex_bors_core::RepositoryClient;
use hartex_log::log;
use jsonwebtoken::EncodingKey;
use octocrab::models::App;
use octocrab::models::AppId;
use octocrab::models::Repository;
use octocrab::Octocrab;
use secrecy::ExposeSecret;
use secrecy::SecretVec;

/// State of the bors Github application
#[allow(dead_code)]
pub struct GithubBorsState {
    application: App,
    client: Octocrab,
}

impl GithubBorsState {
    /// Load the Github application state for bors.
    pub async fn load(
        application_id: AppId,
        private_key: SecretVec<u8>,
    ) -> hartex_eyre::Result<Self> {
        log::trace!("obtaining private key");
        let key = EncodingKey::from_rsa_pem(private_key.expose_secret().as_ref())?;

        log::trace!("building github client");
        let client = Octocrab::builder().app(application_id, key).build()?;

        log::trace!("obtaining github application");
        let application = client.current().app().await?;

        Ok(Self {
            application,
            client,
        })
    }
}

impl BorsState<GithubRepositoryClient> for GithubBorsState {}

/// A Github repository client.
pub struct GithubRepositoryClient {
    /// Octocrab client.
    client: Octocrab,
    /// The name of the repository.
    repository_name: GithubRepositoryName,
    /// The repository.
    repository: Repository,
}

impl GithubRepositoryClient {
    /// Returns a reference to the Octocrab client.
    pub fn client(&self) -> &Octocrab {
        &self.client
    }

    /// Returns a reference to the repository.
    pub fn repository(&self) -> &Repository {
        &self.repository
    }
}

impl RepositoryClient for GithubRepositoryClient {
    fn repository_name(&self) -> &GithubRepositoryName {
        &self.repository_name
    }

    async fn post_comment(&mut self, pr: u64, text: &str) -> hartex_eyre::Result<()> {
        self.client
            .issues(self.repository_name.owner(), self.repository_name.repository())
            .create_comment(pr, text)
            .await?;

        Ok(())
    }
}
