// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use crate::errors::message::{MessageHeaderError, StreamReadError};

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("{}", _0)]
    Message(String),

    #[error("Invalid message length {}. Expected length of {}", _0, _1)]
    InvalidLength(usize, usize),

    #[error("{}", _0)]
    MessageHeaderError(MessageHeaderError),

    #[error("{}", 0)]
    SteamReadError(StreamReadError),
}

impl From<MessageHeaderError> for MessageError {
    fn from(error: MessageHeaderError) -> Self {
        MessageError::MessageHeaderError(error)
    }
}

impl From<StreamReadError> for MessageError {
    fn from(error: StreamReadError) -> Self {
        MessageError::SteamReadError(error)
    }
}

impl From<bincode::Error> for MessageError {
    fn from(error: bincode::Error) -> Self {
        MessageError::Crate("bincode", format!("{:?}", error))
    }
}

impl From<std::io::Error> for MessageError {
    fn from(error: std::io::Error) -> Self {
        MessageError::Crate("std::io", format!("{:?}", error))
    }
}
