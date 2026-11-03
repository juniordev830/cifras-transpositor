# Pipeline de Qualidade de Dados

Importação de dados com validação e separação de registros inválidos.

## Implementado na primeira versão

**Scala — núcleo executável de domínio.** Valida um esquema tabular, separa linhas aceitas e rejeitadas e aponta os motivos com número da linha. Mantém IDs como texto e identifica duplicatas.

## Como executar

Instale JDK 17+ e Scala CLI 1.6.1 ou compatível. O arquivo fixa Scala 3.3.5. Os comandos abaixo partem da raiz do repositório.

```sh
scala-cli run Main.scala --server=false -- examples/entrada.tsv
```

## Testes

```sh
scala-cli run Main.scala --server=false -- --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `Main.scala e examples/entrada.tsv`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Processamento em memória com Scala 3. A primeira ocorrência reserva o ID mesmo se rejeitada; ocorrências seguintes são duplicatas. Validação de e-mail é apenas sintática. Não utiliza Spark ou Delta Lake nesta primeira versão.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Definir esquema dos dados — ampliar o núcleo atual até o fluxo completo.
- [ ] Validar campos e duplicatas — ampliar o núcleo atual até o fluxo completo.
- [ ] Separar registros aceitos e rejeitados — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
