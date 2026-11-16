module Transpose (transposeChord) where

import Data.Char (toUpper)

notes :: [String]
notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]

pitch :: Char -> Int
pitch c = case toUpper c of
  'C' -> 0
  'D' -> 2
  'E' -> 4
  'F' -> 5
  'G' -> 7
  'A' -> 9
  'B' -> 11
  _   -> -1

-- Transpose a chord root, retaining qualities such as "m7" or "sus4".
-- Output roots use sharps for consistent, deterministic notation.
transposeChord :: Int -> String -> Either String String
transposeChord _ [] = Left "empty chord"
transposeChord semitones (root:rest)
  | pitch root < 0 = Left "invalid root"
  | otherwise =
      let (alteration, suffix) = case rest of
            ('#':xs) -> (1, xs)
            ('b':xs) -> (-1, xs)
            xs       -> (0, xs)
          index = mod (pitch root + alteration + semitones) 12
      in Right (notes !! index ++ suffix)
