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

return util
