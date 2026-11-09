use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Suspended,
    Power,
}

impl Quality {
    pub fn label(self) -> &'static str {
        match self {
            Self::Major => "maior",
            Self::Minor => "menor",
            Self::Diminished => "diminuto",
            Self::Augmented => "aumentado",
            Self::Suspended => "suspenso",
            Self::Power => "power chord",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChordSymbol {
    pub root: u8,
    pub quality: Quality,
    pub suffix: String,
    pub bass: Option<u8>,
}

pub const SHARPS: [&str; 12] =
    ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
pub const FLATS: [&str; 12] =
    ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];
const MAJOR_SCALE: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
const MINOR_SCALE: [u8; 7] = [0, 2, 3, 5, 7, 8, 10];

pub fn pitch(input: &str) -> Option<(u8, usize)> {
    let root = match input.as_bytes().first()? {
        b'C' => 0i32, b'D' => 2, b'E' => 4, b'F' => 5,
        b'G' => 7, b'A' => 9, b'B' => 11, _ => return None,
    };
    let (alteration, len) = match input.as_bytes().get(1) {
        Some(b'#') => (1, 2),
        Some(b'b') => (-1, 2),
        _ => (0, 1),
    };
    Some(((root + alteration).rem_euclid(12) as u8, len))
}

pub fn parse_chord(value: &str) -> Option<ChordSymbol> {
    // Reuse the existing grammar; this avoids interpreting lyric words as chords.
    crate::chord(value, 0, false)?;
    let (root, used) = pitch(value)?;
    let mut pieces = value[used..].split('/');
    let suffix = pieces.next()?.to_owned();
    let bass = pieces.next().map(pitch).transpose_option()?;
    if pieces.next().is_some() { return None; }
    let quality = if suffix.starts_with("dim") || suffix.starts_with('o') {
        Quality::Diminished
    } else if suffix.starts_with("aug") || suffix.starts_with('+') {
        Quality::Augmented
    } else if suffix.starts_with("sus") {
        Quality::Suspended
    } else if suffix == "5" {
        Quality::Power
    } else if suffix.starts_with("min") || (suffix.starts_with('m') && !suffix.starts_with("maj")) {
        Quality::Minor
    } else {
        Quality::Major
    };
    Some(ChordSymbol { root, quality, suffix, bass })
}

trait OptionPitch {
    fn transpose_option(self) -> Option<Option<u8>>;
}
impl OptionPitch for Option<Option<(u8, usize)>> {
    fn transpose_option(self) -> Option<Option<u8>> {
        match self {
            None => Some(None),
            Some(Some((pitch, _))) => Some(Some(pitch)),
            Some(None) => None,
        }
    }
}

pub fn name(pitch: u8, flats: bool) -> &'static str {
    if flats { FLATS[(pitch % 12) as usize] }
    else { SHARPS[(pitch % 12) as usize] }
}

