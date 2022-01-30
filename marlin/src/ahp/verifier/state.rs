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

use core::marker::PhantomData;

use crate::{
    ahp::verifier::{VerifierFirstMessage, VerifierSecondMessage},
    marlin::MarlinMode,
};
use snarkvm_algorithms::fft::EvaluationDomain;
use snarkvm_fields::PrimeField;

/// State of the AHP verifier.
#[derive(Debug)]
pub struct VerifierState<F: PrimeField, MM: MarlinMode> {
    pub domain_h: EvaluationDomain<F>,
    pub domain_k: EvaluationDomain<F>,

    pub first_round_message: Option<VerifierFirstMessage<F>>,
    pub second_round_message: Option<VerifierSecondMessage<F>>,

    pub gamma: Option<F>,
    pub(crate) mode: PhantomData<MM>,
}
