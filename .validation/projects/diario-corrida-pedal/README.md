# Diário de Corrida e Pedal

Registro de atividades com evolução semanal de distância e duração.

## Implementado na primeira versão

**Dart — núcleo executável de domínio.** Consolida distância e duração por modalidade em um intervalo de sete dias, com fim exclusivo e validação de datas e valores positivos.

## Como executar

Instale Dart SDK 3.5 ou compatível. Não há pacotes externos. Os comandos abaixo partem da raiz do repositório.

```sh
dart run bin/main.dart 2026-11-02 examples/atividades.json
```

## Testes

```sh
dart run bin/main.dart --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `bin/main.dart e examples/atividades.json`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Entrada manual em JSON; sem GPS, orientação de treino ou sincronização. Flutter, Riverpod e Drift estão planejados.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Registrar corrida e pedal manualmente — ampliar o núcleo atual até o fluxo completo.
- [ ] Consultar histórico semanal — ampliar o núcleo atual até o fluxo completo.
- [ ] Exibir evolução de distância e duração — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
