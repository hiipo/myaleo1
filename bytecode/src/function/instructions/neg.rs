// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{
    function::{parsers::*, Instruction, Opcode, Operation, Registers},
    helpers::Register,
    LiteralType,
    OutputType,
    Program,
    Value,
};
use snarkvm_circuits::{
    count,
    output_mode,
    CircuitCount,
    CircuitOrMode,
    Count,
    Field,
    Group,
    Literal,
    OutputMode,
    Parser,
    ParserResult,
    I8,
    U8,
};
use snarkvm_utilities::{FromBytes, ToBytes};

use core::fmt;
use nom::combinator::map;
use std::{
    io::{Read, Result as IoResult, Write},
    ops::Neg as NegOp,
};

/// Negates `first`, storing the outcome in `destination`.
pub struct Neg<P: Program> {
    operation: UnaryOperation<P>,
}

impl<P: Program> Neg<P> {
    /// Returns the operands of the instruction.
    pub fn operands(&self) -> Vec<Operand<P>> {
        self.operation.operands()
    }

    /// Returns the destination register of the instruction.
    pub fn destination(&self) -> &Register<P> {
        self.operation.destination()
    }
}

impl<P: Program> Opcode for Neg<P> {
    /// Returns the opcode as a string.
    #[inline]
    fn opcode() -> &'static str {
        "neg"
    }
}

impl<P: Program> Operation<P> for Neg<P> {
    /// Evaluates the operation.
    #[inline]
    fn evaluate(&self, registers: &Registers<P>) {
        // Load the values for the first and second operands.
        let first = match registers.load(self.operation.first()) {
            Value::Literal(literal) => literal,
            Value::Composite(name, ..) => P::halt(format!("{name} is not a literal")),
        };

        // Perform the operation.
        let result = match first {
            Literal::Field(a) => Literal::Field(-a),
            Literal::Group(a) => Literal::Group(-a),
            Literal::I8(a) => Literal::I8(-a),
            Literal::I16(a) => Literal::I16(-a),
            Literal::I32(a) => Literal::I32(-a),
            Literal::I64(a) => Literal::I64(-a),
            Literal::I128(a) => Literal::I128(-a),
            _ => P::halt(format!("Invalid '{}' instruction", Self::opcode())),
        };

        registers.assign(self.operation.destination(), result);
    }
}

impl<P: Program> Count<Self> for Neg<P> {
    type Case = LiteralType<P>;

    fn count(input: &Self::Case) -> CircuitCount {
        match input {
            LiteralType::Field(mode) => {
                count!(Field<P::Environment>, NegOp<Output = Field<P::Environment>>, &CircuitOrMode::Mode(*mode))
            }
            LiteralType::Group(mode) => {
                count!(Group<P::Environment>, NegOp<Output = Group<P::Environment>>, &CircuitOrMode::Mode(*mode))
            }
            LiteralType::I8(mode) => {
                count!(I8<P::Environment>, NegOp<Output = I8<P::Environment>>, &CircuitOrMode::Mode(*mode))
            }
            LiteralType::U8(mode) => {
                count!(U8<P::Environment>, NegOp<Output = U8<P::Environment>>, &CircuitOrMode::Mode(*mode))
            }
            _ => P::halt(format!("Invalid '{}' instruction", Self::opcode())),
        }
    }
}

impl<P: Program> OutputType for Neg<P> {
    type Input = LiteralType<P>;
    type Output = LiteralType<P>;

