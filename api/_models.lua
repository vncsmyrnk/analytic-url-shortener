local Model = require("lapis.db.model").Model

local models = {}

local Endpoints = Model:extend("endpoints", { timestamp = true })
local Hits = Model:extend("hits")

models.Endpoints = Endpoints
models.Hits = Hits

return models