pub fn chord_pitches(chord: &ChordSymbol) -> Vec<u8> {
    let offsets: &[u8] = match chord.quality {
        Quality::Major => &[0, 4, 7],
        Quality::Minor => &[0, 3, 7],
        Quality::Diminished => &[0, 3, 6],
        Quality::Augmented => &[0, 4, 8],
        Quality::Suspended => {
            if chord.suffix.starts_with("sus2") { &[0, 2, 7] }
            else { &[0, 5, 7] }
        }
        Quality::Power => &[0, 7],
    };
    offsets.iter().map(|n| (chord.root + n) % 12).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode { Major, Minor }
impl Mode {
    pub fn label(self) -> &'static str {
        match self { Self::Major => "maior", Self::Minor => "menor" }
    }
    fn scale(self) -> &'static [u8; 7] {
        match self { Self::Major => &MAJOR_SCALE, Self::Minor => &MINOR_SCALE }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyCandidate {
    pub tonic: u8,
    pub mode: Mode,
    pub score: i32,
}

pub fn key_candidates(chords: &[ChordSymbol]) -> Vec<KeyCandidate> {
    if chords.is_empty() { return Vec::new(); }
    let mut candidates = Vec::new();
    for tonic in 0..12u8 {
        for mode in [Mode::Major, Mode::Minor] {
            let scale = mode.scale();
            let mut score = 0;
            for (index, chord) in chords.iter().enumerate() {
                let degree = (chord.root + 12 - tonic) % 12;
                let in_scale = scale.contains(&degree);
                score += if in_scale { 4 } else { -4 };
                let expected_minor = match mode {
                    Mode::Major => [2, 4, 9].contains(&degree),
                    Mode::Minor => [0, 5, 7].contains(&degree),
                };
                if chord.quality == Quality::Minor {
                    score += if expected_minor { 2 } else { -1 };
                } else if chord.quality == Quality::Major {
                    score += if expected_minor { -1 } else { 1 };
                }
                if degree == 0 { score += if index == 0 { 3 } else { 1 }; }
                if degree == 7 { score += 1; }
                if index + 1 == chords.len() && degree == 0 { score += 3; }
            }
            candidates.push(KeyCandidate { tonic, mode, score });
        }
    }
    candidates.sort_by(|a, b| b.score.cmp(&a.score)
        .then_with(|| a.tonic.cmp(&b.tonic))
        .then_with(|| (a.mode == Mode::Minor).cmp(&(b.mode == Mode::Minor))));
    candidates
}

pub fn detect_key(chords: &[ChordSymbol]) -> Option<KeyCandidate> {
    key_candidates(chords).into_iter().next()
}

pub fn parse_key(value: &str) -> Option<(u8, Mode)> {
    let trimmed = value.trim();
    let (tonic, len) = pitch(trimmed)?;
    let suffix = trimmed[len..].trim();
    let mode = match suffix.to_ascii_lowercase().as_str() {
        "" | "major" | "maior" | "maj" => Mode::Major,
        "m" | "minor" | "menor" | "min" => Mode::Minor,
        _ => return None,
    };
    Some((tonic, mode))
}

pub fn transpose_key(key: &str, semitones: i32, flats: bool) -> Option<String> {
    let (tonic, mode) = parse_key(key)?;
    let moved = (i32::from(tonic) + semitones.rem_euclid(12)).rem_euclid(12) as u8;
    Some(format!("{}{}", name(moved, flats), if mode == Mode::Minor { "m" } else { "" }))
}

pub fn key_signature(tonic: u8, mode: Mode) -> i8 {
    // Signed circle-of-fifths signature; prefer fewer accidentals.
    // Relative major lies three semitones above a minor tonic.
    let major = if mode == Mode::Minor { (tonic + 3) % 12 } else { tonic };
    match major {
        0 => 0, 7 => 1, 2 => 2, 9 => 3, 4 => 4, 11 => 5,
        6 => 6, 1 => -5, 8 => -4, 3 => -3, 10 => -2, 5 => -1,
        _ => unreachable!(),
    }
}

pub fn nashville(chord: &ChordSymbol, tonic: u8, mode: Mode) -> String {
    // Nashville numbers always use the parallel major scale as the reference,
    // including minor keys (b3, b6 and b7 rather than 3, 6 and 7).
    const DEGREES: [&str; 12] =
        ["1", "b2", "2", "b3", "3", "4", "#4", "5", "b6", "6", "b7", "7"];
    let interval = (chord.root + 12 - tonic) % 12;
    let degree = DEGREES[interval as usize];
    let suffix = match chord.quality {
        Quality::Minor if chord.suffix.is_empty() => "m",
        _ => &chord.suffix,
    };
    let mut result = format!("{degree}{suffix}");
    if let Some(bass) = chord.bass {
        let bass_symbol = ChordSymbol {
            root: bass, quality: Quality::Major, suffix: String::new(), bass: None,
        };
        result.push('/');
        result.push_str(&nashville(&bass_symbol, tonic, mode));
    }
    result
}

pub fn chord_inventory(chords: &[ChordSymbol], flats: bool) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for chord in chords {
        let root = name(chord.root, flats);
        let bass = chord.bass.map(|p| format!("/{}", name(p, flats))).unwrap_or_default();
        *counts.entry(format!("{root}{}{bass}", chord.suffix)).or_insert(0) += 1;
    }
    counts
}

/// A compact harmonic outline for arranging a song. Adjacent duplicates are
/// suppressed, but the original chord progression is never modified.
pub fn compact_progression(chords: &[ChordSymbol], flats: bool) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for chord in chords {
        let bass = chord.bass
            .map(|p| format!("/{}", name(p, flats)))
            .unwrap_or_default();
        let label = format!("{}{}{}", name(chord.root, flats), chord.suffix, bass);
        if result.last() != Some(&label) {
            result.push(label);
        }
    }
    result
}

pub fn suggest_capo(chords: &[ChordSymbol]) -> Vec<(u8, usize)> {
    // Rank playable chord shapes. A capo shifts the written chord downward
    // while retaining the same concert pitch.
    const EASY: &[&str] = &["C", "D", "E", "G", "A", "Am", "Dm", "Em", "F", "A7", "D7", "E7", "G7"];
    let mut results = Vec::new();
    for fret in 0..=11u8 {
        let mut playable = 0;
        for chord in chords {
            let shape = (i32::from(chord.root) - i32::from(fret)).rem_euclid(12) as u8;
            let label = format!("{}{}", name(shape, false), chord.suffix);
            if EASY.contains(&label.as_str()) { playable += 1; }
        }
        results.push((fret, playable));
    }
    results.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    results
}

