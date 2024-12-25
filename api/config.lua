local config = require("lapis.config")

config("development", {
  server = "nginx",
  code_cache = "off",
  num_workers = "1",
  port = 8080,
  postgres = {
    host = "127.0.0.1",
    user = "pg",
    password = "cGFzc3dvcmQ=",
    database = "analytical-shortener",
  },
})

config("production", {
  server = "nginx",
  code_cache = "on",
  num_workers = "5",
  port = 80,
})
