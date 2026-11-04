use std::{fs, path::{Component, Path}};
use crate::{model::MAX_ENTRIES, Result};

pub(crate) fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() || name.len() > 64 || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_') {
        return Err("Nome do snapshot: use 1 a 64 letras ASCII, números, hífen ou sublinhado".into());
    }
    Ok(())
}

pub(crate) fn validate_relative(name: &str) -> Result<()> {
    if name.is_empty() || name.contains(['\\', ':']) || name.chars().any(char::is_control)
        || name.split('/').any(|part| part.is_empty() || part == "." || part == "..")
        || Path::new(name).components().any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(format!("Caminho relativo não permitido: {name:?}").into());
    }
    Ok(())
}

pub(crate) fn regular_file(path: &Path) -> Result<()> {
    if !fs::symlink_metadata(path)?.file_type().is_file() {
        return Err(format!("Esperado arquivo regular, sem link: {}", path.display()).into());
    }
    Ok(())
}

pub(crate) fn real_directory(path: &Path) -> Result<()> {
    if !fs::symlink_metadata(path)?.file_type().is_dir() {
        return Err(format!("Esperado diretório, sem link: {}", path.display()).into());
    }
    Ok(())
}

pub(crate) fn collect_files(root: &Path) -> Result<Vec<String>> {
    fn visit(root: &Path, directory: &Path, files: &mut Vec<String>, depth: usize) -> Result<()> {
        if depth > 128 { return Err("Profundidade máxima de 128 diretórios excedida".into()); }
        let mut children = fs::read_dir(directory)?.collect::<std::io::Result<Vec<_>>>()?;
        children.sort_by_key(|entry| entry.file_name());
        for child in children {
            let kind = child.file_type()?;
            if kind.is_dir() {
                visit(root, &child.path(), files, depth + 1)?;
            } else if kind.is_file() {
                let absolute = child.path();
                let mut parts = Vec::new();
                for part in absolute.strip_prefix(root)?.components() {
                    parts.push(part.as_os_str().to_str().ok_or("Nome de arquivo não é UTF-8")?.to_owned());
                }
                let name = parts.join("/");
                validate_relative(&name)?;
                files.push(name);
                if files.len() > MAX_ENTRIES { return Err("Limite de 100 mil arquivos excedido".into()); }
            } else {
                return Err(format!("Link ou arquivo especial não permitido: {}", child.path().display()).into());
            }
        }
        Ok(())
    }
    let mut result = Vec::new();
    visit(root, root, &mut result, 0)?;
    result.sort();
    Ok(result)
}
