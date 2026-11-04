use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tempfile::NamedTempFile;
use crate::{model::FORMAT_VERSION, paths, Entry, Manifest, Plan, Result, Verification};

const MAX_MANIFEST_BYTES: u64 = 16 * 1024 * 1024;
pub struct Repository { root: PathBuf }
struct WriteLock(PathBuf);

impl WriteLock {
    fn acquire(root: &Path) -> Result<Self> {
        let path = root.join(".write-lock");
        let mut file = OpenOptions::new().write(true).create_new(true).open(&path)
            .map_err(|e| format!("Não foi possível obter a trava de escrita em {}: {e}", path.display()))?;
        let lock = Self(path);
        writeln!(file, "pid={}", std::process::id())?;
        Ok(lock)
    }
}
impl Drop for WriteLock {
    fn drop(&mut self) { let _ = fs::remove_file(&self.0); }
}

fn digest(path: &Path) -> Result<(String, u64)> {
    paths::regular_file(path)?;
    let mut file = File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut bytes = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
        bytes = bytes.checked_add(count as u64).ok_or("Tamanho excede u64")?;
    }
    Ok((hasher.finalize().to_hex().to_string(), bytes))
}

impl Repository {
    pub fn init(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        fs::create_dir(path)?;
        fs::create_dir(path.join("objects"))?;
        fs::create_dir(path.join("manifests"))?;
        Self::open(path)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let root = fs::canonicalize(path)?;
        paths::real_directory(&root)?;
        paths::real_directory(&root.join("objects"))?;
        paths::real_directory(&root.join("manifests"))?;
        Ok(Self { root })
    }

    fn source_root(&self, source: &Path) -> Result<PathBuf> {
        let root = fs::canonicalize(source)?;
        paths::real_directory(&root)?;
        if root.starts_with(&self.root) || self.root.starts_with(&root) {
            return Err("Origem e repositório não podem ser iguais ou aninhados".into());
        }
        Ok(root)
    }

    fn manifest_path(&self, name: &str) -> Result<PathBuf> {
        paths::validate_name(name)?;
        Ok(self.root.join("manifests").join(format!("{name}.json")))
    }

    fn object_path(&self, hash: &str) -> PathBuf { self.root.join("objects").join(hash) }

    fn verify_entry(&self, entry: &Entry) -> Result<()> {
        let (hash, bytes) = digest(&self.object_path(&entry.hash))?;
        if hash != entry.hash || bytes != entry.bytes {
            return Err(format!("Objeto corrompido: {} ({})", entry.path, entry.hash).into());
        }
        Ok(())
    }

