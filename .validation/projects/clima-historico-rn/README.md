# Clima Histórico RN

Exploração de séries históricas de chuva e temperatura do RN.

## Implementado na primeira versão

**Julia — núcleo executável de domínio.** Resume observações por estação e mês, separa dados ausentes de chuva zero e informa quantos dias possuem observação em relação aos dias do mês.

## Como executar

Instale Julia 1.11 ou compatível. São utilizadas somente Dates e Statistics da distribuição. Os comandos abaixo partem da raiz do repositório.

```sh
julia main.jl examples/observacoes.csv
```

## Testes

```sh
julia main.jl --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `main.jl e examples/observacoes.csv`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Fixture sintética, sem dados meteorológicos reais conectados. CSV simples sem campos com vírgulas ou aspas. A soma de chuva é somente dos dias observados, não uma estimativa do total mensal. CSV.jl, DataFrames e Makie estão planejados.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Importar dados históricos de estações — ampliar o núcleo atual até o fluxo completo.
- [ ] Mostrar falhas e cobertura dos dados — ampliar o núcleo atual até o fluxo completo.
- [ ] Comparar chuva e temperatura por período — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
