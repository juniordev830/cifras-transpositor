local M={}
function M.new() return {x=2,y=3,holding=nil,orders={'A','B','C'},nextOrder=1,score=0,remaining=90,items={{kind='A',x=1,y=1},{kind='B',x=8,y=1},{kind='C',x=5,y=5}}} end
function M.move(s,dx,dy)
  if s.remaining<=0 or s.nextOrder>#s.orders then return end
  if math.abs(dx)+math.abs(dy)~=1 then return end
  s.x=math.max(1,math.min(9,s.x+dx));s.y=math.max(1,math.min(6,s.y+dy))
end
function M.interact(s)
  if s.remaining<=0 or s.nextOrder>#s.orders then return 'fim' end
  if s.holding and s.x==9 and s.y==6 then
    if s.holding~=s.orders[s.nextOrder] then return 'pedido_incorreto' end
    s.holding=nil;s.nextOrder=s.nextOrder+1;s.score=s.score+100;return 'entregue'
  end
  if not s.holding then
    for i,item in ipairs(s.items) do
      if item.x==s.x and item.y==s.y then s.holding=item.kind;table.remove(s.items,i);return 'coletado' end
    end
  end
  return 'sem_acao'
end
return M
