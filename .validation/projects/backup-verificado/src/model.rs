use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use crate::{paths, Result};

pub const FORMAT_VERSION: u32 = 1;
pub const MAX_ENTRIES: usize = 100_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub path: String,
    pub bytes: u64,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format_version: u32,
    pub snapshot: String,
    pub created_unix_seconds: u64,
    pub entries: Vec<Entry>,
}

impl Manifest {
    pub fn validate(&self) -> Result<()> {
        if self.format_version != FORMAT_VERSION { return Err("Versão de manifesto não suportada".into()); }
        paths::validate_name(&self.snapshot)?;
        if self.entries.len() > MAX_ENTRIES { return Err("Manifesto excede 100 mil arquivos".into()); }
        let mut names = HashSet::new();
        for entry in &self.entries {
            paths::validate_relative(&entry.path)?;
            if entry.hash.len() != 64 || !entry.hash.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
                return Err("Hash BLAKE3 inválido no manifesto".into());
            }
            if !names.insert(entry.path.to_lowercase()) { return Err("Caminhos duplicados ou com conflito de maiúsculas".into()); }
        }
        for entry in &self.entries {
            let parts: Vec<_> = entry.path.split('/').collect();
            for end in 1..parts.len() {
                if names.contains(&parts[..end].join("/").to_lowercase()) { return Err("Um arquivo foi usado como diretório no manifesto".into()); }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct Plan {
    pub files: usize,
    pub total_bytes: u64,
    pub new_objects: usize,
    pub reused_objects: usize,
    pub new_object_bytes: u64,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize)]
pub struct Verification {
    pub snapshot: String,
    pub files: usize,
    pub bytes: u64,
}
