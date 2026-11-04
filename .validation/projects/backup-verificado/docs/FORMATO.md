# Formato de armazenamento — versão 1

| Caminho | Conteúdo |
| --- | --- |
| `objects/HASH` | Bytes de um arquivo, nomeados pelo BLAKE3 hexadecimal de 64 caracteres |
| `manifests/NOME.json` | Manifesto de uma versão da pasta |
| `.write-lock` | Marcador transitório de um salvamento, com PID informativo |

O manifesto contém `format_version`, `snapshot`, `created_unix_seconds` e `entries`. Cada entrada registra `path`, `bytes` e `hash`. `path` é relativo à origem e usa `/`; o hash identifica o conteúdo, permitindo que várias entradas apontem ao mesmo objeto.

`created_unix_seconds` é o instante de criação do manifesto, não o horário de modificação dos arquivos de origem. A ordenação das entradas usa os caminhos relativos. Limites: 100 mil entradas, profundidade de 128 diretórios e 16 MiB para o manifesto serializado.

O carregamento recusa versões desconhecidas, campos não reconhecidos, hashes inválidos, caminhos absolutos ou com travessia, caminhos repetidos, conflito de maiúsculas e arquivos usados como diretórios. O nome dentro do JSON deve corresponder ao nome do arquivo do manifesto.

Um hash detecta alteração de conteúdo em relação ao manifesto carregado. O manifesto não é assinado: esta verificação não autentica uma origem nem impede que alguém com permissão de escrita substitua manifesto e objetos juntos.

`manifest.schema.json` documenta os tipos e limites básicos. As regras de unicidade de caminhos, conflito entre diretórios e nome do arquivo são aplicadas pelo domínio Rust, além do esquema.
