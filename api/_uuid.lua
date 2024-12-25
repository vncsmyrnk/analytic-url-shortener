local uuid = require("uuid")
uuid.randomseed(os.time())
uuid.set_rng(function(bytes)
  local random_bytes = ""
  for _ = 1, bytes do
    random_bytes = random_bytes .. string.char(math.random(0, 255))
  end
  return random_bytes
end)
return uuid
