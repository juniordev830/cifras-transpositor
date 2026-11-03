# Conciliador CSV

Comparação de arquivos tabulares para encontrar divergências.

## Implementado na primeira versão

**R — núcleo executável de domínio.** Compara CSVs por chave textual, identifica registros ausentes e campos divergentes e exporta um relatório por chave. Preserva zeros à esquerda.

## Como executar

Instale R 4.4 ou compatível. O programa usa somente R base. Os comandos abaixo partem da raiz do repositório.

```sh
Rscript main.R examples/esquerda.csv examples/direita.csv id > relatorio.csv
```

## Testes

```sh
Rscript main.R --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `main.R e examples/*.csv`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Comparação textual exata, sem tolerância numérica ou normalização. Chaves duplicadas e colunas duplicadas encerram a execução para evitar reconciliação ambígua. O relatório lista campos divergentes, sem copiar os valores. data.table e Quarto estão planejados.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Importar dois arquivos CSV — ampliar o núcleo atual até o fluxo completo.
- [ ] Escolher chave de comparação — ampliar o núcleo atual até o fluxo completo.
- [ ] Exportar ausências, duplicatas e divergências — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
