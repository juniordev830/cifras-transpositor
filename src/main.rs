mod music;
mod songbook;

use std::{env, fs, io::{self, Read}, process};

fn note(input: &str, shift: i32, flats: bool) -> Option<(String, usize)> {
    let (pitch, used) = music::pitch(input)?;
    let shifted = (i32::from(pitch) + shift.rem_euclid(12)).rem_euclid(12) as u8;
    Some((music::name(shifted, flats).to_owned(), used))
}

fn chord(token: &str, shift: i32, flats: bool) -> Option<String> {
    let (name, used) = note(token, shift, flats)?;
    let mut parts = token[used..].split('/');
    let suffix = parts.next()?;
    // Explicit grammar prevents treating arbitrary words as chords.
    let mut rest = suffix;
    if let Some(s) = ["maj", "min", "dim", "aug", "sus", "add", "m", "°", "ø"].iter().find(|s| rest.starts_with(**s)) {
        rest = &rest[s.len()..];
    }
    if !rest.chars().all(|c| c.is_ascii_digit() || "#b♯♭()+-".contains(c)) { return None; }
    let bass = match parts.next() {
        Some(b) => { let (transposed, n) = note(b, shift, flats)?; if n != b.len() { return None; } format!("/{transposed}") },
        None => String::new()
    };
    if parts.next().is_some() { return None; }
    Some(format!("{name}{suffix}{bass}"))
}

/// Transpose [C#m7/G#] chords embedded in lyrics without touching the lyrics.
fn bracketed_line(input: &str, shift: i32, flats: bool) -> Option<String> {
    let mut output = String::with_capacity(input.len());
    let mut remaining = input;
    let mut changed = false;
    while let Some(open) = remaining.find('[') {
        output.push_str(&remaining[..open]);
        let after_open = &remaining[open + 1..];
        let Some(close) = after_open.find(']') else {
            output.push_str(&remaining[open..]);
            remaining = "";
            break;
        };
        let inside = &after_open[..close];
        match chord(inside, shift, flats) {
            Some(transposed) => {
                output.push('[');
                output.push_str(&transposed);
                output.push(']');
                changed = true;
            }
            None => {
                // Preserve section headings and malformed bracketed text exactly.
                output.push_str(&remaining[open..open + close + 2]);
            }
        }
        remaining = &after_open[close + 1..];
    }
    output.push_str(remaining);
    changed.then_some(output)
}

fn line(input: &str, shift: i32, flats: bool) -> String {
    if let Some(transposed) = bracketed_line(input, shift, flats) {
        return transposed;
    }
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() || tokens.iter().any(|t| *t != "|" && chord(t, shift, flats).is_none()) {
        return input.to_owned();
    }
    // Keep chord starting columns when there is enough whitespace; never join chords.
    let mut output = String::new();
    let mut input_offset = 0;
    let mut input_column = 0;
    let mut output_column = 0;
    for token in tokens {
        let start = input_offset + input[input_offset..].find(token).unwrap();
        input_column += input[input_offset..start].chars().count();
        let needed = input_column.saturating_sub(output_column);
        let padding = if output.is_empty() { needed } else { needed.max(1) };
        let rendered = chord(token, shift, flats).unwrap_or_else(|| token.to_owned());
        output.push_str(&" ".repeat(padding));
        output.push_str(&rendered);
        output_column += padding + rendered.chars().count();
        input_column += token.chars().count();
        input_offset = start + token.len();
    }
    output.push_str(&input[input_offset..]);
    output
}

fn transpose(text: &str, shift: i32, flats: bool) -> String {
    text.split_inclusive('\n').map(|part| {
        let ending = if part.ends_with("\r\n") { "\r\n" } else if part.ends_with('\n') { "\n" } else { "" };
        format!("{}{}", line(part.trim_end_matches(['\r','\n']), shift, flats), ending)
    }).collect()
}

