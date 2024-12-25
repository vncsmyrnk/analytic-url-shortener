local lapis = require("lapis")
local app = lapis.Application()

local Model = require("lapis.db.model").Model
local Endpoints = Model:extend("endpoints", {
  timestamp = true,
})
local Hits = Model:extend("hits")
local uuid = require("uuid")
uuid.randomseed(os.time())
uuid.set_rng(function(bytes)
  local random_bytes = ""
  for _ = 1, bytes do
    random_bytes = random_bytes .. string.char(math.random(0, 255))
  end
  return random_bytes
end)

local json_params = require("lapis.application").json_params

app:get("/r/:hash", function(self)
  local endpoint = Endpoints:find({ hash = self.params.hash })
  if not endpoint then
    return { status = 404, layout = false }
  end
  local new_hit = {
    hit_time = os.date("%Y-%m-%d %H:%M:%S"),
    endpoint_id = endpoint.id,
    ip = self.req.remote_addr or "0.0.0.0",
    user_agent = self.req.headers["user-agent"] or "unknown",
  }
  Hits:create(new_hit)
  return {
    redirect_to = endpoint.url,
  }
end)

app:get("/endpoint/:id", function(self)
  local endpoint = Endpoints:find(self.params.id)
  if not endpoint then
    return { status = 404, layout = false }
  end
  return { json = endpoint }
end)

app:get("/endpoint/:id/hits", function(self)
  local endpoint = Endpoints:find(self.params.id)
  if not endpoint then
    return { status = 404, layout = false }
  end
  local hits = Hits:find_all({ endpoint.id }, "endpoint_id")
  if #hits == 0 then
    return { status = 404, layout = false }
  end
  return { json = hits }
end)

app:post(
  "/endpoint",
  json_params(function(self)
    local new_endpoint = {
      name = self.params.name,
      url = self.params.url,
      hash = uuid(),
    }
    local endpoint = Endpoints:create(new_endpoint)
    return { json = { endpoint } }
  end)
)

return app
