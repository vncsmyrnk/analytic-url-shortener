local schema = require("lapis.db.schema")
local types = schema.types

return {
  [1734904670] = function()
    schema.create_table("endpoints", {
      { "id", types.serial({ primary_key = true }) },
      { "name", types.varchar },
      { "url", types.varchar },
      { "hash", types.varchar },
      { "created_at", types.time },
      { "updated_at", types.time },
    })
  end,
  [1734906046] = function()
    schema.create_table("hits", {
      { "id", types.serial({ primary_key = true }) },
      { "ip", types.varchar },
      { "hit_time", types.time },
      { "endpoint_id", types.foreign_key },
      { "user_agent", types.varchar },
    })
  end,
}
