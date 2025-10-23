use anyhow::Result;
use clap::{Parser, Subcommand};

// Re-use modules from lib.rs
use zkenc_cli::circuit;
use zkenc_cli::crypto;
use zkenc_cli::r1cs;
use zkenc_cli::witness;

mod commands;

/// zkenc CLI - 零知識證明工具
#[derive(Parser)]
#[command(name = "zkenc")]
#[command(about = "zkenc - Zero-Knowledge Encryption CLI Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encapsulate: 產生密文和金鑰 (使用電路和公開輸入)
    Encap {
        /// R1CS 電路檔案路徑 (.r1cs)
        #[arg(short, long)]
        circuit: String,
        /// 公開輸入 JSON 檔案 (e.g., sudoku puzzle)
        #[arg(short, long)]
        input: String,
        /// 輸出密文檔案路徑
        #[arg(short, long)]
        ciphertext: String,
        /// 輸出金鑰檔案路徑
        #[arg(short, long)]
        key: String,
    },
    /// Decapsulate: 恢復金鑰 (使用電路和完整 witness)
    Decap {
        /// R1CS 電路檔案路徑 (.r1cs)
        #[arg(short, long)]
        circuit: String,
        /// Witness 檔案路徑 (.wtns from snarkjs)
        #[arg(short, long)]
        witness: String,
        /// 密文檔案路徑
        #[arg(short, long)]
        ciphertext: String,
        /// 輸出金鑰檔案路徑
        #[arg(short, long)]
        key: String,
    },
    /// Encrypt: 使用金鑰加密訊息
    Encrypt {
        /// 金鑰檔案路徑
        #[arg(short, long)]
        key: String,
        /// 輸入訊息檔案
        #[arg(short, long)]
        input: String,
        /// 輸出加密檔案
        #[arg(short, long)]
        output: String,
    },
    /// Decrypt: 使用金鑰解密訊息
    Decrypt {
        /// 金鑰檔案路徑
        #[arg(short, long)]
        key: String,
        /// 輸入加密檔案
        #[arg(short, long)]
        input: String,
        /// 輸出解密檔案
        #[arg(short, long)]
        output: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encap {
            circuit,
            input,
            ciphertext,
            key,
        } => {
            commands::encap_command(&circuit, &input, &ciphertext, &key)?;
        }
        Commands::Decap {
            circuit,
            witness,
            ciphertext,
            key,
        } => {
            commands::decap_command(&circuit, &witness, &ciphertext, &key)?;
        }
        Commands::Encrypt { key, input, output } => {
            commands::encrypt_command(&key, &input, &output)?;
        }
        Commands::Decrypt { key, input, output } => {
            commands::decrypt_command(&key, &input, &output)?;
        }
    }

    Ok(())
}
