# Log Redactor CLI

Ocultação de padrões configurados de dados sensíveis em logs.

## Implementado na primeira versão

**Rust — núcleo executável de domínio.** Mascara padrões de e-mail, CPF e credenciais em atribuições simples. Emite contagens por regra e cria a saída somente se o destino não existir.

## Como executar

Instale Rust 1.85 com Cargo. Dependências adicionais constam em Cargo.toml. Os comandos abaixo partem da raiz do repositório.

```sh
cargo run -- examples/demo.log
```

## Testes

```sh
cargo test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `src/main.rs e examples/demo.log`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Não garante anonimização completa. Valores com espaços, credenciais fora dos padrões, Bearer tokens, telefones e dados no corpo podem permanecer. A contagem é por substituição, não por pessoa. Revise o resultado antes de compartilhar. Regras configuráveis e clap estão planejados.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Aplicar regras de mascaramento — ampliar o núcleo atual até o fluxo completo.
- [ ] Preservar arquivo original — ampliar o núcleo atual até o fluxo completo.
- [ ] Gerar saída e resumo de substituições — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
