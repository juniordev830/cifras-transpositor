# Cifras Transpositor

CLI em **Rust** para mudar o tom de linhas de cifras, incluindo acidentes, extensões e baixo invertido. Exemplo: `C#m7/G#` transposto dois semitons abaixo resulta em `Bm7/F#`.

## Primeira versão

- Transposição positiva e negativa, com nomes em sustenidos ou bemóis.
- Letras preservadas: linhas compostas inteiramente de cifras e barras são transpostas; em linhas com texto, acordes delimitados por colchetes como `[Am]` e `[G/B]` também são transpostos sem alterar a letra.
- Mantém as colunas iniciais quando há espaço; se um acorde cresce, conserva ao menos um espaço antes do próximo.
- Leitura de arquivo ou entrada padrão; o original nunca é regravado.
- Testes de domínio para acidentes enarmônicos, baixos invertidos, finais CRLF, cifras embutidas e cabeçalhos que não são acordes.

## Executar

Instale Rust com Cargo a partir de [rust-lang.org](https://www.rust-lang.org/tools/install). Não há dependências externas.

```sh
cargo run --offline -- 2 examples/cancao.txt
cargo run --offline -- -2 examples/cancao.txt --bemol
printf '[Am]Cantarei [G/B]com alegria\n' | cargo run --offline -- 2
cargo test --offline
```

O resultado vai para a saída padrão. Para salvar, redirecione para **outro arquivo**, evitando substituir a entrada.

## Estrutura e validação

`src/main.rs` contém o parser, a transformação pura e o adaptador de CLI. `examples/` traz conteúdo fictício/autoral. O workflow em `.github/workflows/ci.yml` compila, testa e executa o exemplo em Rust 1.85.

O ambiente de preparação não possui compilador Rust; a validação executável está a cargo do GitHub Actions. Consulte a execução do commit para conhecer seu resultado.

## Limites e próximos passos

Esta versão aceita cifras latinas de A a G, acidentes simples e os sufixos descritos no parser. Acordes **entre colchetes** são transpostos dentro de linhas de letra, mantendo palavras e marcações inválidas, como `[Refrão]`, intactas. Em linhas mistas, somente os acordes entre colchetes são transpostos. Ainda não interpreta tablaturas, notação em graus ou cifras embutidas sem colchetes. Uma linha de letra formada somente por nomes de acordes também seria interpretada como cifras. Alinhamento usa colunas ASCII das cifras.

`nom`, `clap`, editor gráfico e importação de formatos são possibilidades futuras, ainda não instaladas. A arquitetura proposta anterior está em `ARQUITETURA.md` e descreve a evolução planejada.

Licença: ainda não definida.
