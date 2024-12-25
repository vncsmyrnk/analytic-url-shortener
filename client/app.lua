local lapis = require("lapis")
local http = require("lapis.nginx.http")
local cjson = require("cjson")

local app = lapis.Application()
app.layout = "layout"
app:enable("etlua")

app:get("/", function()
  return { render = "home" }
end)

app:post("/", function(self)
  local body, status_code, _ = http.simple("http://127.0.0.1:8080/endpoint", {
    name = "name",
    url = self.POST.url,
  })
  if status_code ~= 200 then
    self.message = body
    return { render = "error" }
  end
  local data = cjson.decode(body)
  print(body)
  self.url = data["url"]
  self.hash = data["hash"]
  return { render = "ready-url" }
end)

app:get("/hits", function(self)
  if not self.params.endpoint_hash then
    return { render = "hits" }
  end

  local body, status_code, _ = http.simple("http://127.0.0.1:8080/hits/" .. self.params.endpoint_hash)
  if status_code ~= 200 then
    self.message = body
    return { render = "error" }
  end
  local data = cjson.decode(body)
  print(body)
  self.hits = data
  self.hash = self.params.endpoint_hash
  return { render = "hits" }
end)

return app
