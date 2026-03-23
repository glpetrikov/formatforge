mod formats;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

/// translator — convert between data formats
///
/// Supported formats:
///
///   json  — JSON
///
///   toml  — TOML (no null values)
///
///   yaml  — YAML
///
///   xml   — XML
///
///   csv   — CSV (top-level array of objects)
///
///   env   — .env key=value
///
///   kdl   — KDL (kdl-lang.com)
///
///   cbor  — CBOR binary (pack/unpack)
///
///   ini    — INI sections
///
///   json5  — JSON5 (comments, trailing commas)
///
///   jsonc  — JSONC (JSON with comments)
///
///   msgpack — MessagePack binary
///
/// Examples:
///
///   translator data.json data.toml
///
///   translator data.json data.cbor        # pack to binary ===
///   translator data.cbor data.json        # unpack from binary
///
///   translator data.json out.yaml --to yaml
#[derive(Parser)]
#[command(name = "translator", version)]
struct Cli {
    /// Input file (format detected from extension)
    input: PathBuf,

    /// Output file (format detected from extension)
    output: PathBuf,

    /// Override input format (e.g. --from json)
    #[arg(long)]
    from: Option<String>,

    /// Override output format (e.g. --to toml)
    #[arg(long)]
    to: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let in_fmt = cli
        .from
        .as_deref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            formats::detect_format(&cli.input)
                .unwrap_or_else(|e| {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                })
                .to_string()
        });

    let out_fmt = cli.to.as_deref().map(|s| s.to_string()).unwrap_or_else(|| {
        formats::detect_format(&cli.output)
            .unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            })
            .to_string()
    });

    let value = formats::read_to_value(&cli.input, &in_fmt)
        .with_context(|| format!("Failed to read {:?} as {}", cli.input, in_fmt))?;

    formats::write_from_value(&value, &cli.output, &out_fmt)
        .with_context(|| format!("Failed to write {:?} as {}", cli.output, out_fmt))?;

    println!(
        "✓ {} ({}) → {} ({})",
        cli.input.display(),
        in_fmt,
        cli.output.display(),
        out_fmt
    );

    Ok(())
}
