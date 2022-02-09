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

use std::env::VarError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

use http::Error as HttpError;
use hyper::Error as HyperError;
use serde_json::Error as JsonError;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("hartex error: ")?;

        match &self.kind {
            ErrorKind::EnvVarError { src } => write!(f, "env error: {src}")?,
            ErrorKind::HttpError { src } => write!(f, "http error: {src}")?,
            ErrorKind::HyperError { src } => write!(f, "hyper error: {src}")?,
            ErrorKind::IoError { src } => write!(f, "io error: {src}")?,
            ErrorKind::JsonError { src } => write!(f, "json error: {src}")?,
            ErrorKind::PortNotNumber { name } => {
                write!(f, "port error: specified port not a number for port {name}")?
            }
        }

        Ok(())
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl From<IoError> for Error {
    fn from(src: IoError) -> Self {
        Self::from(ErrorKind::IoError { src })
    }
}

impl From<VarError> for Error {
    fn from(src: VarError) -> Self {
        Self::from(ErrorKind::EnvVarError { src })
    }
}

impl StdError for Error {}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    EnvVarError { src: VarError },
    HttpError { src: HttpError },
    HyperError { src: HyperError },
    IoError { src: IoError },
    JsonError { src: JsonError },
    PortNotNumber { name: String },
}

pub type Result<T> = StdResult<T, Error>;
