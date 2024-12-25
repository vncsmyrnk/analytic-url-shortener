local lapis = require("lapis")
local util = require("util")
local uuid = require("_uuid")
local json_params = require("lapis.application").json_params
local models = require("_models")

local Endpoints = models.Endpoints
local Hits = models.Hits

local app = lapis.Application()

app:get("/r/:hash", function(self)
  local endpoint = Endpoints:find({ hash = self.params.hash })
  if not endpoint then
    return { status = 404, layout = false }
  end
  local new_hit = {
    hit_time = util:current_time_string(),
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
  endpoint["custom_url"] =
    self:build_url("/r/" .. endpoint["hash"], { host = util:origin_host(), port = util:origin_port() })
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

app:get("/hits/:endpoint_hash", function(self)
  local endpoint = Endpoints:find({ hash = self.params.endpoint_hash })
  if not endpoint then
    return { status = 404, layout = false }
  end
  local hits = Hits:find_all({ endpoint.id }, "endpoint_id")
  if #hits == 0 then
    return { status = 404, layout = false }
  end
  for _, hit in ipairs(hits) do
    local hit_timestamp = util:time_string_to_timestamp(hit["hit_time"])
    hit["last_hit_desc"] = util:time_ago_in_words(hit_timestamp)
  end
  endpoint["custom_url"] =
    self:build_url("/r/" .. endpoint["hash"], { host = util:origin_host(), port = util:origin_port() })
  return { json = { endpoint = endpoint, hits = hits } }
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
    endpoint["custom_url"] =
      self:build_url("/r/" .. endpoint["hash"], { host = util:origin_host(), port = util:origin_port() })
    return { json = endpoint }
  end)
)

function app:handle_error(err, trace)
  ngx.log(ngx.ERR, "Error: ", err)
  ngx.log(ngx.ERR, "Trace: ", trace)

  return {
    status = 500,
    json = {
      message = "An unexpected error occurred.",
      error = err,
      trace = trace,
    },
  }
end

return app
