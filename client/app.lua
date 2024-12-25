local lapis = require("lapis")
local app = lapis.Application()
app.layout = "layout"
app:enable("etlua")

app:get("/", function()
  return { render = "home" }
end)

app:post("/", function(self)
  print(self.POST.url)
  local generated_url = "http://localhost:8080/r/6ecafd00-0987-4b7c-a174-2f82098f63f0"
  self.url = generated_url
  self.hash = "6ecafd00-0987-4b7c-a174-2f82098f63f0"
  return { render = "ready-url" }
end)

app:get("/hits/(:endpoint_hash)", function(self)
  if not self.params.endpoint_hash then
    return { render = "hits" }
  end

  local hits = {
    {
      ip = "127.0.0.1",
      hit_time = "2024-12-25 09:00",
      user_agent = "Mozilla/5.0 (X11; Linux x86_64)",
      last_hit_desc = "1 day ago",
    },
  }
  self.hits = hits
  self.url = "http://localhost:8080/r/6ecafd00-0987-4b7c-a174-2f82098f63f0"
  return { render = "hits" }
end)

return app
