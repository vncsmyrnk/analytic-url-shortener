local config = require("lapis.config")

config("development", {
  server = "nginx",
  code_cache = "off",
  num_workers = "1",
  port = 8081,
  api_url = os.getenv("API_URL") or "http://127.0.0.1:8080",
})

config("production", {
  server = "nginx",
  code_cache = "on",
  num_workers = "5",
  port = 80,
})
