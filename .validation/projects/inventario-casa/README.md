# Inventário da Casa

Catálogo de objetos por cômodo com exportação de inventário.

## Implementado na primeira versão

**Dart — núcleo executável de domínio.** Agrupa um inventário por cômodo, soma unidades e valor em centavos inteiros e rejeita IDs duplicados.

## Como executar

Instale Dart SDK 3.5 ou compatível. Não há pacotes externos. Os comandos abaixo partem da raiz do repositório.

```sh
dart run bin/main.dart examples/itens.json
```

## Testes

```sh
dart run bin/main.dart --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `bin/main.dart e examples/itens.json`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Lê JSON e exporta resumo pela saída padrão. Não gerencia fotos ou banco local ainda. Flutter e Drift fazem parte do roteiro.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Cadastrar cômodos e objetos — ampliar o núcleo atual até o fluxo completo.
- [ ] Adicionar fotos locais — ampliar o núcleo atual até o fluxo completo.
- [ ] Exportar inventário — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
