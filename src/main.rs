#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

use clap::{Parser, Subcommand};
use itertools::Itertools;
use scry_asm::Assemble;
use scry_isa::{Instruction, Parser as ScryParser};
use std::{
	io,
	io::{Read, Write, stdout},
};

#[derive(clap::ValueEnum, Clone, Eq, PartialEq)]
enum Format
{
	/// Each byte is printed as a two-digit hexadecimal
	TextHex,

	/// Raw bytes
	Raw,
}

/// Command-line arguments
#[derive(Parser)]
struct Cli
{
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
enum Command
{
	/// (Default) Assemble textual assembly to machine code
	Assemble
	{
		#[clap(long)]
		#[arg(value_enum, default_value_t = Format::TextHex)]
		out_format: Format,
	},

	/// Disassemble machine code to textual assembly
	Disassemble
	{
		#[clap(long)]
		#[arg(value_enum, default_value_t = Format::Raw)]
		in_format: Format,
	},
}

fn main() -> io::Result<()>
{
	let args = Cli::parse();

	let mut stdin_buf = Vec::new();
	io::stdin().read_to_end(&mut stdin_buf)?;

	if let Command::Assemble { out_format } = args.command
	{
		let text_asm = String::from_utf8(stdin_buf).unwrap();
		let assembled = scry_asm::Raw::assemble(std::iter::once(text_asm.as_str())).unwrap();

		if out_format == Format::TextHex
		{
			let mut iter = assembled.iter();
			print!("{:02x}", iter.next().unwrap());
			iter.for_each(|b| print!(" {:02x}", b));
		}
		else
		{
			stdout().write(assembled.as_slice()).unwrap();
		};
	}
	else
	{
		// Input is encoded machine code, decode it
		for (b1, b2) in stdin_buf.iter().tuples()
		{
			let encoded_instr = u16::from_le_bytes([*b1, *b2]);
			let mut text_instr = String::new();

			Instruction::print(&Instruction::decode(encoded_instr), &mut text_instr).unwrap();
			print!("{} ", text_instr);
		}
	}

	Ok(())
}
