# Torre de Defesa

Jogo de estratégia com ondas de inimigos, torres e melhorias.

## Implementado na primeira versão

**GDScript — núcleo executável de domínio.** Fase 2D jogável com torres, alcance de tiro, melhorias, trinta inimigos em dificuldade crescente, ouro, vidas, vitória/derrota e reinício. Toda a arte é desenhada por código.

## Como executar

Instale Godot 4.4.1 ou compatível. Abra project.godot pelo editor ou use os comandos abaixo. Os comandos abaixo partem da raiz do repositório.

```sh
godot --path .
```

## Testes

```sh
godot --headless --path . --script tests.gd
godot --headless --path . --quit-after 3
```

Os testes cobrem invariantes e entradas inválidas. A preparação local não dispõe deste runtime; o workflow em `.github/workflows/ci.yml` executa a validação no GitHub Actions. O resultado de cada execução está na aba Actions, associado ao commit.

## Organização e limites

Código e exemplos: `project.godot, main.tscn, main.gd, rules.gd e tests.gd`. Os dados de exemplo são fictícios. As funções de domínio são separadas da leitura de argumentos no mesmo arquivo para manter a primeira versão pequena e revisável.

Protótipo de fase única com caminho reto, sem assets externos ou áudio. Requer Godot 4.4.1 ou compatível. Equilíbrio de dificuldade e funcionamento visual ainda precisam de teste manual; os testes automatizados cobrem regras e inicialização.

A proposta completa de evolução está em `ARQUITETURA.md`. As tecnologias propostas nessa arquitetura não devem ser interpretadas como dependências já implementadas.

## Próximas entregas

- [ ] Criar uma fase jogável — ampliar o núcleo atual até o fluxo completo.
- [ ] Implementar torres e ondas — ampliar o núcleo atual até o fluxo completo.
- [ ] Adicionar vitória, derrota e reinício — ampliar o núcleo atual até o fluxo completo.

Licença: ainda não definida.
