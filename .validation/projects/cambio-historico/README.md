# Câmbio Histórico

Consulta e comparação de cotações históricas de moedas.

## Implementado na primeira versão

**Elixir — núcleo executável de domínio.** Importa uma série TSV, valida observações por data/moeda e compara as pontas da série em pontos-base com aritmética inteira. Um ponto-base corresponde a 0,01 ponto percentual.

## Como executar

Instale Elixir 1.18 com Erlang/OTP. O arquivo usa somente biblioteca padrão, GenServer ou ExUnit. Os comandos abaixo partem da raiz do repositório.

```sh
elixir main.exs examples/cotacoes.tsv USD
```

## Testes

```sh
elixir main.exs --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `main.exs e examples/cotacoes.tsv`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Os valores de exemplo são inteiramente fictícios. Não consulta câmbio atual ou qualquer fonte pública. Unidade de cotação: milionésimos de real por unidade estrangeira; a variação é truncada para pontos-base. OTP e LiveView são evoluções previstas.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Consultar períodos na fonte pública — ampliar o núcleo atual até o fluxo completo.
- [ ] Exibir data, fonte e cotação — ampliar o núcleo atual até o fluxo completo.
- [ ] Comparar períodos e exportar dados — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
