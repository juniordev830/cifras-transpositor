local r=require('rules')
local s={index=1,score=0,answered=false}
assert(not r.answer(s,nil) and not r.answer(s,1.5))
assert(r.answer(s,1) and s.score==10)
assert(not r.answer(s,1) and s.score==10)
s={index=2,score=0,answered=false};assert(r.answer(s,1) and s.score==0)
print('4 cenarios OK')