pub fn analyze_progression(tokens: &[String], key: Option<&str>) -> String {
    let chords: Vec<_> = tokens.iter().filter_map(|t| parse_chord(t)).collect();
    if chords.is_empty() { return "Nenhuma cifra reconhecida.\n".to_owned(); }
    let chosen = key.and_then(parse_key).map(|(tonic, mode)| KeyCandidate {
        tonic, mode, score: 0,
    }).or_else(|| detect_key(&chords));
    let mut output = format!("Acordes encontrados: {}\n", chords.len());
    if let Some(candidate) = chosen {
        output.push_str(&format!("Tom {}: {} {}\n",
            if key.is_some() { "informado" } else { "estimado" },
            name(candidate.tonic, key_signature(candidate.tonic, candidate.mode) < 0),
            candidate.mode.label()));
        output.push_str("Graus: ");
        output.push_str(&chords.iter().map(|chord| nashville(chord, candidate.tonic, candidate.mode))
            .collect::<Vec<_>>().join(" "));
        output.push('\n');
    }
    output.push_str("Qualidades:\n");
    let mut qualities = BTreeMap::new();
    for chord in &chords {
        *qualities.entry(chord.quality.label()).or_insert(0usize) += 1;
    }
    for (quality, count) in qualities {
        output.push_str(&format!("  {quality}: {count}\n"));
    }
    output.push_str("Frequências:\n");
    for (chord, count) in chord_inventory(&chords, false) {
        output.push_str(&format!("  {chord}: {count}\n"));
    }
    output.push_str("Sequência simplificada: ");
    output.push_str(&compact_progression(&chords, false).join(" -> "));
    output.push('\n');
    if let Some((fret, playable)) = suggest_capo(&chords).first() {
        output.push_str(&format!("Capotraste sugerido: casa {fret} ({playable}/{} formas simples)\n", chords.len()));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    fn c(value: &str) -> ChordSymbol { parse_chord(value).unwrap() }

    #[test] fn all_twelve_notes_have_unique_pitch() {
        for (index, note) in SHARPS.iter().enumerate() {
            assert_eq!(pitch(note), Some((index as u8, note.len())));
        }
    }
    #[test] fn flats_and_sharps_are_enharmonic() {
        for index in 0..12 {
            assert_eq!(pitch(FLATS[index]).unwrap().0, pitch(SHARPS[index]).unwrap().0);
        }
    }
    #[test] fn reject_lyrics_and_lowercase() {
        for value in ["Deus", "Cantarei", "a", "Hm", "X7", "C/G/B", ""] {
            assert!(parse_chord(value).is_none(), "{value}");
        }
    }
    #[test] fn parse_inversions() {
        assert_eq!(c("C#m7/G#").root, 1);
        assert_eq!(c("C#m7/G#").bass, Some(8));
        assert_eq!(c("C#m7/G#").quality, Quality::Minor);
    }
    #[test] fn recognize_quality() {
        assert_eq!(c("Am").quality, Quality::Minor);
        assert_eq!(c("Cmaj7").quality, Quality::Major);
        assert_eq!(c("Bdim").quality, Quality::Diminished);
        assert_eq!(c("Daug").quality, Quality::Augmented);
        assert_eq!(c("Gsus4").quality, Quality::Suspended);
        assert_eq!(c("A5").quality, Quality::Power);
    }
    #[test] fn triads_have_expected_pitches() {
        assert_eq!(chord_pitches(&c("C")), vec![0, 4, 7]);
        assert_eq!(chord_pitches(&c("Am")), vec![9, 0, 4]);
        assert_eq!(chord_pitches(&c("Bdim")), vec![11, 2, 5]);
        assert_eq!(chord_pitches(&c("Gsus2")), vec![7, 9, 2]);
    }
    #[test] fn key_parser_accepts_portuguese_and_english() {
        assert_eq!(parse_key("Am"), Some((9, Mode::Minor)));
        assert_eq!(parse_key("Bb maior"), Some((10, Mode::Major)));
        assert_eq!(parse_key("F# minor"), Some((6, Mode::Minor)));
        assert_eq!(parse_key("G maj"), Some((7, Mode::Major)));
    }
    #[test] fn key_parser_rejects_unknown_modes() {
        assert_eq!(parse_key("Cm jazz"), None);
        assert_eq!(parse_key("H"), None);
        assert_eq!(parse_key(""), None);
    }
    #[test] fn transpose_key_handles_negative_and_large_offsets() {
        assert_eq!(transpose_key("Am", -2, false).as_deref(), Some("Gm"));
        assert_eq!(transpose_key("F", 1, true).as_deref(), Some("Gb"));
        assert_eq!(transpose_key("C", i32::MAX, false).as_deref(), Some("G"));
    }
    #[test] fn signatures_match_circle_of_fifths() {
        assert_eq!(key_signature(0, Mode::Major), 0);
        assert_eq!(key_signature(7, Mode::Major), 1);
        assert_eq!(key_signature(5, Mode::Major), -1);
        assert_eq!(key_signature(9, Mode::Minor), 0);
        assert_eq!(key_signature(2, Mode::Minor), -1);
    }
    #[test] fn nashville_in_c_major() {
        assert_eq!(nashville(&c("C"), 0, Mode::Major), "1");
        assert_eq!(nashville(&c("Dm7"), 0, Mode::Major), "2m7");
        assert_eq!(nashville(&c("G7"), 0, Mode::Major), "57");
        assert_eq!(nashville(&c("Bb"), 0, Mode::Major), "b7");
        assert_eq!(nashville(&c("F#"), 0, Mode::Major), "#4");
    }
    #[test] fn nashville_in_g_major() {
        assert_eq!(nashville(&c("G"), 7, Mode::Major), "1");
        assert_eq!(nashville(&c("C"), 7, Mode::Major), "4");
        assert_eq!(nashville(&c("D/F#"), 7, Mode::Major), "5/7");
    }
    #[test] fn nashville_minor_scale() {
        assert_eq!(nashville(&c("Am"), 9, Mode::Minor), "1m");
        assert_eq!(nashville(&c("Dm"), 9, Mode::Minor), "4m");
        assert_eq!(nashville(&c("E"), 9, Mode::Minor), "5");
        assert_eq!(nashville(&c("C"), 9, Mode::Minor), "b3");
    }
    #[test] fn inventory_is_sorted_and_counts_repeats() {
        let counts = chord_inventory(&[c("G"), c("C"), c("G")], false);
        assert_eq!(counts.get("G"), Some(&2));
        assert_eq!(counts.get("C"), Some(&1));
    }
    #[test] fn no_key_without_chords() {
        assert!(detect_key(&[]).is_none());
        assert!(key_candidates(&[]).is_empty());
    }
    #[test] fn candidates_cover_all_keys() {
        let candidates = key_candidates(&[c("C"), c("F"), c("G")]);
        assert_eq!(candidates.len(), 24);
        assert!(candidates.windows(2).all(|w| w[0].score >= w[1].score));
    }
    #[test] fn capo_ranking_includes_all_frets() {
        let ranks = suggest_capo(&[c("C"), c("G"), c("Am"), c("F")]);
        assert_eq!(ranks.len(), 12);
        assert_eq!(ranks[0].0, 0);
        assert_eq!(ranks[0].1, 4);
    }
    #[test] fn report_is_deterministic() {
        let report = analyze_progression(&["C".into(), "G".into(), "Am".into()], Some("C"));
        assert!(report.contains("Tom informado: C maior"));
        assert!(report.contains("Graus: 1 5 6m"));
        assert!(report.contains("C: 1"));
    }
    #[test] fn report_without_chords() {
        assert_eq!(analyze_progression(&[], None), "Nenhuma cifra reconhecida.\n");
    }
    #[test] fn compact_progression_removes_only_adjacent_repeats() {
        let chords = [c("C"), c("C"), c("G/B"), c("G/B"), c("Am"), c("C")];
        assert_eq!(compact_progression(&chords, false),
            vec!["C", "G/B", "Am", "C"]);
    }
    #[test] fn compact_progression_respects_inversion_and_flats() {
        let chords = [c("C"), c("C/E"), c("C"), c("B#"), c("Db")];
        assert_eq!(compact_progression(&chords, true),
            vec!["C", "C/E", "C", "Db"]);
        assert!(compact_progression(&[], false).is_empty());
    }
    #[test] fn report_contains_the_compact_sequence() {
        let report = analyze_progression(&[
            "C".into(), "C".into(), "G".into(), "G".into(), "Am".into()
        ], Some("C"));
        assert!(report.contains("Sequência simplificada: C -> G -> Am"));
        assert!(report.contains("Acordes encontrados: 5"));
    }

}
