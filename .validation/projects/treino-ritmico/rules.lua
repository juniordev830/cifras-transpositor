local M={}
function M.interval(bpm) assert(type(bpm)=='number' and bpm==bpm and bpm>=30 and bpm<=240,'BPM deve estar entre 30 e 240');return 60/bpm end
function M.advance(phase,dt,bpm)
  assert(type(dt)=='number' and dt==dt and dt>=0 and dt<math.huge,'Tempo invalido')
  local interval=M.interval(bpm);local total=phase+dt;local beats=math.floor(total/interval)
  return total-beats*interval,beats
end
return M
