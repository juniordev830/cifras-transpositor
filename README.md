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


## Leitura de ChordPro e análise musical

O CLI mantém o modo original de transposição e acrescenta três formatos de saída
sem dependências externas. As cifras entre colchetes podem aparecer no meio da
letra; o texto original e os finais de linha CRLF são preservados no modo ChordPro.

| Formato | Uso |
| --- | --- |
| texto | Saída compatível com o comportamento original |
| chordpro | Reconhece acordes entre colchetes e normaliza diretivas conhecidas |
| html | Gera documento HTML estático com metadados e cifras acessíveis |
| relatorio | Conta acordes, estima tonalidade, sugere capotraste e mostra graus |

Exemplos com conteúdo autoral ou fictício:

~~~sh
printf '{title: Canção}\n{key: Am}\n[Am]Canto [G/B]com alegria\n' | cargo run --offline -- 2 - --formato=chordpro
printf '{title: Canção}\n[Am]Canto [G]hoje\n' | cargo run --offline -- 0 - --formato=html > cancao.html
printf '{key: C}\n[C]Luz [G]paz [Am]vida\n' | cargo run --offline -- 0 - --formato=relatorio
cargo test --offline
~~~

Diretivas reconhecidas: title/t, subtitle/st, artist/a, key/k, capo,
comment/c, start_of_chorus/soc, end_of_chorus/eoc, start_of_verse/sov e
end_of_verse/eov. Diretivas desconhecidas permanecem intactas em ChordPro.
A validação avisa sobre tom inválido, capo fora de 0 a 12, títulos duplicados
e seções sem fechamento.

A análise harmônica é **heurística**: a tonalidade estimada não substitui a
análise de um músico. O ranking de capotraste considera um conjunto simples
de formas comuns de violão; não garante facilidade de execução. A cifra
Nashville é calculada por graus e acidentes cromáticos. O HTML aplica escape
de caracteres especiais à letra e aos metadados, sem executar scripts.

**Segurança e compatibilidade:** o comando lê o arquivo ou stdin e escreve
somente em stdout. Redirecione para um arquivo de saída diferente da entrada.
O modo padrão continua aceitando as mesmas opções da primeira versão.

## Sequência harmônica simplificada

O relatório musical (`--formato=relatorio`) apresenta agora **Sequência simplificada**, removendo apenas as repetições consecutivas de um mesmo acorde. A sequência original e a saída da transposição permanecem intactas; cifras com baixo invertido, como `C` e `C/E`, continuam distintas. A representação normaliza notas enarmônicas no relatório, e os testes verificam repetições, inversões e notas com bemóis. É uma visão auxiliar para ensaio e arranjo, não uma análise automática do ritmo.
