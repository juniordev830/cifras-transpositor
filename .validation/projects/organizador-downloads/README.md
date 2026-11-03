# Organizador de Downloads

Organização de arquivos por regras com prévia e desfazer.

## Implementado na primeira versão

**Swift — núcleo executável de domínio.** Gera um plano de organização por extensão, ordena os arquivos, ignora links e sinaliza destinos já existentes.

## Como executar

Instale Swift 6.0 ou compatível, com Foundation. O programa usa somente a biblioteca padrão. Os comandos abaixo partem da raiz do repositório.

```sh
swift main.swift examples/downloads
```

## Testes

```sh
swift main.swift --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `main.swift e examples/downloads/`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Esta primeira versão mostra somente a prévia e não move arquivos. Aplicação com confirmação, histórico e desfazer ainda precisam ser implementados. Foundation é utilizado; Swift Package Manager está planejado.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Definir regras por extensão — ampliar o núcleo atual até o fluxo completo.
- [ ] Mostrar prévia antes de mover — ampliar o núcleo atual até o fluxo completo.
- [ ] Registrar operações para desfazer — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
