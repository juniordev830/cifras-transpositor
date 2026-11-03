# Detetive de Bugs

Jogo educativo para identificar erros em pequenos programas.

## Implementado na primeira versão

**Lua — núcleo executável de domínio.** Jogo gráfico LÖVE com quatro desafios de código, múltipla escolha, explicação da resposta, pontuação e reinício. O código dos desafios é apenas texto e não é executado.

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

Conteúdo inicial fixo e progresso somente em memória. Expansão dos desafios e salvamento local estão planejados. Requer LÖVE 11.5 para a interface e Lua 5.4 para testes sem janela.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Apresentar desafios de código — ampliar o núcleo atual até o fluxo completo.
- [ ] Receber resposta sem executar código arbitrário — ampliar o núcleo atual até o fluxo completo.
- [ ] Explicar erros e acompanhar pontuação — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
