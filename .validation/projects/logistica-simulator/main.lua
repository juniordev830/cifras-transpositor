local rules=require('rules');local s;local message='Colete A, B e C nessa ordem e leve a EXPEDICAO.'
local palette={A={0.3,0.85,1},B={1,0.65,0.25},C={0.8,0.45,1}}
function love.load() love.window.setMode(940,650);love.window.setTitle('Logistica Simulator');love.graphics.setFont(love.graphics.newFont(20));s=rules.new() end
function love.keypressed(key)
  if key=='r' then s=rules.new();message='Nova rodada';return end
  if key=='escape' then love.event.quit() end
  local moves={w={0,-1},up={0,-1},s={0,1},down={0,1},a={-1,0},left={-1,0},d={1,0},right={1,0}}
  if moves[key] then rules.move(s,moves[key][1],moves[key][2]) end
  if key=='e' then message=rules.interact(s) end
end
function love.update(dt) if s.nextOrder<=#s.orders then s.remaining=math.max(0,s.remaining-dt) end end
function love.draw()
  love.graphics.clear(0.04,0.06,0.10);love.graphics.setColor(0.4,0.9,0.8)
  love.graphics.print('LOGISTICA / ARMAZEM',32,24)
  love.graphics.setColor(0.9,0.93,1);love.graphics.print('Tempo '..math.ceil(s.remaining)..'s   Pontos '..s.score..'   Pedido '..(s.orders[s.nextOrder] or 'concluido'),32,64)
  for x=1,9 do for y=1,6 do
    love.graphics.setColor(0.10,0.14,0.21);love.graphics.rectangle('fill',32+(x-1)*96,110+(y-1)*68,90,62,6,6)
  end end
  love.graphics.setColor(0.2,0.6,0.4);love.graphics.rectangle('fill',800,450,90,62,6,6)
  love.graphics.setColor(1,1,1);love.graphics.print('SAIDA',809,467)
  for _,item in ipairs(s.items) do love.graphics.setColor(palette[item.kind]);love.graphics.rectangle('fill',56+(item.x-1)*96,126+(item.y-1)*68,38,30,5,5);love.graphics.setColor(0,0,0);love.graphics.print(item.kind,67+(item.x-1)*96,128+(item.y-1)*68) end
  love.graphics.setColor(0.95,0.96,1);love.graphics.circle('fill',77+(s.x-1)*96,140+(s.y-1)*68,17)
  love.graphics.print('Carga: '..(s.holding or 'vazia')..'  |  '..message,32,540)
  love.graphics.print('WASD/setas: andar   E: coletar/entregar   R: reiniciar',32,582)
  if s.remaining<=0 or s.nextOrder>#s.orders then love.graphics.setColor(1,0.85,0.4);love.graphics.print(s.nextOrder>#s.orders and 'Todos os pedidos entregues!' or 'Tempo esgotado. Tente outra vez!',32,615) end
end
