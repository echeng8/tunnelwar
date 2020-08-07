local world_control = {}

local nk = require("nakama")

-- Custom operation codes. Nakama specific codes are <= 0.
local OpCodes = {
    update_position = 1,
    update_input = 2,
    update_state = 3,
    do_spawn = 4,
    initial_state = 5
}

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

-- Command pattern table for boiler plate updates that uses data and state.
local commands = {}

commands[OpCodes.do_spawn] = function(data, state)
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

-- Called `tickrate` times per second. Handles client messages and sends game state updates. Uses
-- boiler plate commands from the command pattern except when specialization is required.
function world_control.match_loop(_, dispatcher, _, state, messages)
    for _, message in ipairs(messages) do
        local op_code = message.op_code

        local decoded = nk.json_decode(message.data)

        -- Run boiler plate commands (state updates.)
        local command = commands[op_code]
        if command ~= nil then
            commands[op_code](decoded, state)
        end

        -- A client has selected a character and is spawning. Get or generate position data,
        -- send them initial state, and broadcast their spawning to existing clients.
        if op_code == OpCodes.do_spawn then
            state.names[message.sender.user_id] = decoded.nm

            local data = {
                ["nms"] = state.names
            }

            local encoded = nk.json_encode(data)
            dispatcher.broadcast_message(OpCodes.initial_state, encoded, {message.sender})

            dispatcher.broadcast_message(OpCodes.do_spawn, message.data)
        end
    end
    
    return state
end

-- Server is shutting down. Save positions of all existing characters to storage.
function world_control.match_terminate(_, _, _, state, _)
    return state
end



return world_control