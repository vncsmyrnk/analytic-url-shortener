local util = {}

function util:time_ago_in_words(timestamp)
  local diff = os.time() - timestamp
  local seconds_in_minute = 60
  local seconds_in_hour = 3600
  local seconds_in_day = 86400

  if diff < seconds_in_minute then
    return diff .. " seconds ago"
  elseif diff < seconds_in_hour then
    return math.floor(diff / seconds_in_minute) .. " minutes ago"
  elseif diff < seconds_in_day then
    return math.floor(diff / seconds_in_hour) .. " hours ago"
  else
    return math.floor(diff / seconds_in_day) .. " days ago"
  end
end

function util:current_time_string()
  return os.date("%Y-%m-%d %H:%M:%S")
end

function util:time_string_to_timestamp(time_string)
  local pattern = "(%d+)%-(%d+)%-(%d+) (%d+):(%d+):(%d+)"
  local year, month, day, hour, min, sec = time_string:match(pattern)
  local timestamp = os.time({
    year = year,
    month = month,
    day = day,
    hour = hour,
    min = min,
    sec = sec,
  })
  return timestamp
end

function util:origin_host()
  local host = ngx.var.host
  return host
end

function util:origin_port()
  local port = ngx.var.server_port
  return port
end

return util
