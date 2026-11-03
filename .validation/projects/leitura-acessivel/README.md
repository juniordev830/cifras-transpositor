# Leitura Acessível

Leitor de textos com ajustes de fonte, contraste e espaçamento.

## Implementado na primeira versão

**Swift — núcleo executável de domínio.** Gera uma página HTML local a partir de texto, escapa conteúdo, oferece fonte ajustável, navegação por teclado e contraste conforme a preferência do sistema.

## Como executar

Instale Swift 6.0 ou compatível, com Foundation. O programa usa somente a biblioteca padrão. Os comandos abaixo partem da raiz do repositório.

```sh
swift main.swift examples/texto.txt 22 > leitura.html
```

## Testes

```sh
swift main.swift --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `main.swift e examples/texto.txt`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Não é uma certificação de acessibilidade. Preferências ainda não são persistidas. Não há leitura por voz, importação de PDF ou publicação na web.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Importar texto — ampliar o núcleo atual até o fluxo completo.
- [ ] Ajustar apresentação e navegação por teclado — ampliar o núcleo atual até o fluxo completo.
- [ ] Salvar preferências locais — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
