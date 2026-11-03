use std::{env,fs::{self,OpenOptions},io::Write,process};
use regex::Regex;
fn redact(input:&str)->(String,Vec<(&'static str,usize)>){
    let rules=[("credencial",r"(?i)\b(?:token|password|senha|api_key)\s*[=:]\s*[^\s,;]+","[CREDENCIAL]"),
               ("email",r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b","[EMAIL]"),
               ("cpf",r"\b[0-9]{3}\.?[0-9]{3}\.?[0-9]{3}-?[0-9]{2}\b","[CPF]")];
    let mut output=input.to_owned();let mut summary=Vec::new();
    for (name,pattern,replacement) in rules {
        let regex=Regex::new(pattern).expect("padrão interno inválido");
        let count=regex.find_iter(&output).count();output=regex.replace_all(&output,replacement).into_owned();summary.push((name,count));
    }
    (output,summary)
}
fn run()->Result<(),Box<dyn std::error::Error>> {
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()!=1 && args.len()!=2{return Err("Uso: log-redactor-cli ENTRADA [NOVO_ARQUIVO_SAIDA]".into());}
    if fs::metadata(&args[0])?.len()>8*1024*1024{return Err("Arquivo excede 8 MiB".into());}
    let input=fs::read_to_string(&args[0])?;let (output,summary)=redact(&input);
    if args.len()==2 {
        let mut file=OpenOptions::new().create_new(true).write(true).open(&args[1])?;file.write_all(output.as_bytes())?;
    } else {print!("{output}");}
    for (rule,count) in summary{eprintln!("{rule}: {count} substituições");} Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("Erro: {e}");process::exit(2);}}
#[cfg(test)]mod tests{use super::*;
    #[test]fn known_patterns(){let (text,stats)=redact("email=demo@example.com CPF 123.456.789-00 token=abc123");assert!(!text.contains("example.com"));assert!(!text.contains("abc123"));assert!(!text.contains("123.456"));assert_eq!(stats.iter().map(|(_,n)|n).sum::<usize>(),3);}
    #[test]fn repeat_is_stable(){let a=redact("senha=demo\nErro normal").0;assert_eq!(redact(&a).0,a);assert!(a.contains("Erro normal"));}
}
