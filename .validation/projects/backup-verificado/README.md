# Backup Verificado

Ferramenta de linha de comando em **Rust** para guardar versões de uma pasta, verificar a integridade dos arquivos e restaurar uma versão em um diretório novo. Arquivos com conteúdo igual compartilham um único objeto BLAKE3, inclusive entre snapshots.

## Entrega 0.2

- Comandos `init`, `plan`, `save`, `verify` e `restore`, com ajuda gerada por clap.
- Leitura recursiva com detecção de links, caminhos inválidos e nomes conflitantes.
- Manifestos JSON versionados, objetos por hash e reutilização de conteúdo.
- Gravação de novos objetos e manifestos por arquivo temporário, sem sobrescrita.
- Verificação de tamanho e hash antes da restauração; destino existente é recusado.
- Testes de integração para deduplicação, recuperação, corrupção e execução da CLI.

## Executar um exemplo

Instale Rust 1.85 ou compatível. Os nomes `.demo-repo` e `.demo-restored` precisam estar livres; a origem de exemplo contém apenas dados fictícios.

```sh
cargo run -- init .demo-repo
cargo run -- plan examples/origem .demo-repo
cargo run -- save examples/origem .demo-repo --name exemplo-01
cargo run -- verify .demo-repo --name exemplo-01
cargo run -- restore .demo-repo .demo-restored --name exemplo-01
cargo run -- --help
```

`plan` somente lê arquivos. `save` cria uma versão; para outra versão, escolha outro nome. `restore` exige um destino novo e nunca usa a pasta de origem como destino implicitamente. A interface e os manifestos usam caminhos relativos com separador `/`.

## Tecnologias e organização

| Caminho | Responsabilidade |
| --- | --- |
| `src/main.rs` | Argumentos e apresentação da CLI com clap |
| `src/lib.rs` | API pública e tipo de erro |
| `src/model.rs` | Manifesto, plano e invariantes de domínio |
| `src/paths.rs` | Nomes, caminhos e varredura recursiva |
| `src/repository.rs` | Objetos, snapshots, verificação e restauração |
| `tests/repository.rs` | Cenários de integração e execução real da CLI |
| `docs/OPERACAO.md` | Comandos, falhas e recuperação operacional |
| `docs/FORMATO.md` | Estrutura dos objetos e do manifesto |
| `docs/DECISOES.md` | Decisões técnicas e seus limites |
| `examples/` | Arquivos fictícios para experimentar |

BLAKE3 calcula os hashes no processo Rust. Serde e serde_json serializam os manifestos; tempfile prepara os arquivos antes da publicação no armazenamento. O projeto não chama `sha256sum` nem depende de um comando externo de hash.

## Validação

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

A CI usa Rust 1.85 e Ubuntu. O resultado por commit está em Actions. Os exemplos e a CLI também são exercitados pelos testes de integração. Outros sistemas operacionais ainda não foram executados na CI.

## Limites atuais

O snapshot é uma cópia de arquivos lidos em sequência, sem congelar o sistema de arquivos; mantenha a origem estável durante o salvamento. Diretórios vazios, permissões, proprietário, links e atributos estendidos não são preservados. São aceitos até 100 mil arquivos e 16 MiB de manifesto.

Não há criptografia, autenticação do manifesto, compressão, exclusão de snapshots ou limpeza automática de objetos sem referência. Uma falha de restauração posterior à criação da pasta pode deixar uma cópia parcial para inspeção. Consulte os detalhes em `docs/OPERACAO.md` antes de usar em dados importantes.

A versão 0.1 fazia cópias avulsas com SHA-256; não há importação automática dessas cópias para o armazenamento da versão 0.2.

Licença: ainda não definida.
