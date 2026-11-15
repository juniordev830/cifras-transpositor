# Desenvolvimento e validação — Cifras Transpositor

## Estado implementado

CLI Rust que transpõe cifras A–G com acidentes simples; não processa tablaturas completas.

## Estrutura relevante

`src/main.rs`, `examples/cancao.txt`, `Cargo.toml` e `ARQUITETURA.md`.

## Como reproduzir verificações

`cargo test --offline`; `cargo run --offline -- -2 examples/cancao.txt --bemol`.

Os exemplos devem usar entradas fictícias. Registrar explicitamente o que foi de fato executado no ambiente e comparar com o CI.

## Regressão prioritária

Verificar equivalência enarmônica, acordes com baixo, entradas inválidas, linhas de letras interpretadas incorretamente e preservação de quebras de linha.

## Próximos limites a resolver

Para publicar como ferramenta musical mais ampla, separar parser e formatação, documentar gramática reconhecida e adicionar testes de regressão de cifras autorais.

## Revisão de contribuição

- [ ] Mudança melhora uma regra, teste ou integração real.
- [ ] Cenários de entrada, saída e erro estão documentados.
- [ ] Nada de dados pessoais, credenciais ou arquivos gerados versionados.
- [ ] PR separado antes de alterar a principal.
