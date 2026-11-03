# Planeja Semana

Planejamento de tarefas conforme a capacidade disponível na semana.

## Implementado na primeira versão

**Swift — núcleo executável de domínio.** Distribui tarefas indivisíveis conforme a capacidade de sete dias, prioriza notas maiores e devolve uma lista explícita do que não coube.

## Como executar

Instale Swift 6.0 ou compatível, com Foundation. O programa usa somente a biblioteca padrão. Os comandos abaixo partem da raiz do repositório.

```sh
swift main.swift examples/semana.json
```

## Testes

```sh
swift main.swift --self-test
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `main.swift e examples/semana.json`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Algoritmo guloso determinístico: não garante a distribuição ótima. Não divide tarefas nem inclui horários fixos. Capacidade em minutos, de segunda a domingo; não integra calendário. Swift Package Manager e interface são próximos passos.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Definir capacidade semanal — ampliar o núcleo atual até o fluxo completo.
- [ ] Distribuir e reagendar tarefas — ampliar o núcleo atual até o fluxo completo.
- [ ] Salvar dados no navegador — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
