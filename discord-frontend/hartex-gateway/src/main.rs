/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use hartex_core::dotenv;
use hartex_core::log;
use hartex_core::tokio;

mod error;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> hartex_eyre::eyre::Result<()> {
    hartex_eyre::initialize()?;
    log::initialize();

    log::trace!("loading environment variables");
    dotenv::dotenv()?;

    Ok(())
}