    /// Prévia somente de leitura: não cria arquivos nem trava de escrita.
    pub fn plan(&self, source: impl AsRef<Path>) -> Result<Plan> {
        let root = self.source_root(source.as_ref())?;
        let mut plan = Plan { files: 0, total_bytes: 0, new_objects: 0, reused_objects: 0, new_object_bytes: 0, entries: Vec::new() };
        let mut seen = HashSet::new();
        for path in paths::collect_files(&root)? {
            let (hash, bytes) = digest(&root.join(&path))?;
            let entry = Entry { path, bytes, hash };
            plan.total_bytes = plan.total_bytes.checked_add(bytes).ok_or("Total excede u64")?;
            if seen.insert(entry.hash.clone()) {
                match fs::symlink_metadata(self.object_path(&entry.hash)) {
                    Ok(_) => { self.verify_entry(&entry)?; plan.reused_objects += 1; }
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                        plan.new_objects += 1;
                        plan.new_object_bytes = plan.new_object_bytes.checked_add(bytes).ok_or("Total excede u64")?;
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            plan.entries.push(entry);
        }
        plan.files = plan.entries.len();
        Manifest { format_version: FORMAT_VERSION, snapshot: "plan".into(), created_unix_seconds: 0, entries: plan.entries.clone() }.validate()?;
        Ok(plan)
    }

    pub fn save(&self, source: impl AsRef<Path>, name: &str) -> Result<Manifest> {
        let target = self.manifest_path(name)?;
        let _lock = WriteLock::acquire(&self.root)?;
        match fs::symlink_metadata(&target) {
            Ok(_) => return Err("Snapshot já existe; escolha outro nome".into()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        let root = self.source_root(source.as_ref())?;
        let plan = self.plan(&root)?;
        let mut stored = HashSet::new();
        for entry in &plan.entries {
            if !stored.insert(entry.hash.clone()) { continue; }
            let object = self.object_path(&entry.hash);
            if fs::symlink_metadata(&object).is_ok() { self.verify_entry(entry)?; continue; }
            let input_path = root.join(&entry.path);
            paths::regular_file(&input_path)?;
            let mut input = File::open(input_path)?;
            let mut temporary = NamedTempFile::new_in(self.root.join("objects"))?;
            std::io::copy(&mut input, &mut temporary)?;
            temporary.as_file().sync_all()?;
            let (hash, bytes) = digest(temporary.path())?;
            if hash != entry.hash || bytes != entry.bytes {
                return Err(format!("Origem mudou durante a cópia: {}", entry.path).into());
            }
            temporary.persist_noclobber(&object).map_err(|error| error.error)?;
        }
        let manifest = Manifest { format_version: FORMAT_VERSION, snapshot: name.to_owned(), created_unix_seconds: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(), entries: plan.entries };
        manifest.validate()?;
        let mut temporary = NamedTempFile::new_in(self.root.join("manifests"))?;
        serde_json::to_writer_pretty(&mut temporary, &manifest)?;
        temporary.write_all(b"\n")?;
        if temporary.as_file().metadata()?.len() > MAX_MANIFEST_BYTES { return Err("Manifesto excede 16 MiB".into()); }
        temporary.as_file().sync_all()?;
        temporary.persist_noclobber(target).map_err(|error| error.error)?;
        Ok(manifest)
    }

    pub fn read_manifest(&self, name: &str) -> Result<Manifest> {
        let path = self.manifest_path(name)?;
        paths::regular_file(&path)?;
        if fs::metadata(&path)?.len() > MAX_MANIFEST_BYTES { return Err("Manifesto excede 16 MiB".into()); }
        let manifest: Manifest = serde_json::from_reader(File::open(path)?)?;
        manifest.validate()?;
        if manifest.snapshot != name { return Err("Nome do manifesto não corresponde ao arquivo".into()); }
        Ok(manifest)
    }

    pub fn verify(&self, name: &str) -> Result<Verification> {
        let manifest = self.read_manifest(name)?;
        self.verify_manifest(&manifest)
    }

    fn verify_manifest(&self, manifest: &Manifest) -> Result<Verification> {
        let mut checked = HashSet::new();
        let mut bytes = 0u64;
        for entry in &manifest.entries {
            if checked.insert((entry.hash.clone(), entry.bytes)) { self.verify_entry(entry)?; }
            bytes = bytes.checked_add(entry.bytes).ok_or("Total excede u64")?;
        }
        Ok(Verification { snapshot: manifest.snapshot.clone(), files: manifest.entries.len(), bytes })
    }

    /// Verifica primeiro e exige um destino novo; erros posteriores preservam a pasta parcial.
    pub fn restore(&self, name: &str, destination: impl AsRef<Path>) -> Result<Verification> {
        let manifest = self.read_manifest(name)?;
        let verification = self.verify_manifest(&manifest)?;
        let destination = destination.as_ref();
        let parent = fs::canonicalize(destination.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new(".")))?;
        if parent.starts_with(&self.root) { return Err("Restauração não pode ficar dentro do repositório".into()); }
        fs::create_dir(destination)?;
        for entry in &manifest.entries {
            let path = destination.join(&entry.path);
            if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
            let mut input = File::open(self.object_path(&entry.hash))?;
            let mut output = OpenOptions::new().write(true).create_new(true).open(&path)?;
            std::io::copy(&mut input, &mut output)?;
            output.sync_all()?;
            let (hash, bytes) = digest(&path)?;
            if hash != entry.hash || bytes != entry.bytes { return Err("Integridade da restauração não confirmada".into()); }
        }
        Ok(verification)
    }
}
