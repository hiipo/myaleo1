// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::Record;
use snarkvm_dpc::{prelude::*, testnet2::Payload};

use anyhow::Result;

pub struct VirtualRecord {
    pub(crate) owner: Option<String>,
    pub(crate) value: Option<u64>,
    pub(crate) payload: Option<Payload>,
    pub(crate) birth_program_id: Option<Vec<u8>>,
    pub(crate) death_program_id: Option<Vec<u8>>,
    pub(crate) commitment: Option<[u8; 32]>,
    pub(crate) commitment_randomness: Option<[u8; 32]>,
    pub(crate) serial_number_nonce: Option<[u8; 32]>,
    pub(crate) serial_number_nonce_randomness: Option<[u8; 32]>,
}

impl Record for VirtualRecord {
    fn is_dummy(&self) -> bool {
        // Check that the record value is zero or unset.
        match self.value {
            Some(0u64) | None => (),
            Some(_value) => false,
        }

        // // Check that the record payload is empty or unset.
        // match self.payload {
        //     Some() | None => (),
        //     Some(_value) => false,
        // }

        true
    }
}
