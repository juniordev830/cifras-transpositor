use std::{path::PathBuf, process::ExitCode};
use backup_verificado::{Repository, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Snapshots locais com BLAKE3, deduplicação e verificação")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Cria um repositório em um caminho que ainda não existe.
    Init { repository: PathBuf },
    /// Calcula uma prévia sem gravar arquivos.
    Plan { source: PathBuf, repository: PathBuf },
    /// Grava um novo snapshot; objetos iguais são reutilizados.
    Save { source: PathBuf, repository: PathBuf, #[arg(long)] name: String },
    /// Verifica hashes e tamanhos de todos os arquivos do snapshot.
    Verify { repository: PathBuf, #[arg(long)] name: String },
    /// Recupera os arquivos em um diretório que ainda não existe.
    Restore { repository: PathBuf, destination: PathBuf, #[arg(long)] name: String },
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Init { repository } => {
            Repository::init(repository)?;
            println!("Repositório inicializado.");
        }
        Command::Plan { source, repository } => {
            println!("{}", serde_json::to_string_pretty(&Repository::open(repository)?.plan(source)?)?);
        }
        Command::Save { source, repository, name } => {
            println!("{}", serde_json::to_string_pretty(&Repository::open(repository)?.save(source, &name)?)?);
        }
        Command::Verify { repository, name } => {
            println!("{}", serde_json::to_string_pretty(&Repository::open(repository)?.verify(&name)?)?);
        }
        Command::Restore { repository, destination, name } => {
            println!("{}", serde_json::to_string_pretty(&Repository::open(repository)?.restore(&name, destination)?)?);
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => { eprintln!("Erro: {error}"); ExitCode::from(1) }
    }
}
