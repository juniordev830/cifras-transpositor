let () =
  assert (Transpose.transpose 3 "Cm7" = Ok "D#m7");
  assert (Transpose.transpose 2 "Bbmaj7" = Ok "Cmaj7");
  assert (Transpose.transpose (-1) "C" = Ok "B");
  assert (Transpose.transpose 1 "Xm" = Error "invalid root");
  print_endline "OCaml chord checks passed"
