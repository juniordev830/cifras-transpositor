# Operação

## Prévia e salvamento

`init REPOSITORIO` cria uma pasta nova, contendo `objects/` e `manifests/`. O diretório pai deve existir. Uma pasta já existente é recusada, mesmo que esteja vazia.

`plan ORIGEM REPOSITORIO` lê recursivamente a origem e informa número de arquivos, bytes totais, objetos novos e objetos reutilizados. Conteúdo repetido dentro da mesma origem conta como um objeto; `reused_objects` conta os objetos que já existem no armazenamento. A prévia não grava arquivos.

`save ORIGEM REPOSITORIO --name NOME` salva uma versão com nome de 1 a 64 letras ASCII, números, hífen ou sublinhado. O mesmo nome não pode ser substituído. Guarde a origem estável durante a operação; o programa não congela arquivos de outros processos.

## Verificação e restauração

`verify REPOSITORIO --name NOME` valida o formato do manifesto, os caminhos, o hash e o tamanho de todos os objetos referenciados. Arquivo ausente ou corrompido encerra o comando com código 1.

`restore REPOSITORIO NOVO_DESTINO --name NOME` verifica os objetos antes de criar uma pasta nova. Não reutiliza destinos existentes, nem restaura dentro do próprio repositório. Após a cópia, confere novamente cada arquivo. A pasta pai do destino deve existir.

## Falhas e recuperação

- Snapshot existente: escolha outro nome; o manifesto anterior é preservado.
- Objeto corrompido ou ausente: restaure esse armazenamento a partir de outra cópia confiável. O programa não tenta reparar automaticamente nem substituir objetos existentes.
- Trava `.write-lock`: outro salvamento pode estar ativo. Se o processo encerrou de forma anormal, confirme que nenhum salvamento está em execução antes de remover manualmente a trava. Uma trava existente nunca é removida pelo comando que falhou ao obtê-la.
- Falha durante restauração: a pasta parcialmente criada permanece. Inspecione-a e escolha outro destino vazio para uma nova tentativa; o programa não a exclui automaticamente.
- Falha durante `init`: uma estrutura parcial pode permanecer. Inspecione a pasta recém-criada antes de escolher um novo caminho ou removê-la.
- Origem igual ou aninhada no repositório: a operação é recusada para evitar copiar o próprio armazenamento.

## O que é preservado

Conteúdo dos arquivos regulares, nomes e subpastas necessárias. Não são preservados diretórios vazios, metadados de permissão/proprietário, links, atributos estendidos ou horários originais. Links e arquivos especiais na árvore de origem são recusados. A ferramenta não é projetada para árvores modificadas por processos hostis.

Os caminhos precisam ser UTF-8 e não podem conter componentes `.` ou `..`, barra invertida, dois-pontos ou caracteres de controle. Nomes que diferem somente por maiúsculas são recusados. Restrições adicionais do sistema operacional de destino ainda podem impedir uma restauração.

## Códigos de saída

| Código | Significado |
| --- | --- |
| 0 | Comando concluído ou ajuda exibida |
| 1 | Erro de operação, formato ou integridade |
| 2 | Argumentos inválidos, tratados por clap |
