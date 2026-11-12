# Cifras Transpositor

CLI em **Rust** para mudar o tom de linhas de cifras, incluindo acidentes, extensões e baixo invertido. Exemplo: `C#m7/G#` transposto dois semitons abaixo resulta em `Bm7/F#`.

## Primeira versão

- Transposição positiva e negativa, com nomes em sustenidos ou bemóis.
- Letras preservadas: somente linhas compostas inteiramente de cifras e barras são alteradas.
- Mantém as colunas iniciais quando há espaço; se um acorde cresce, conserva ao menos um espaço antes do próximo.
- Leitura de arquivo ou entrada padrão; o original nunca é regravado.
- Cinco testes de domínio, incluindo acidentes enarmônicos, baixo e finais CRLF.

## Executar

Instale Rust com Cargo a partir de [rust-lang.org](https://www.rust-lang.org/tools/install). Não há dependências externas.

```sh
cargo run --offline -- 2 examples/cancao.txt
cargo run --offline -- -2 examples/cancao.txt --bemol
cargo test --offline
```

O resultado vai para a saída padrão. Para salvar, redirecione para **outro arquivo**, evitando substituir a entrada.

## Estrutura e validação

`src/main.rs` contém o parser, a transformação pura e o adaptador de CLI. `examples/` traz conteúdo fictício/autoral. O workflow em `.github/workflows/ci.yml` compila, testa e executa o exemplo em Rust 1.85.

O ambiente de preparação não possui compilador Rust; a validação executável está a cargo do GitHub Actions. Consulte a execução do commit para conhecer seu resultado.

## Limites e próximos passos

Esta versão aceita cifras latinas de A a G, acidentes simples e os sufixos descritos no parser. Não interpreta cifras embutidas na letra, tablaturas, acordes entre colchetes ou notação em graus. Uma linha de letra formada somente por nomes de acordes também seria interpretada como cifras. Alinhamento usa colunas ASCII das cifras.

`nom`, `clap`, editor gráfico e importação de formatos são possibilidades futuras, ainda não instaladas. A arquitetura proposta anterior está em `ARQUITETURA.md` e descreve a evolução planejada.

Licença: ainda não definida.

## Roteiro de qualidade

Consulte o [guia de validação e manutenção](docs/GUIA_DE_VALIDACAO.md) para reproduzir testes, entender as limitações e revisar mudanças com segurança.
