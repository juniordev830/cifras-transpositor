# Transposicao funcional de cifras

Duas implementacoes independentes que transpoem a raiz do acorde e preservam seu sufixo. A saida canonica usa sustenidos.

- Haskell: runghc -ilabs/haskell labs/haskell/TestTranspose.hs
- OCaml: ocamlc -o /tmp/test_transpose labs/ocaml/transpose.ml labs/ocaml/test_transpose.ml && /tmp/test_transpose

Sao experimentos da representacao harmonica, sem alterar o transpositor Rust atual. Nao realizam enarmonia contextual.
