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

use anyhow::Result;

pub trait Address {}
pub trait PrivateKey {}

pub trait Program: Executable {
    type ID;
    type ProofSystem;
    type ProvingKey;
    type VerifyingKey;

    fn id(&self) -> Self::ID;
}

// Use Option for each element.
pub trait Record {
    // type Owner: Address;
    // type Value;
    // type Payload;
    // type BirthProgram: Program;
    // type DeathProgram: Program;
    // type Commitment;
    // type CommitmentRandomness;
    // type SerialNumber;
    // type SerialNumberNonce;
    // type SerialNumberNoneRandomness;

    fn is_dummy(&self) -> bool;
}

pub trait Register {
    type Data;
}

pub enum State<Record: crate::Record, Register: crate::Register> {
    Input(u8, Record),
    Output(u8, Record),
    Register(u8, Register),
}

pub trait Execution {
    type Proof;
    type VerifierInputs;
    type VerifyingKey;

    fn is_valid(&self) -> bool;

    fn proof(&self) -> Self::Proof;

    fn verifier_inputs(&self) -> Self::VerifierInputs;

    fn verifying_key(&self) -> Self::VerifyingKey;
}

pub trait Executable {
    type Arguments;
    type Execution: Execution;
    type Record: Record;
    type Register: Register;
    type State;

    fn add_state(&mut self, state: Self::State);

    fn execute(&self, arguments: Self::Arguments) -> Self::Execution;
}

pub trait TransactionIn {
    type Execution: Execution;
    type PrivateKey: PrivateKey;
    type Record: Record;
    type Signature;
}

pub trait TransactionOut {
    type Execution: Execution;
    type Record: Record;
}

pub trait TransactionBuilder {
    type Input: TransactionIn;
    type Output: TransactionOut;
    type Transaction;

    fn new() -> Self;

    fn add_input(&mut self, input: Self::Input) -> Self;

    fn add_inputs(&mut self, inputs: &[Self::Input]) -> Self;

    fn add_output(&mut self, output: Self::Output) -> Self;

    fn add_outputs(&mut self, outputs: &[Self::Output]) -> Self;

    fn build(&self) -> Self::Transaction;
}

// pub trait StateTransition {
//     type Input: Input;
//     type Output: Output;
//     type Transaction;
//
//     fn add_input(&mut self, input: Self::Input) -> Result<()>;
//
//     fn add_output(&mut self, output: Self::Output) -> Result<()>;
//
//     fn execute(&self, other: Option<impl Self>) -> Self::Transaction;
// }

pub trait Account {
    type Address: Address;
    type Amount;
    type PrivateKey: PrivateKey;
    type Record: Record;

    fn from(private_key: Self::PrivateKey) -> Self;

    // fn load_records() -> Self;

    fn add_record(&mut self, record: Self::Record);

    fn get_records_for_balance(&self, balance: &Self::Amount) -> Result<Vec<Self::Record>>;
}

// pub trait Record {
//
//     fn add_input_program(&mut self) -> Result<()>;
//
//
// }

// pub trait Program: StateTransition + TransactionBuilder {
//     type Sender: Account;
//     type Receiver;
//     type Amount;
//
//     type InputRecord: InputRecord;
//     type OutputRecord: OutputRecord;
//
//     fn send(sender: Self::Sender, receiver: Self::Receiver, amount: Self::Amount) -> Result<Self> {
//         let input_records = sender.get_records_for_balance(amount)?;
//         let output_0 = Self::OutputRecord::build().owner(receiver).value(amount).build();
//         Ok(TransactionBuilder::new().add_input_all(input_records).add_output(output_0).build())
//     }
// }

// pub trait Program {
//     fn execute_dummy() -> Self::
// }
