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

use crate::{ir, Instruction};

use anyhow::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Entrypoint {
    pub argument_start_variable: u32,
    pub instructions: Vec<Instruction>,
}

impl Entrypoint {
    pub(crate) fn decode(entrypoint: ir::Entrypoint) -> Result<Self> {
        Ok(Self {
            argument_start_variable: entrypoint.argument_start_variable,
            instructions: entrypoint
                .instructions
                .into_iter()
                .map(Instruction::decode)
                .collect::<Result<Vec<_>>>()?,
        })
    }

    pub(crate) fn encode(&self) -> ir::Entrypoint {
        ir::Entrypoint {
            argument_start_variable: self.argument_start_variable,
            instructions: self.instructions.iter().map(|x| x.encode()).collect(),
        }
    }
}
