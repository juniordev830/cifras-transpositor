# Webhook Inbox

Recepção e inspeção de webhooks em ambiente de desenvolvimento.

## Implementado na primeira versão

**Elixir — núcleo executável de domínio.** Implementa a caixa de entrada em GenServer, com limite de 16 KiB por corpo, 50 eventos e retenção de dez minutos. Oculta três cabeçalhos conhecidos.

## Como executar

Instale Elixir 1.18 com Erlang/OTP. O arquivo usa somente biblioteca padrão, GenServer ou ExUnit. Os comandos abaixo partem da raiz do repositório.

```sh
elixir main.exs --demo
```

## Testes

```sh
elixir main.exs --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `main.exs`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Ainda não expõe endpoint HTTP: somente a API interna e uma demonstração em memória. Cabeçalhos customizados e o corpo não são sanitizados; use dados fictícios. Phoenix, ETS e interface de inspeção estão planejados.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Receber requisições de teste — ampliar o núcleo atual até o fluxo completo.
- [ ] Exibir cabeçalhos e conteúdo — ampliar o núcleo atual até o fluxo completo.
- [ ] Aplicar limites de tamanho e retenção — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
