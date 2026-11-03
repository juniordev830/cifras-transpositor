use std::{env,fs::{self,OpenOptions},io, path::{Path,PathBuf},process::{self,Command}};
fn hash(path:&Path)->io::Result<String> {
    let result=Command::new("sha256sum").arg("--").arg(path).output()?;
    let text=String::from_utf8_lossy(&result.stdout);
    let value=text.split_whitespace().next().unwrap_or("");
    if !result.status.success() || value.len()!=64 || !value.chars().all(|c|c.is_ascii_hexdigit()) {
        return Err(io::Error::new(io::ErrorKind::InvalidData,"sha256sum falhou"));
    }
    Ok(value.to_owned())
}
fn files(source:&Path,target:&Path)->io::Result<Vec<PathBuf>> {
    let source=fs::canonicalize(source)?; let target=fs::canonicalize(target)?;
    if !source.is_dir() || !target.is_dir() || source.starts_with(&target) || target.starts_with(&source) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,"Origem e destino devem ser pastas separadas, sem aninhamento"));
    }
    let mut result=Vec::new();
    for entry in fs::read_dir(&source)? {
        let entry=entry?;
        if entry.file_type()?.is_file() {result.push(entry.path());}
    }
    result.sort(); Ok(result)
}
fn copy_verified(source:&Path,target:&Path)->io::Result<()> {
    if !fs::symlink_metadata(source)?.file_type().is_file(){return Err(io::Error::new(io::ErrorKind::InvalidInput,"Origem não é arquivo regular"));}
    let before=hash(source)?;
    let mut input=fs::File::open(source)?;
    let mut output=OpenOptions::new().write(true).create_new(true).open(target)?;
    let result=(||{io::copy(&mut input,&mut output)?;output.sync_all()?;drop(output);
        if before!=hash(target)? || before!=hash(source)? {return Err(io::Error::new(io::ErrorKind::InvalidData,"Integridade não confirmada"));} Ok(())})();
    if result.is_err(){let _=fs::remove_file(target);} result
}
fn run()->Result<(),Box<dyn std::error::Error>> {
    let args:Vec<String>=env::args().skip(1).collect();
    if !(args.len()==2 || (args.len()==3 && args[2]=="--aplicar")){return Err("Uso: backup-verificado ORIGEM DESTINO [--aplicar]".into());}
    let source=Path::new(&args[0]);let target=Path::new(&args[1]);
    for file in files(source,target)? {
        let destination=target.join(file.file_name().unwrap());
        if destination.try_exists()? {
            if fs::symlink_metadata(&destination)?.file_type().is_file() && hash(&file)?==hash(&destination)? {println!("Íntegro: {}",file.display());}
            else {return Err(format!("Conflito; destino preservado: {}",destination.display()).into());}
        } else if args.len()==3 {copy_verified(&file,&destination)?;println!("Copiado e verificado: {}",destination.display());}
        else {println!("Prévia: {} -> {}",file.display(),destination.display());}
    }
    Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("Erro: {e}");process::exit(2);}}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn verifies_and_never_overwrites(){
        let p=env::temp_dir().join(format!("backup-{}-{}",process::id(),std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir(&p).unwrap();let source=p.join("source");let target=p.join("target");
        fs::write(&source,b"conteudo").unwrap();copy_verified(&source,&target).unwrap();
        assert_eq!(hash(&source).unwrap(),hash(&target).unwrap());
        fs::write(&source,b"alterado").unwrap();assert!(copy_verified(&source,&target).is_err());
        assert_eq!(fs::read(&target).unwrap(),b"conteudo");fs::remove_dir_all(p).unwrap();
    }
}
