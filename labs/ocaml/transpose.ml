let notes = [|
  "C"; "C#"; "D"; "D#"; "E"; "F"; "F#";
  "G"; "G#"; "A"; "A#"; "B"
|]

let pitch = function
  | 'C' -> Some 0 | 'D' -> Some 2 | 'E' -> Some 4
  | 'F' -> Some 5 | 'G' -> Some 7 | 'A' -> Some 9
  | 'B' -> Some 11 | _ -> None

let transpose semitones chord =
  if String.length chord = 0 then Error "empty chord"
  else
    match pitch (Char.uppercase_ascii chord.[0]) with
    | None -> Error "invalid root"
    | Some root ->
        let alteration, suffix_start =
          if String.length chord > 1 then
            match chord.[1] with
            | '#' -> (1, 2) | 'b' -> (-1, 2)
            | _ -> (0, 1)
          else (0, 1)
        in
        let index = ((root + alteration + semitones) mod 12 + 12) mod 12 in
        Ok (notes.(index) ^ String.sub chord suffix_start (String.length chord - suffix_start))
