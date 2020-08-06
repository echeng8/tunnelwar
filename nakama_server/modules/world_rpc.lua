local nakama = require("nakama")

-- Finds the ID of the first match in the listings. If no match is found, creates one.
-- @return The ID of the match found or created.
local function _get_first_world()
    local matches = nakama.match_list()
    local current_match = matches[1]

    if current_match == nil then
        return nakama.match_create("world_control", {})
    else
        return current_match.match_id
    end
end

-- Returns the ID of the world match so users can join it.
-- @return ID as a string
local function get_world_id(_, _)
    return _get_first_world()
end


nakama.register_rpc(get_world_id, "get_world_id")