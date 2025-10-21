use assert_cmd::{
	Command,
	assert::{Assert, AssertError},
};
use predicates::prelude::predicate;
use quickcheck_macros::quickcheck;
use scry_isa::{Instruction, Parser};
use std::fmt::Write;

/// Where temporary test files should be created.
pub const TEMPORARY_DIR: &'static str = "tests/tmp";

/// Executes the program with the given input on stdin, returning a testable
/// assert.
fn invoke_assemble<S: Into<Vec<u8>>>(input: S, args: &[&str]) -> Assert
{
	let mut cmd = Command::cargo_bin("scryasm").unwrap();

	cmd.write_stdin(input);
	cmd.args(args);

	cmd.assert()
}

fn to_encoded(instructions: &Vec<Instruction>) -> Vec<u8>
{
	instructions.iter().fold(Vec::new(), |mut v, b| {
		v.extend_from_slice(&b.encode().to_le_bytes());
		v
	})
}

/// Returns the textual assembly of the given instruction sequence
fn to_text_assembly(instructions: &Vec<Instruction>) -> String
{
	let mut asm_buf = String::new();
	for instr in instructions
	{
		Instruction::print(&instr, &mut asm_buf).unwrap();
		asm_buf.write_char(' ').unwrap();
	}
	asm_buf
}

/// Tests can get instructions in textual form as input through stdin
/// and output its hexadecimal encoding
#[quickcheck]
fn text_instruction_to_text_hex(
	first: Instruction,
	mut instructions: Vec<Instruction>,
) -> Result<(), AssertError>
{
	// Ensure there is at least 1 instruction
	instructions.push(first);

	let encoded = to_encoded(&instructions);

	// Convert encoding to textual hex
	let expected_text_hex = encoded.iter().fold(String::new(), |mut s, b| {
		write!(&mut s, "{:02x} ", b).unwrap();
		s
	});

	let asm_buf = to_text_assembly(&instructions);

	// Assemble
	let assert = invoke_assemble(asm_buf, &["assemble"]);

	// Test
	assert
		.try_code(predicate::eq(0))?
		.try_stdout(predicate::eq(expected_text_hex.trim()))?
		.try_stderr(predicates::str::is_empty())
		.map(|_| ())
}

/// Tests can get instructions in textual form as input through stdin
/// and output it raw
#[quickcheck]
fn text_instruction_to_raw(
	first: Instruction,
	mut instructions: Vec<Instruction>,
) -> Result<(), AssertError>
{
	// Ensure there is at least 1 instruction
	instructions.push(first);

	// Get instruction encoding
	let encoded = to_encoded(&instructions);

	let asm_buf = to_text_assembly(&instructions);

	// Assemble
	let assert = invoke_assemble(asm_buf, &["assemble", "--out-format=raw"]);

	// Test
	assert
		.try_code(predicate::eq(0))?
		.try_stdout(predicate::eq(encoded))?
		.try_stderr(predicates::str::is_empty())
		.map(|_| ())
}

/// Tests can get instructions in raw bytes and output textual assembly
#[quickcheck]
fn raw_instruction_to_text(
	first: Instruction,
	mut instructions: Vec<Instruction>,
) -> Result<(), AssertError>
{
	// Ensure there is at least 1 instruction
	instructions.push(first);

	let encoded = to_encoded(&instructions);

	let asm_buf = to_text_assembly(&instructions);

	// Assemble
	let assert = invoke_assemble(encoded, &["disassemble"]);

	// Test
	assert
		.try_code(predicate::eq(0))?
		.try_stdout(predicate::eq(asm_buf))?
		.try_stderr(predicates::str::is_empty())
		.map(|_| ())
}
