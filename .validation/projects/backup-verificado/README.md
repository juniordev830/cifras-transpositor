# Backup Verificado

Cópias incrementais de arquivos com verificação de integridade.

## Implementado na primeira versão

**Rust — núcleo executável de domínio.** Mostra uma prévia de cópia, copia arquivos novos mediante --aplicar, compara SHA-256 antes e depois e recusa sobrescrever conteúdo diferente.

## Como executar

Instale Rust 1.85 com Cargo. Dependências adicionais constam em Cargo.toml. Os comandos abaixo partem da raiz do repositório.

```sh
cargo run -- examples/origem examples/destino
# Para executar a cópia, acrescente --aplicar ao comando.
```

## Testes

```sh
cargo test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `src/main.rs e examples/`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Requer Linux com sha256sum (GNU coreutils). Processa somente arquivos regulares na pasta imediata, sem subpastas ou links. Arquivos iguais são ignorados; conflitos encerram a execução, preservando cópias já concluídas. Não preserva permissões especiais, não é transacional para o lote e não se destina a árvores alteradas por processos hostis. Restauração, BLAKE3 e clap ficam para próximas versões.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Simular plano de cópia — ampliar o núcleo atual até o fluxo completo.
- [ ] Copiar arquivos para destino escolhido — ampliar o núcleo atual até o fluxo completo.
- [ ] Verificar hashes e oferecer restauração — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
