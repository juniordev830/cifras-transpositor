local rules=require('rules')
local state
local function reset() state={index=1,score=0,answered=false} end
function love.load()
  love.window.setMode(940,640);love.window.setTitle('Detetive de Bugs')
  love.graphics.setFont(love.graphics.newFont(20));reset()
end
function love.keypressed(key)
  if key=='r' then reset();return end
  if key=='escape' then love.event.quit();return end
  if state.index>#rules.challenges then return end
  if key=='return' and state.answered then state.index=state.index+1;state.answered=false;return end
  rules.answer(state,tonumber(key))
end
function love.draw()
  love.graphics.clear(0.035,0.055,0.10)
  love.graphics.setColor(0.35,0.95,0.78);love.graphics.print('DETECTIVE / LABORATORIO DE CODIGO',36,26)
  love.graphics.setColor(0.85,0.89,0.96);love.graphics.print('Pontos: '..state.score..'  |  R reinicia  |  Esc sai',36,68)
  local c=rules.challenges[state.index]
  if not c then love.graphics.printf('Investigacao concluida!\n'..state.score..' de '..(#rules.challenges*10)..' pontos.\nPressione R para jogar novamente.',36,210,850);return end
  love.graphics.setColor(0.10,0.15,0.23);love.graphics.rectangle('fill',36,118,868,150,16,16)
  love.graphics.setColor(0.95,0.95,0.98);love.graphics.printf(c.code,56,136,828)
  love.graphics.printf(c.question,36,290,868)
  for i,choice in ipairs(c.choices) do love.graphics.print(i..'. '..choice,46,330+i*42) end
  if state.answered then
    love.graphics.setColor(state.correct and 0.4 or 1,0.85,0.55)
    love.graphics.printf((state.correct and 'Acertou! ' or 'Vamos revisar: ')..c.why..'\nEnter para continuar.',36,520,868)
  end
end
