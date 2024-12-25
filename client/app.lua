local lapis = require("lapis")
local http = require("lapis.nginx.http")
local cjson = require("cjson")
local app_helpers = require("lapis.application")
local config = require("lapis.config").get()

local capture_errors, yield_error = app_helpers.capture_errors, app_helpers.yield_error

local app = lapis.Application()
app.layout = "layout"
app:enable("etlua")

app:get("/", function()
  return { render = "home" }
end)

app:post(
  "/",
  capture_errors({
    on_error = function()
      return { render = "error", status = 500 }
    end,
    function(self)
      local body, status_code, _ = http.simple(config.api_url .. "/endpoint", {
        url = self.POST.url,
      })
      if status_code ~= 200 then
        yield_error("Unable to fetch data")
      end
      local data = cjson.decode(body)
      self.url = data["url"]
      self.custom_url = data["custom_url"]
      self.hash = data["hash"]
      return { render = "ready-url" }
    end,
  })
)

app:get(
  "/hits",
  capture_errors({
    on_error = function()
      return { render = "error", status = 500 }
    end,
    function(self)
      if not self.params.endpoint_hash then
        return { render = "hits" }
      end

      self.hash = self.params.endpoint_hash
      local body, status_code, _ = http.simple(config.api_url .. "/hits/" .. self.params.endpoint_hash)
      if status_code >= 500 then
        yield_error("Unable to fetch data")
      end
      if status_code == 404 then
        self.hits = {}
        return { render = "hits" }
      end
      local data = cjson.decode(body)
      self.hits = data.hits
      self.url = data.endpoint.url
      self.custom_url = data.endpoint.custom_url
      return { render = "hits" }
    end,
  })
)

return app
