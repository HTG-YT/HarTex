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

use rdkafka::ClientConfig;

use crate::types::CompressionType;

/// Extension functions for Kafka client configuration.
pub trait ClientConfigUtils {
    /// Configure bootstrap servers.
    fn bootstrap_servers(&mut self, servers: impl Iterator<Item = String>) -> &mut Self;

    /// Configure compression type.
    fn compression_type(&mut self, compression: CompressionType) -> &mut Self;

    /// Configure the duration for delivery timeout.
    fn delivery_timeout_ms(&mut self, timeout: u32) -> &mut Self;

    /// Configure group id.
    fn group_id(&mut self, group_id: &str) -> &mut Self;
}

impl ClientConfigUtils for ClientConfig {
    fn bootstrap_servers(&mut self, servers: impl Iterator<Item = String>) -> &mut Self {
        self.set(
            "bootstrap.servers",
            servers.intersperse(String::from(";")).collect::<String>(),
        )
    }

    fn compression_type(&mut self, compression: CompressionType) -> &mut Self {
        self.set("compression.type", compression)
    }

    fn delivery_timeout_ms(&mut self, timeout: u32) -> &mut Self {
        self.set("delivery.timeout.ms", timeout.to_string())
    }

    fn group_id(&mut self, group_id: &str) -> &mut Self {
        self.set("group.id", group_id)
    }
}
