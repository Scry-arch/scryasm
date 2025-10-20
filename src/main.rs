#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

use clap::Parser;
use scry_asm::Assemble;
use std::{
	io,
	io::{Write, stdout},
};

#[derive(clap::ValueEnum, Clone, Eq, PartialEq)]
enum OutputFormat
{
	/// Each output byte is printed as a two-digit hexadecimal
	TextHex,

	/// Output directly
	Raw,
}

/// Command-line arguments
#[derive(Parser)]
struct Cli
{
	/// Disassemble machine code to textual assembly
	#[clap(short, long)]
	disassemble: bool,

	#[clap(long)]
	#[arg(value_enum, default_value_t = OutputFormat::TextHex)]
	out_format: OutputFormat,
}

fn main() -> io::Result<()>
{
	let args = Cli::parse();

	let mut stdin_buf = String::new();
	io::stdin().read_line(&mut stdin_buf)?;

	// File is in textual assembly, assemble it
	let assembled = scry_asm::Raw::assemble(std::iter::once(stdin_buf.as_str())).unwrap();

	if args.out_format == OutputFormat::TextHex
	{
		let mut iter = assembled.iter();
		print!("{:02x}", iter.next().unwrap());
		iter.for_each(|b| print!(" {:02x}", b));
	}
	else
	{
		stdout().write(assembled.as_slice()).unwrap();
	};
	Ok(())
}
