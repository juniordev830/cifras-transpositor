# Labirinto Procedural

Jogo com labirintos gerados a partir de sementes reproduzíveis.

## Implementado na primeira versão

**Rust — núcleo executável de domínio.** Gera labirintos reproduzíveis com semente, garante conectividade dos corredores e oferece um modo jogável no terminal. Testa vinte sementes e os limites das dimensões.

## Como executar

Instale Rust 1.85 com Cargo. Dependências adicionais constam em Cargo.toml. Os comandos abaixo partem da raiz do repositório.

```sh
cargo run -- 15 42 --jogar
```

## Testes

```sh
cargo test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `src/main.rs`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Interface textual, sem gráficos de macroquad. Tamanhos ímpares de 5 a 81; o gerador é determinístico e não criptográfico. Dificuldade inicial corresponde ao tamanho. macroquad e rand são próximos passos.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Gerar labirintos com semente — ampliar o núcleo atual até o fluxo completo.
- [ ] Garantir caminho entre início e fim — ampliar o núcleo atual até o fluxo completo.
- [ ] Adicionar movimento e diferentes dificuldades — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
