use crate::music::{self, ChordSymbol};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Segment {
    Text(String),
    Chord(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectiveKind {
    Title,
    Subtitle,
    Artist,
    Key,
    Capo,
    Comment,
    StartChorus,
    EndChorus,
    StartVerse,
    EndVerse,
}

impl DirectiveKind {
    fn canonical(&self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Subtitle => "subtitle",
            Self::Artist => "artist",
            Self::Key => "key",
            Self::Capo => "capo",
            Self::Comment => "comment",
            Self::StartChorus => "start_of_chorus",
            Self::EndChorus => "end_of_chorus",
            Self::StartVerse => "start_of_verse",
            Self::EndVerse => "end_of_verse",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    Directive { kind: DirectiveKind, value: String, ending: String },
    Lyrics { segments: Vec<Segment>, ending: String },
    Plain { text: String, ending: String },
}

impl Line {
    fn ending(&self) -> &str {
        match self {
            Self::Directive { ending, .. } |
            Self::Lyrics { ending, .. } |
            Self::Plain { ending, .. } => ending,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Song {
    pub lines: Vec<Line>,
}

fn split_ending(line: &str) -> (&str, &str) {
    if let Some(stripped) = line.strip_suffix("\r\n") {
        (stripped, "\r\n")
    } else if let Some(stripped) = line.strip_suffix('\n') {
        (stripped, "\n")
    } else {
        (line, "")
    }
}

fn parse_directive(line: &str) -> Option<(DirectiveKind, String)> {
    let body = line.strip_prefix('{')?.strip_suffix('}')?;
    let (raw_name, value) = body.split_once(':').unwrap_or((body, ""));
    let name = raw_name.trim().to_ascii_lowercase();
    let kind = match name.as_str() {
        "title" | "t" => DirectiveKind::Title,
        "subtitle" | "st" => DirectiveKind::Subtitle,
        "artist" | "a" => DirectiveKind::Artist,
        "key" | "k" => DirectiveKind::Key,
        "capo" => DirectiveKind::Capo,
        "comment" | "c" => DirectiveKind::Comment,
        "start_of_chorus" | "soc" => DirectiveKind::StartChorus,
        "end_of_chorus" | "eoc" => DirectiveKind::EndChorus,
        "start_of_verse" | "sov" => DirectiveKind::StartVerse,
        "end_of_verse" | "eov" => DirectiveKind::EndVerse,
        _ => return None,
    };
    let is_block = matches!(kind, DirectiveKind::StartChorus | DirectiveKind::EndChorus
        | DirectiveKind::StartVerse | DirectiveKind::EndVerse);
    if is_block && !value.trim().is_empty() { return None; }
    Some((kind, value.trim().to_owned()))
}

fn push_text(segments: &mut Vec<Segment>, text: &str) {
    if text.is_empty() { return; }
    if let Some(Segment::Text(last)) = segments.last_mut() {
        last.push_str(text);
    } else {
        segments.push(Segment::Text(text.to_owned()));
    }
}

fn parse_segments(line: &str) -> Option<Vec<Segment>> {
    let mut segments = Vec::new();
    let mut remaining = line;
    let mut found_chord = false;
    while let Some(open) = remaining.find('[') {
        push_text(&mut segments, &remaining[..open]);
        let after = &remaining[open + 1..];
        let Some(close) = after.find(']') else {
            push_text(&mut segments, &remaining[open..]);
            remaining = "";
            break;
        };
        let inside = &after[..close];
        if crate::chord(inside, 0, false).is_some() {
            segments.push(Segment::Chord(inside.to_owned()));
            found_chord = true;
        } else {
            push_text(&mut segments, &remaining[open..open + close + 2]);
        }
        remaining = &after[close + 1..];
    }
    push_text(&mut segments, remaining);
    found_chord.then_some(segments)
}

impl Song {
    pub fn parse(input: &str) -> Self {
        let mut lines = Vec::new();
        for raw in input.split_inclusive('\n') {
            let (body, ending) = split_ending(raw);
            let ending = ending.to_owned();
            if let Some((kind, value)) = parse_directive(body) {
                lines.push(Line::Directive { kind, value, ending });
            } else if let Some(segments) = parse_segments(body) {
                lines.push(Line::Lyrics { segments, ending });
            } else {
                lines.push(Line::Plain { text: body.to_owned(), ending });
            }
        }
        Self { lines }
    }

    pub fn transpose(&self, semitones: i32, flats: bool) -> Self {
        let lines = self.lines.iter().map(|line| match line {
            Line::Directive { kind, value, ending } => {
                let value = if *kind == DirectiveKind::Key {
                    music::transpose_key(value, semitones, flats).unwrap_or_else(|| value.clone())
                } else { value.clone() };
                Line::Directive { kind: kind.clone(), value, ending: ending.clone() }
            }
            Line::Lyrics { segments, ending } => {
                let segments = segments.iter().map(|segment| match segment {
                    Segment::Text(text) => Segment::Text(text.clone()),
                    Segment::Chord(chord) => Segment::Chord(
                        crate::chord(chord, semitones, flats).unwrap_or_else(|| chord.clone())),
                }).collect();
                Line::Lyrics { segments, ending: ending.clone() }
            }
            Line::Plain { text, ending } => {
                Line::Plain { text: crate::line(text, semitones, flats), ending: ending.clone() }
            }
        }).collect();
        Self { lines }
    }

    pub fn chordpro(&self) -> String {
        let mut output = String::new();
        for line in &self.lines {
            match line {
                Line::Directive { kind, value, .. } => {
                    output.push('{');
                    output.push_str(kind.canonical());
                    if !value.is_empty() {
                        output.push_str(": ");
                        output.push_str(value);
                    }
                    output.push('}');
                }
                Line::Lyrics { segments, .. } => {
                    for segment in segments {
                        match segment {
                            Segment::Text(text) => output.push_str(text),
                            Segment::Chord(chord) => {
                                output.push('[');
                                output.push_str(chord);
                                output.push(']');
                            }
                        }
                    }
                }
                Line::Plain { text, .. } => output.push_str(text),
            }
            output.push_str(line.ending());
        }
        output
    }

    pub fn title(&self) -> Option<&str> {
        self.first_value(DirectiveKind::Title)
    }

    pub fn artist(&self) -> Option<&str> {
        self.first_value(DirectiveKind::Artist)
    }

    pub fn key(&self) -> Option<&str> {
        self.first_value(DirectiveKind::Key)
    }

    fn first_value(&self, wanted: DirectiveKind) -> Option<&str> {
        self.lines.iter().find_map(|line| match line {
            Line::Directive { kind, value, .. } if *kind == wanted => Some(value.as_str()),
            _ => None,
        })
    }

    pub fn chords(&self) -> Vec<String> {
        let mut chords = Vec::new();
        for line in &self.lines {
            match line {
                Line::Lyrics { segments, .. } => {
                    for segment in segments {
                        if let Segment::Chord(chord) = segment { chords.push(chord.clone()); }
                    }
                }
                Line::Plain { text, .. } => {
                    let tokens: Vec<_> = text.split_whitespace().collect();
                    if !tokens.is_empty() && tokens.iter().all(|token| {
                        *token == "|" || crate::chord(token, 0, false).is_some()
                    }) {
                        chords.extend(tokens.iter().filter(|token| **token != "|").map(|s| s.to_string()));
                    }
                }
                Line::Directive { .. } => {}
            }
        }
        chords
    }

    pub fn inventory(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();
        for chord in self.chords() {
            *counts.entry(chord).or_insert(0) += 1;
        }
        counts
    }

    pub fn validate(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        let mut title_count = 0;
        let mut chorus_open = false;
        let mut verse_open = false;
        for (index, line) in self.lines.iter().enumerate() {
            let line_no = index + 1;
            if let Line::Directive { kind, value, .. } = line {
                match kind {
                    DirectiveKind::Title => {
                        title_count += 1;
                        if value.is_empty() { warnings.push(format!("Linha {line_no}: título vazio")); }
                    }
                    DirectiveKind::Key => {
                        if music::parse_key(value).is_none() {
                            warnings.push(format!("Linha {line_no}: tom inválido: {value}"));
                        }
                    }
                    DirectiveKind::Capo => {
                        match value.parse::<u8>() {
                            Ok(fret) if fret <= 12 => {}
                            _ => warnings.push(format!("Linha {line_no}: capotraste fora de 0..12")),
                        }
                    }
                    DirectiveKind::StartChorus => {
                        if chorus_open { warnings.push(format!("Linha {line_no}: refrão já aberto")); }
                        chorus_open = true;
                    }
                    DirectiveKind::EndChorus => {
                        if !chorus_open { warnings.push(format!("Linha {line_no}: fim de refrão sem início")); }
                        chorus_open = false;
                    }
                    DirectiveKind::StartVerse => {
                        if verse_open { warnings.push(format!("Linha {line_no}: verso já aberto")); }
                        verse_open = true;
                    }
                    DirectiveKind::EndVerse => {
                        if !verse_open { warnings.push(format!("Linha {line_no}: fim de verso sem início")); }
                        verse_open = false;
                    }
                    _ => {}
                }
            }
        }
        if title_count > 1 { warnings.push("Mais de um título definido".to_owned()); }
        if chorus_open { warnings.push("Refrão sem fechamento".to_owned()); }
        if verse_open { warnings.push("Verso sem fechamento".to_owned()); }
        warnings
    }

    pub fn report(&self) -> String {
        let mut result = String::from("Relatório de cifras\n=================\n");
        if let Some(title) = self.title() { result.push_str(&format!("Título: {title}\n")); }
        if let Some(artist) = self.artist() { result.push_str(&format!("Artista: {artist}\n")); }
        if let Some(key) = self.key() { result.push_str(&format!("Tom: {key}\n")); }
        result.push_str(&format!("Linhas: {}\n", self.lines.len()));
        let chords = self.chords();
        result.push_str(&format!("Ocorrências de acordes: {}\n", chords.len()));
        result.push_str(&format!("Acordes distintos: {}\n", self.inventory().len()));
        let parsed: Vec<ChordSymbol> = chords.iter().filter_map(|c| music::parse_chord(c)).collect();
        if !parsed.is_empty() {
            result.push('\n');
            result.push_str(&music::analyze_progression(&chords, self.key()));
        }
        let warnings = self.validate();
        if warnings.is_empty() {
            result.push_str("\nValidação: sem alertas\n");
        } else {
            result.push_str("\nAlertas:\n");
            for warning in warnings { result.push_str(&format!("- {warning}\n")); }
        }
        result
    }

    pub fn html(&self) -> String {
        let title = self.title().unwrap_or("Cifras");
        let mut output = String::from("<!doctype html>\n<html lang=\"pt-BR\">\n<head>\n");
        output.push_str("<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n");
        output.push_str("<title>");
        output.push_str(&escape_html(title));
        output.push_str("</title>\n");
        output.push_str("<style>body{font:1rem/1.6 system-ui,sans-serif;max-width:60rem;margin:2rem auto;padding:0 1rem;color:#202124;background:#fafafa}main{white-space:pre-wrap}section{margin:1rem 0}div.line{min-height:1.5rem}span.chord{color:#165d8a;font-weight:700}header{border-bottom:1px solid #bbb;margin-bottom:1rem}small{color:#555}</style>\n");
        output.push_str("</head><body><header><h1>");
        output.push_str(&escape_html(title));
        output.push_str("</h1>");
        if let Some(artist) = self.artist() {
            output.push_str("<p>Artista: ");
            output.push_str(&escape_html(artist));
            output.push_str("</p>");
        }
        if let Some(key) = self.key() {
            output.push_str("<p>Tom: ");
            output.push_str(&escape_html(key));
            output.push_str("</p>");
        }
        output.push_str("</header><main>\n");
        for line in &self.lines {
            match line {
                Line::Directive { kind, value, .. } => match kind {
                    DirectiveKind::StartChorus => output.push_str("<section class=\"chorus\" aria-label=\"Refrão\">\n"),
                    DirectiveKind::EndChorus => output.push_str("</section>\n"),
                    DirectiveKind::StartVerse => output.push_str("<section class=\"verse\" aria-label=\"Verso\">\n"),
                    DirectiveKind::EndVerse => output.push_str("</section>\n"),
                    DirectiveKind::Comment => {
                        output.push_str("<p><small>");
                        output.push_str(&escape_html(value));
                        output.push_str("</small></p>\n");
                    }
                    _ => {}
                },
                Line::Lyrics { segments, .. } => {
                    output.push_str("<div class=\"line\">");
                    for segment in segments {
                        match segment {
                            Segment::Text(text) => output.push_str(&escape_html(text)),
                            Segment::Chord(chord) => {
                                output.push_str("<span class=\"chord\" aria-label=\"Acorde ");
                                output.push_str(&escape_html(chord));
                                output.push_str("\">[");
                                output.push_str(&escape_html(chord));
                                output.push_str("]</span>");
                            }
                        }
                    }
                    output.push_str("</div>\n");
                }
                Line::Plain { text, .. } => {
                    output.push_str("<div class=\"line\">");
                    output.push_str(&escape_html(text));
                    output.push_str("</div>\n");
                }
            }
        }
        output.push_str("</main></body></html>\n");
        output
    }
}

pub fn escape_html(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            _ => result.push(character),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn song(text: &str) -> Song { Song::parse(text) }

    #[test] fn empty_document_is_empty() {
        assert!(song("").lines.is_empty());
        assert_eq!(song("").chordpro(), "");
        assert!(song("").chords().is_empty());
    }
    #[test] fn preserve_utf8_and_crlf() {
        let text = "{title: Louvor à graça}\r\n[Am]Jesus é fiel\r\n";
        assert_eq!(song(text).chordpro(), text);
    }
    #[test] fn canonicalize_short_directives() {
        assert_eq!(song("{t: Canção}\n{a: Autor}\n{k: Am}\n").chordpro(),
            "{title: Canção}\n{artist: Autor}\n{key: Am}\n");
    }
    #[test] fn preserve_unknown_directives() {
        assert_eq!(song("{custom: value}\n").chordpro(), "{custom: value}\n");
    }
    #[test] fn preserve_unknown_brackets() {
        assert_eq!(song("[Refrão] [Am]cantar\n").chordpro(), "[Refrão] [Am]cantar\n");
    }
    #[test] fn transpose_inline_and_key() {
        let actual = song("{key: Am}\n[Am]Vou [G/B]cantar\n").transpose(2, false);
        assert_eq!(actual.chordpro(), "{key: Bm}\n[Bm]Vou [A/C#]cantar\n");
    }
    #[test] fn transpose_preserves_lyrics_and_line_endings() {
        let actual = song("[C]Vida\r\n[F]Graça\r\n").transpose(1, true);
        assert_eq!(actual.chordpro(), "[Db]Vida\r\n[Gb]Graça\r\n");
    }
    #[test] fn transpose_plain_chord_lines() {
        assert_eq!(song("C    G/B\ntexto aqui\n").transpose(2, false).chordpro(),
            "D    A/C#\ntexto aqui\n");
    }
    #[test] fn transpose_does_not_touch_unknown_directive() {
        assert_eq!(song("{custom: C}\n").transpose(2, false).chordpro(), "{custom: C}\n");
    }
    #[test] fn inventory_counts_inline_and_chord_lines() {
        let counts = song("[C]Oi [G]oi\nC G\n").inventory();
        assert_eq!(counts.get("C"), Some(&2));
        assert_eq!(counts.get("G"), Some(&2));
    }
    #[test] fn reject_lyric_words_as_chords() {
        assert!(song("Cantarei ao Senhor\n").chords().is_empty());
    }
    #[test] fn metadata_accessors() {
        let document = song("{title: Luz}\n{artist: Autor}\n{key: C}\n");
        assert_eq!(document.title(), Some("Luz"));
        assert_eq!(document.artist(), Some("Autor"));
        assert_eq!(document.key(), Some("C"));
    }
    #[test] fn valid_directives_have_no_warnings() {
        let text = "{title: Luz}\n{key: G}\n{capo: 3}\n{soc}\n[C]Oi\n{eoc}\n";
        assert!(song(text).validate().is_empty());
    }
    #[test] fn invalid_key_and_capo_are_reported() {
        let warnings = song("{key: Hm}\n{capo: 30}\n").validate();
        assert_eq!(warnings.len(), 2);
        assert!(warnings[0].contains("tom inválido"));
        assert!(warnings[1].contains("capotraste"));
    }
    #[test] fn chorus_closing_without_opening_is_reported() {
        assert!(song("{eoc}\n").validate()[0].contains("sem início"));
    }
    #[test] fn chorus_open_without_closing_is_reported() {
        assert!(song("{soc}\n").validate()[0].contains("sem fechamento"));
    }
    #[test] fn repeated_chorus_start_is_reported() {
        assert!(song("{soc}\n{soc}\n{eoc}\n").validate()[0].contains("já aberto"));
    }
    #[test] fn verse_mismatch_is_reported() {
        assert_eq!(song("{eov}\n{sov}\n").validate().len(), 2);
    }
    #[test] fn duplicate_title_is_reported() {
        assert!(song("{title: A}\n{title: B}\n").validate()[0].contains("Mais de um"));
    }
    #[test] fn empty_title_is_reported() {
        assert!(song("{title: }\n").validate()[0].contains("título vazio"));
    }
    #[test] fn html_escapes_untrusted_metadata() {
        let html = song("{title: <script>alert(1)</script>}\n").html();
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>"));
    }
    #[test] fn html_escapes_untrusted_lyrics() {
        let html = song("[Am]A & B < C \"D\"\n").html();
        assert!(html.contains("A &amp; B &lt; C &quot;D&quot;"));
        assert!(html.contains("aria-label=\"Acorde Am\""));
    }
    #[test] fn html_renders_chorus_sections() {
        let html = song("{soc}\n[C]Oi\n{eoc}\n").html();
        assert!(html.contains("class=\"chorus\""));
        assert!(html.contains("</section>"));
    }
    #[test] fn report_contains_counts_and_analysis() {
        let report = song("{title: Luz}\n{key: C}\n[C]oi [G]oi [C]oi\n").report();
        assert!(report.contains("Título: Luz"));
        assert!(report.contains("Ocorrências de acordes: 3"));
        assert!(report.contains("Acordes distintos: 2"));
        assert!(report.contains("Graus: 1 5 1"));
    }
    #[test] fn report_includes_validation_warnings() {
        let report = song("{key: inválido}\n").report();
        assert!(report.contains("tom inválido"));
    }
    #[test] fn no_final_newline_is_preserved() {
        assert_eq!(song("[C]Fim").chordpro(), "[C]Fim");
    }
    #[test] fn multiple_chords_adjacent_are_preserved() {
        assert_eq!(song("[C][G][Am]Oi").transpose(-2, false).chordpro(),
            "[A#][F][Gm]Oi");
    }
    #[test] fn html_escape_all_special_characters() {
        assert_eq!(escape_html("&<>\"'"), "&amp;&lt;&gt;&quot;&#39;");
    }
}
