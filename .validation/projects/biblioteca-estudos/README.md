# Biblioteca de Estudos

Catálogo de livros, fichamentos, citações e assuntos estudados.

## Implementado na primeira versão

**Scala — núcleo executável de domínio.** Importa fichamentos TSV com obra, autor, página, assunto e texto; busca em todos os campos e exporta os resultados em ordem de obra/página.

## Como executar

Instale JDK 17+ e Scala CLI 1.6.1 ou compatível. O arquivo fixa Scala 3.3.5. Os comandos abaixo partem da raiz do repositório.

```sh
scala-cli run Main.scala --server=false -- examples/notas.tsv testes
```

## Testes

```sh
scala-cli run Main.scala --server=false -- --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `Main.scala e examples/notas.tsv`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Não importa PDFs, não acessa obras protegidas e não gera citações acadêmicas automaticamente. TSV não aceita tabs ou quebras dentro de campos. DuckDB e cadastro persistente estão planejados.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Cadastrar obras e assuntos — ampliar o núcleo atual até o fluxo completo.
- [ ] Escrever fichamentos com referência — ampliar o núcleo atual até o fluxo completo.
- [ ] Buscar e exportar anotações — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
