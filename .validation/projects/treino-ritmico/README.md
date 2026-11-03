# Treino Rítmico

Metrônomo e exercícios de ritmo com registro de sessões.

## Implementado na primeira versão

**Lua — núcleo executável de domínio.** Metrônomo gráfico com clique sintetizado, BPM entre 30 e 240, compassos 3/4 e 4/4, acento visual/sonoro e contadores da sessão.

## Como executar

Instale LÖVE 11.5 para jogar e Lua 5.4 para executar os testes sem janela. Os comandos abaixo partem da raiz do repositório.

```sh
love .
```

## Testes

```sh
lua tests.lua
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `main.lua, rules.lua e tests.lua`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

O áudio é disparado pelo ciclo de quadros e pode atrasar em computadores sobrecarregados; não substitui um relógio de áudio profissional. Após pausa longa, o contador avança, mas soa apenas o pulso mais recente. Histórico persistente e exercícios de precisão estão planejados.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Configurar andamento e compasso — ampliar o núcleo atual até o fluxo completo.
- [ ] Reproduzir pulsos com controle de início — ampliar o núcleo atual até o fluxo completo.
- [ ] Registrar sessões de prática — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
