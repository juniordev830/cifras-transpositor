local rules=require('rules')
local bpm,beats,phase,elapsed,running,flash,compass=90,0,0,0,false,0,4
local click
function love.load()
  love.window.setMode(900,580);love.window.setTitle('Treino Ritmico')
  love.graphics.setFont(love.graphics.newFont(22))
  local sound=love.sound.newSoundData(1323,44100,16,1)
  for i=0,1322 do sound:setSample(i,math.sin(i/44100*2*math.pi*1000)*0.22*(1-i/1323)) end
  click=love.audio.newSource(sound,'static')
end
function love.keypressed(key)
  if key=='space' then running=not running;phase=0 end
  if key=='up' then bpm=math.min(240,bpm+1);phase=0 end
  if key=='down' then bpm=math.max(30,bpm-1);phase=0 end
  if key=='3' then compass=3;beats=0 end
  if key=='4' then compass=4;beats=0 end
  if key=='r' then elapsed=0;beats=0;phase=0;running=false end
  if key=='escape' then love.event.quit() end
end
function love.update(dt)
  flash=math.max(0,flash-dt)
  if not running then return end
  elapsed=elapsed+dt;local count;phase,count=rules.advance(phase,dt,bpm)
  if count>0 then beats=beats+count;flash=0.12;click:stop();click:setPitch((beats-1)%compass==0 and 1.4 or 1);click:play() end
end
function love.draw()
  love.graphics.clear(0.035,0.055,0.10);love.graphics.setColor(0.3,0.9,0.8)
  love.graphics.print('TREINO RITMICO / PULSO E COMPASSO',35,30)
  love.graphics.setColor(0.95,0.96,1);love.graphics.printf(bpm..' BPM',0,130,900,'center')
  for i=1,compass do
    local active=flash>0 and ((beats-1)%compass)+1==i
    love.graphics.setColor(active and 0.4 or 0.16,active and 1 or 0.22,active and 0.8 or 0.32)
    love.graphics.circle('fill',450+(i-(compass+1)/2)*120,270,38)
    love.graphics.setColor(0.95,0.96,1);love.graphics.print(i,442+(i-(compass+1)/2)*120,256)
  end
  love.graphics.setColor(0.85,0.9,1);love.graphics.printf((running and 'TOCANDO' or 'PAUSADO')..'  |  '..math.floor(elapsed)..'s  |  '..beats..' pulsos',0,365,900,'center')
  love.graphics.printf('Espaco: iniciar/pausar   Setas: BPM\n3 ou 4: compasso   R: zerar sessao   Esc: sair',30,440,840,'center')
end
