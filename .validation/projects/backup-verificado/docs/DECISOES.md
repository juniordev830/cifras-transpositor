# Decisões técnicas

## Objetos por conteúdo e BLAKE3

Um arquivo é identificado pelo hash de seus bytes. Isso permite reutilização dentro de uma versão e entre versões, com comparação de integridade no mesmo processo. O algoritmo é fixo na versão 1 do formato; trocar o algoritmo exigirá um novo formato ou uma migração explícita.

## Manifesto separado dos objetos

Cada snapshot tem nome exclusivo e lista os objetos necessários. Os objetos são publicados antes do manifesto. Em caso de falha, podem sobrar objetos sem referência; não há coletor automático nesta versão. Snapshots antigos não são alterados para reaproveitar espaço.

## Tipos separados da interface

clap cuida dos argumentos e da ajuda. Serde cuida de tipos e campos do JSON. O domínio adiciona invariantes que não dependem da CLI: caminhos, hashes, unicidade, versão e conflitos entre arquivos e diretórios. A leitura de disco está concentrada no repositório e no módulo de caminhos.

## Publicação sem sobrescrita

Arquivos temporários ficam na mesma pasta de destino. O conteúdo é sincronizado antes de `persist_noclobber`. O comando não oferece uma transação do diretório inteiro nem promete durabilidade contra qualquer falha de energia. A restauração exige um destino novo, usa criação exclusiva e preserva uma pasta parcial se ocorrer uma falha posterior.

## Escopo local

Não há nuvem, criptografia, assinatura, compressão ou transporte de dados. O protótipo usa arquivos regulares em um armazenamento local estável. A CI testa Ubuntu; outros sistemas ainda dependem de validação específica. Permissões originais e diretórios vazios não fazem parte do formato.
