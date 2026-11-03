local M={}
M.challenges={
  {code='for i = 1, #itens - 1 do\n  total = total + itens[i]\nend',question='Qual erro faz o total ficar incompleto?',choices={'O ultimo item nao entra na soma','O Lua comeca em zero','O total deveria ser texto'},answer=1,why='Em Lua, listas convencionais comecam em 1. O limite correto aqui e #itens.'},
  {code='if senha = "demo" then\n  entrar()\nend',question='Qual operador deveria comparar valores?',choices={'=','==','=>'},answer=2,why='Comparacao usa ==. Atribuicao usa = e nao e uma expressao de condicao valida aqui.'},
  {code='local saldo = "10"\nprint(saldo .. "5")',question='Qual e o resultado impresso?',choices={'15','Erro de divisao','105'},answer=3,why='.. concatena textos. Para soma numerica, converta e use +.'},
  {code='local dados = nil\nprint(dados.nome)',question='Por que esse acesso falha?',choices={'dados e nil','nome e uma palavra reservada','print sempre retorna erro'},answer=1,why='E preciso validar a existencia do objeto antes de indexa-lo.'}
}
function M.answer(state,choice)
  local challenge=M.challenges[state.index]
  if not challenge or state.answered or type(choice)~='number' or choice%1~=0 or choice<1 or choice>#challenge.choices then return false end
  state.answered=true; state.correct=choice==challenge.answer
  if state.correct then state.score=state.score+10 end
  return true
end
return M
