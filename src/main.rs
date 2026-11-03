use std::{env, fs, io::{self, Read}, process};

const SHARPS: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const FLATS: [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

fn note(input: &str, shift: i32, flats: bool) -> Option<(String, usize)> {
    let first = input.chars().next()?;
    let pitch: i32 = match first { 'C'=>0, 'D'=>2, 'E'=>4, 'F'=>5, 'G'=>7, 'A'=>9, 'B'=>11, _=>return None };
    let (accidental, used) = match input.as_bytes().get(1) { Some(b'#')=>(1,2), Some(b'b')=>(-1,2), _=>(0,1) };
    let names = if flats { &FLATS } else { &SHARPS };
    Some((names[(pitch + accidental + shift.rem_euclid(12)).rem_euclid(12) as usize].to_owned(), used))
}

fn chord(token: &str, shift: i32, flats: bool) -> Option<String> {
    let (name, used) = note(token, shift, flats)?;
    let mut parts = token[used..].split('/');
    let suffix = parts.next()?;
    // Explicit grammar prevents treating arbitrary words as chords.
    let mut rest = suffix;
    if let Some(s) = ["maj", "min", "dim", "aug", "sus", "add", "m"].iter().find(|s| rest.starts_with(**s)) {
        rest = &rest[s.len()..];
    }
    if !rest.chars().all(|c| c.is_ascii_digit() || "#b()+-".contains(c)) { return None; }
    let bass = match parts.next() {
        Some(b) => { let (transposed, n) = note(b, shift, flats)?; if n != b.len() { return None; } format!("/{transposed}") },
        None => String::new()
    };
    if parts.next().is_some() { return None; }
    Some(format!("{name}{suffix}{bass}"))
}

fn line(input: &str, shift: i32, flats: bool) -> String {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() || tokens.iter().any(|t| *t != "|" && chord(t, shift, flats).is_none()) {
        return input.to_owned();
    }
    // Keep chord starting columns when there is enough whitespace; never join chords.
    let mut output = String::new();
    let mut input_column = 0;
    for token in tokens {
        let offset = input[input_column..].find(token).unwrap();
        let start = input_column + offset;
        let needed = start.saturating_sub(output.len());
        output.push_str(&" ".repeat(if output.is_empty() { needed } else { needed.max(1) }));
        output.push_str(&chord(token, shift, flats).unwrap_or_else(|| token.to_owned()));
        input_column = start + token.len();
    }
    output.push_str(&input[input_column..]);
    output
}

fn transpose(text: &str, shift: i32, flats: bool) -> String {
    text.split_inclusive('\n').map(|part| {
        let ending = if part.ends_with("\r\n") { "\r\n" } else if part.ends_with('\n') { "\n" } else { "" };
        format!("{}{}", line(part.trim_end_matches(['\r','\n']), shift, flats), ending)
    }).collect()
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" {
        println!("Uso: cifras-transpositor SEMITONS [ARQUIVO|-] [--bemol]\nSem arquivo, lê stdin. Apenas linhas compostas de cifras são transpostas.");
        return Ok(());
    }
    let shift: i32 = args[0].parse().map_err(|_| "SEMITONS deve ser inteiro")?;
    let flats = args.iter().any(|a| a == "--bemol");
    let paths: Vec<_> = args[1..].iter().filter(|a| a.as_str() != "--bemol").collect();
    if paths.len() > 1 { return Err("Informe somente um arquivo".into()); }
    let input = match paths.first() {
        Some(p) if p.as_str() != "-" => fs::read_to_string(p)?,
        _ => { let mut text=String::new(); io::stdin().read_to_string(&mut text)?; text }
    };
    print!("{}", transpose(&input, shift, flats));
    Ok(())
}
fn main() { if let Err(error) = run() { eprintln!("Erro: {error}"); process::exit(2); } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn slash_and_negative() { assert_eq!(chord("C#m7/G#",-2,false).unwrap(),"Bm7/F#"); }
    #[test] fn enharmonic_and_overflow() { assert_eq!(chord("B#",i32::MAX,true).unwrap(),"G"); assert_eq!(chord("Cb",1,false).unwrap(),"C"); }
    #[test] fn preserve_lyrics_and_columns() { assert_eq!(transpose("C    G/B   Am\nA vida segue\n",2,false),"D    A/C#  Bm\nA vida segue\n"); }
    #[test] fn flat_output_and_crlf() { assert_eq!(transpose("C  F\r\n",1,true),"Db Gb\r\n"); }
    #[test] fn reject_ambiguous_words() { assert!(chord("Deus",2,false).is_none()); assert_eq!(transpose("Canta ao Senhor",2,false),"Canta ao Senhor"); }
}
