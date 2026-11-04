//! Snapshots locais: objetos por conteúdo e manifestos versionados.
mod model;
mod paths;
mod repository;

pub use model::{Entry, Manifest, Plan, Verification};
pub use repository::Repository;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
