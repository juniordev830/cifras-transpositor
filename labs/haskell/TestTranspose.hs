import Transpose (transposeChord)

main :: IO ()
main = do
  assertEqual (Right "D#m7") (transposeChord 3 "Cm7")
  assertEqual (Right "Cmaj7") (transposeChord 2 "Bbmaj7")
  assertEqual (Right "B") (transposeChord (-1) "C")
  assertEqual (Left "invalid root") (transposeChord 1 "Xm7")
  assertEqual (Left "empty chord") (transposeChord 1 "")
  putStrLn "Haskell chord checks passed"

assertEqual :: (Eq a, Show a) => a -> a -> IO ()
assertEqual expected actual
  | expected == actual = pure ()
  | otherwise = error ("Expected " ++ show expected ++ " but got " ++ show actual)
