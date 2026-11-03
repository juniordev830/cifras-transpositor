# Logística Simulator

Jogo de organização de armazém com pedidos e limite de tempo.

## Implementado na primeira versão

**Lua — núcleo executável de domínio.** Jogo gráfico de armazém com movimentação, coleta de itens, pedidos em sequência, entrega validada, contagem regressiva e reinício.

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

Fase única com três pedidos fixos. Não há persistência ou geração aleatória de fases. Interface com LÖVE 11.5, regras puras testáveis em Lua 5.4.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Movimentar itens no armazém — ampliar o núcleo atual até o fluxo completo.
- [ ] Separar pedidos — ampliar o núcleo atual até o fluxo completo.
- [ ] Pontuar acertos dentro do tempo — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