fn render(input: &str, shift: i32, flats: bool, format: &str) -> String {
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);
    if format == "texto" {
        return transpose(input, shift, flats);
    }
    let song = songbook::Song::parse(input).transpose(shift, flats);
    match format {
        "chordpro" => song.chordpro(),
        "html" => song.html(),
        "relatorio" => song.report(),
        _ => unreachable!(),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" {
        println!("Uso: cifras-transpositor SEMITONS [ARQUIVO|-] [--bemol] [--formato=texto|chordpro|html|relatorio]\nSem arquivo, lê stdin. Reconhece linhas de cifras e acordes [C] embutidos na letra.\nFormatos adicionais: ChordPro, HTML escapado e relatório de análise musical.");
        return Ok(());
    }
    let shift: i32 = args[0].parse().map_err(|_| "SEMITONS deve ser inteiro")?;
    let flats = args.iter().any(|a| a == "--bemol");
    let mut formats = args[1..].iter().filter_map(|arg| arg.strip_prefix("--formato="));
    let format = formats.next().unwrap_or("texto");
    if formats.next().is_some() {
        return Err("Informe --formato somente uma vez".into());
    }
    if !["texto", "chordpro", "html", "relatorio"].contains(&format) {
        return Err(format!("Formato inválido: {format}").into());
    }
    let paths: Vec<_> = args[1..].iter()
        .filter(|a| a.as_str() != "--bemol" && !a.starts_with("--formato=")).collect();
    if paths.len() > 1 { return Err("Informe somente um arquivo".into()); }
    let input = match paths.first() {
        Some(p) if p.as_str() != "-" => fs::read_to_string(p)?,
        _ => { let mut text=String::new(); io::stdin().read_to_string(&mut text)?; text }
    };
    print!("{}", render(&input, shift, flats, format));
    Ok(())
}
fn main() { if let Err(error) = run() { eprintln!("Erro: {error}"); process::exit(2); } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bom_does_not_hide_first_chord_or_directive() {
        assert_eq!(render("\u{feff}C  F\r\n", 2, false, "texto"), "D  G\r\n");
        let chordpro = "\u{feff}{title: Noite}\r\n{key: C}\r\n[C]Verso\r\n";
        assert_eq!(
            render(chordpro, 2, false, "chordpro"),
            "{title: Noite}\r\n{key: D}\r\n[D]Verso\r\n"
        );
        assert!(render(chordpro, 2, false, "html").contains("<h1>Noite</h1>"));
        assert!(render(chordpro, 2, false, "relatorio").contains("D"));
    }

    #[test] fn slash_and_negative() { assert_eq!(chord("C#m7/G#",-2,false).unwrap(),"Bm7/F#"); }
    #[test] fn enharmonic_and_overflow() { assert_eq!(chord("B#",i32::MAX,true).unwrap(),"G"); assert_eq!(chord("Cb",1,false).unwrap(),"C"); }
    #[test] fn preserve_lyrics_and_columns() { assert_eq!(transpose("C    G/B   Am\nA vida segue\n",2,false),"D    A/C#  Bm\nA vida segue\n"); }
    #[test] fn flat_output_and_crlf() { assert_eq!(transpose("C  F\r\n",1,true),"Db Gb\r\n"); }
    #[test] fn preserves_alterations_in_text_and_chordpro() {
        assert_eq!(transpose("[C7(♭9)]Letra [F♯m7♭5]\n", 2, false), "[D7(♭9)]Letra [G#m7♭5]\n");
        let song = songbook::Song::parse("[Bø7]Letra [D°7]\n");
        assert_eq!(song.transpose(2, false).chordpro(), "[C#ø7]Letra [E°7]\n");
    }
    #[test] fn transposes_unicode_accidentals_in_text_and_chordpro() {
        assert_eq!(transpose("C♯m7/G♯   B♭\n", -2, false), "Bm7/F#    G#\n");
        assert_eq!(transpose("[B♭]Verso [F♯/A♯]\n", 1, true), "[B]Verso [G/B]\n");
        let song = songbook::Song::parse("{key: B♭m}\n[B♭m]Verso\n");
        assert_eq!(song.transpose(2, true).chordpro(), "{key: Cm}\n[Cm]Verso\n");
    }
    #[test] fn transpose_inline_lyrics_and_bass_chords() {
        assert_eq!(
            transpose("[Am]Cantarei [G/B]com alegria\n", 2, false),
            "[Bm]Cantarei [A/C#]com alegria\n"
        );
    }
    #[test] fn preserve_headings_unmatched_brackets_and_crlf() {
        assert_eq!(
            transpose("[Refrão]\r\n[Am]Eu canto [F]sempre\r\n[sem fechamento\n", -2, true),
            "[Refrão]\r\n[Gm]Eu canto [Eb]sempre\r\n[sem fechamento\n"
        );
    }
    #[test] fn valid_inline_chord_does_not_modify_surrounding_words() {
        assert_eq!(
            transpose("Acordes [C] e [???] e texto\n", 2, false),
            "Acordes [D] e [???] e texto\n"
        );
    }
    #[test] fn reject_ambiguous_words() { assert!(chord("Deus",2,false).is_none()); assert_eq!(transpose("Canta ao Senhor",2,false),"Canta ao Senhor"); }
}
