reconcile <- function(left, right, key) {
  if (!(key %in% names(left)) || !(key %in% names(right))) stop("Chave ausente")
  if (anyDuplicated(names(left)) || anyDuplicated(names(right))) stop("Coluna duplicada")
  for (x in list(left,right)) {
    if (anyNA(x) || any(x[[key]] == "") || anyDuplicated(x[[key]])) stop("Chave vazia/duplicada ou valor NA; corrija a entrada")
  }
  ids <- sort(unique(c(left[[key]],right[[key]])))
  columns <- union(setdiff(names(left),key),setdiff(names(right),key))
  result <- lapply(ids,function(id) {
    li <- match(id,left[[key]]); ri <- match(id,right[[key]])
    if (is.na(li)) return(data.frame(chave=id,status="somente_direita",campos=""))
    if (is.na(ri)) return(data.frame(chave=id,status="somente_esquerda",campos=""))
    changed <- columns[vapply(columns,function(col) {
      if (!(col %in% names(left)) || !(col %in% names(right))) return(TRUE)
      left[li,col] != right[ri,col]
    },logical(1))]
    data.frame(chave=id,status=if(length(changed)) "divergente" else "igual",campos=paste(changed,collapse=";"))
  })
  if (!length(result)) return(data.frame(chave=character(),status=character(),campos=character()))
  do.call(rbind,result)
}
args <- commandArgs(trailingOnly=TRUE)
tryCatch({
  if (identical(args,"--self-test")) {
    a <- data.frame(id=c("01","02"),valor=c("10","20"),stringsAsFactors=FALSE)
    b <- data.frame(id=c("02","03"),valor=c("21","30"),stringsAsFactors=FALSE)
    out <- reconcile(a,b,"id")
    stopifnot(identical(out$status,c("somente_esquerda","divergente","somente_direita")),out$campos[2]=="valor")
    stopifnot(inherits(try(reconcile(rbind(a,a[1,]),b,"id"),silent=TRUE),"try-error"))
    cat("2 cenários OK\n")
  } else {
    if (length(args)!=3) stop("Uso: Rscript main.R esquerda.csv direita.csv CHAVE > relatorio.csv")
    load_csv <- function(path) read.csv(path,colClasses="character",check.names=FALSE,na.strings=character(),stringsAsFactors=FALSE)
    write.csv(reconcile(load_csv(args[1]),load_csv(args[2]),args[3]),stdout(),row.names=FALSE,na="")
  }
}, error=function(e) {cat(paste0("Erro: ",conditionMessage(e),"\n"),file=stderr());quit(status=2)})
