# Arquitetura proposta — Clima Histórico RN

## Objetivo

Exploração de séries históricas de chuva e temperatura do RN.

## Escolha técnica

Cálculo numérico de séries climáticas e gráficos com um ecossistema novo no portfólio.

Stack proposta: Julia, CSV.jl, DataFrames.jl e Makie.

## Conceitos de domínio a detalhar

- Estação
- Observação
- Cobertura

## Responsabilidades

| Camada | Responsabilidade |
| --- | --- |
| Entrada e interface | Receber dados, mostrar validações e apresentar o resultado |
| Casos de uso | Coordenar cada operação do escopo inicial |
| Domínio | Definir regras, invariantes e transformações sem depender da interface |
| Adaptadores | Isolar arquivos, persistência e serviços externos quando forem adicionados |

A estrutura de código será ajustada às convenções de Julia; estas camadas descrevem responsabilidades e não impõem uma quantidade fixa de pastas.

## Entregas por etapas

1. Escrever um cenário de uso com entrada e resultado esperado.
2. Implementar a menor operação útil e tratar entrada inválida.
3. Concluir os demais itens do escopo inicial.
4. Adicionar persistência ou integração somente onde o fluxo precisar.
5. Registrar instalação, execução, validação e limitações.

## Escopo funcional

- [ ] Importar dados históricos de estações
- [ ] Mostrar falhas e cobertura dos dados
- [ ] Comparar chuva e temperatura por período

## Dados e integrações

Não há serviços externos conectados nem dados reais neste pacote. Exemplos devem ser fictícios. Projetos que consultarem fontes públicas devem registrar a fonte, a data e as condições de uso junto dos dados importados.

## Estado de validação

A especificação foi preparada. Compilação e testes da aplicação ainda não se aplicam porque o código não foi implementado.
