use anyhow::Result;
use clap::{Parser, Subcommand};

// Re-use modules from lib.rs
use zkenc_cli::circuit;
use zkenc_cli::crypto;
use zkenc_cli::r1cs;
use zkenc_cli::witness;

mod commands;
mod sym_parser;

/// zkenc CLI - Zero-Knowledge Encryption Tool
#[derive(Parser)]
#[command(name = "zkenc")]
#[command(about = "zkenc - Zero-Knowledge Encryption CLI Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encapsulate: Generate ciphertext and key (using circuit and public input)
    Encap {
        /// R1CS circuit file path (.r1cs)
        #[arg(short, long)]
        circuit: String,
        /// Symbol file path (.sym from Circom) - Required for correct input mapping
        #[arg(short, long)]
        sym: String,
        /// Public input JSON file (e.g., sudoku puzzle)
        #[arg(short, long)]
        input: String,
        /// Output ciphertext file path
        #[arg(short, long)]
        ciphertext: String,
        /// Output key file path
        #[arg(short, long)]
        key: String,
    },
    /// Decapsulate: Recover key (using circuit and complete witness)
    Decap {
        /// R1CS circuit file path (.r1cs)
        #[arg(short, long)]
        circuit: String,
        /// Witness file path (.wtns from snarkjs)
        #[arg(short, long)]
        witness: String,
        /// Ciphertext file path
        #[arg(short, long)]
        ciphertext: String,
        /// Output key file path
        #[arg(short, long)]
        key: String,
    },
    /// Encrypt: High-level encryption (compatible with zkenc-js format)
    Encrypt {
        /// R1CS circuit file path (.r1cs)
        #[arg(short, long)]
        circuit: String,
        /// Symbol file path (.sym from Circom) - Required for correct input mapping
        #[arg(short, long)]
        sym: String,
        /// Public input JSON file
        #[arg(short, long)]
        input: String,
        /// Message file
        #[arg(short, long)]
        message: String,
        /// Output combined ciphertext file
        #[arg(short, long)]
        output: String,
        /// Do not include public input in ciphertext (default: includes it)
        #[arg(long, default_value = "false")]
        no_public_input: bool,
    },
    /// Decrypt: High-level decryption (compatible with zkenc-js format)
    Decrypt {
        /// R1CS circuit file path (.r1cs)
        #[arg(short, long)]
        circuit: String,
        /// Witness file path (.wtns from snarkjs)
        #[arg(short, long)]
        witness: String,
        /// Combined ciphertext file
        #[arg(short = 'i', long)]
        ciphertext: String,
        /// Output decrypted message file
        #[arg(short, long)]
        output: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encap {
            circuit,
            sym,
            input,
            ciphertext,
            key,
        } => {
            commands::encap_command(&circuit, &sym, &input, &ciphertext, &key)?;
        }
        Commands::Decap {
            circuit,
            witness,
            ciphertext,
            key,
        } => {
            commands::decap_command(&circuit, &witness, &ciphertext, &key)?;
        }

        Commands::Encrypt {
            circuit,
            sym,
            input,
            message,
            output,
            no_public_input,
        } => {
            commands::encrypt_command(&circuit, &sym, &input, &message, &output, !no_public_input)?;
        }
        Commands::Decrypt {
            circuit,
            witness,
            ciphertext,
            output,
        } => {
            commands::decrypt_command(&circuit, &witness, &ciphertext, &output)?;
        }
    }

    Ok(())
}
