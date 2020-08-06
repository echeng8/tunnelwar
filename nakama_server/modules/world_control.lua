local world_control = {}

local nk = require("nakama")

-- When the match is initialized. Creates empty tables in the game state that will be populated by
-- clients.
function world_control.match_init(_, _)
    local gamestate = {
        presences = {},
        inputs = {},
        positions = {},
        names = {}
    }
    local tickrate = 10
    local label = "Shovelgun Match"
    return gamestate, tickrate, label
end

-- When someone tries to join the match. Checks if someone is already logged in and blocks them from
-- doing so if so.
function world_control.match_join_attempt(_, _, _, state, presence, _)
    if state.presences[presence.user_id] ~= nil then
        return state, false, "User already logged in."
    end
    return state, true
end

-- When someone does join the match. Initializes their entries in the game state tables with dummy
-- values until they spawn in.
function world_control.match_join(_, dispatcher, _, state, presences)
    for _, presence in ipairs(presences) do
        state.presences[presence.user_id] = presence

        state.positions[presence.user_id] = {
            ["x"] = 0,
            ["y"] = 0
        }

        state.inputs[presence.user_id] = {
            ["dir"] = 0,
        }

        state.names[presence.user_id] = "User"
    end

    return state
end


-- When someone leaves the match. Clears their entries in the game state tables, but saves their
-- position to storage for next time.
function world_control.match_leave(_, _, _, state, presences)
    for _, presence in ipairs(presences) do
        state.presences[presence.user_id] = nil
        state.positions[presence.user_id] = nil
        state.inputs[presence.user_id] = nil
        state.names[presence.user_id] = nil
    end
    return state
end

-- Called `tickrate` times per second. Handles client messages and sends game state updates. Uses
-- boiler plate commands from the command pattern except when specialization is required.
function world_control.match_loop(_, dispatcher, _, state, messages)
    return state
end

-- Server is shutting down. Save positions of all existing characters to storage.
function world_control.match_terminate(_, _, _, state, _)
    return state
end



return world_control