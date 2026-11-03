# Escala Ministério

Organização de disponibilidade e escalas de músicos.

## Implementado na primeira versão

**Elixir — núcleo executável de domínio.** Monta uma escala para uma data, respeita instrumentos e disponibilidade e usa busca com retorno para não ocupar a mesma pessoa em duas funções.

## Como executar

Instale Elixir 1.18 com Erlang/OTP. O arquivo usa somente biblioteca padrão, GenServer ou ExUnit. Os comandos abaixo partem da raiz do repositório.

```sh
elixir main.exs examples/equipe.tsv 2026-11-08 voz,violao,baixo
```

## Testes

```sh
elixir main.exs --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `main.exs e examples/equipe.tsv`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Até 20 registros e 7 funções distintas por execução. Não trata intervalo de horários, rodízio entre semanas ou preferências. Não envia mensagens ou convites; Phoenix e persistência estão planejados.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Cadastrar participantes e instrumentos — ampliar o núcleo atual até o fluxo completo.
- [ ] Registrar disponibilidade — ampliar o núcleo atual até o fluxo completo.
- [ ] Montar escalas e identificar conflitos — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