    fn output_type(input_type: &Self::Input) -> Self::Output {
        match input_type {
            LiteralType::Field(mode) => LiteralType::Field(output_mode!(
                Field<P::Environment>,
                NegOp<Output = Field<P::Environment>>,
                &CircuitOrMode::Mode(*mode)
            )),
            LiteralType::Group(mode) => LiteralType::Group(output_mode!(
                Group<P::Environment>,
                NegOp<Output = Group<P::Environment>>,
                &CircuitOrMode::Mode(*mode)
            )),
            LiteralType::I8(mode) => LiteralType::I8(output_mode!(
                I8<P::Environment>,
                NegOp<Output = I8<P::Environment>>,
                &CircuitOrMode::Mode(*mode)
            )),
            LiteralType::U8(mode) => LiteralType::U8(output_mode!(
                U8<P::Environment>,
                NegOp<Output = U8<P::Environment>>,
                &CircuitOrMode::Mode(*mode)
            )),
            _ => P::halt(format!("Invalid '{}' instruction", Self::opcode())),
        }
    }
}

impl<P: Program> Parser for Neg<P> {
    type Environment = P::Environment;

    /// Parses a string into an 'neg' operation.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        // Parse the operation from the string.
        map(UnaryOperation::parse, |operation| Self { operation })(string)
    }
}

impl<P: Program> fmt::Display for Neg<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.operation)
    }
}

impl<P: Program> FromBytes for Neg<P> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        Ok(Self { operation: UnaryOperation::read_le(&mut reader)? })
    }
}

impl<P: Program> ToBytes for Neg<P> {
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.operation.write_le(&mut writer)
    }
}

#[allow(clippy::from_over_into)]
impl<P: Program> Into<Instruction<P>> for Neg<P> {
    /// Converts the operation into an instruction.
    fn into(self) -> Instruction<P> {
        Instruction::Neg(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_instruction_halts, test_modes, Process};

    #[test]
    fn test_parse() {
        let (_, instruction) = Instruction::<Process>::parse("neg r0 into r1;").unwrap();
        assert!(matches!(instruction, Instruction::Neg(_)));
    }

    test_modes!(field, Neg, "1field", "-1field");
    test_modes!(group, Neg, "2group", "-2group");
    test_modes!(i8, Neg, "1i8", "-1i8");
    test_modes!(i16, Neg, "1i16", "-1i16");
    test_modes!(i32, Neg, "1i32", "-1i32");
    test_modes!(i64, Neg, "1i64", "-1i64");
    test_modes!(i128, Neg, "1i128", "-1i128");

    test_instruction_halts!(
        i8_min_neg_halts,
        Neg,
        "Integer overflow on addition of two constants",
        &format!("{}i8", i8::MIN)
    );
    test_instruction_halts!(
        i16_min_neg_halts,
        Neg,
        "Integer overflow on addition of two constants",
        &format!("{}i16", i16::MIN)
    );
    test_instruction_halts!(
        i32_min_neg_halts,
        Neg,
        "Integer overflow on addition of two constants",
        &format!("{}i32", i32::MIN)
    );
    test_instruction_halts!(
        i64_min_neg_halts,
        Neg,
        "Integer overflow on addition of two constants",
        &format!("{}i64", i64::MIN)
    );
    test_instruction_halts!(
        i128_min_neg_halts,
        Neg,
        "Integer overflow on addition of two constants",
        &format!("{}i128", i128::MIN)
    );
    test_instruction_halts!(u8_neg_halts, Neg, "Invalid 'neg' instruction", "1u8");
    test_instruction_halts!(u16_neg_halts, Neg, "Invalid 'neg' instruction", "1u16");
    test_instruction_halts!(u32_neg_halts, Neg, "Invalid 'neg' instruction", "1u32");
    test_instruction_halts!(u64_neg_halts, Neg, "Invalid 'neg' instruction", "1u64");
    test_instruction_halts!(u128_neg_halts, Neg, "Invalid 'neg' instruction", "1u128");
    test_instruction_halts!(scalar_neg_halts, Neg, "Invalid 'neg' instruction", "1scalar.constant");
    test_instruction_halts!(
        address_neg_halts,
        Neg,
        "Invalid 'neg' instruction",
        "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.constant"
    );
    test_instruction_halts!(boolean_neg_halts, Neg, "Invalid 'neg' instruction", "true.constant");
    test_instruction_halts!(string_neg_halts, Neg, "Invalid 'neg' instruction", "\"hello\".constant");
}
