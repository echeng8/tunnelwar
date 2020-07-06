extends Node

####DEBUG VARIABLES####
var time_passed = 0
####/DEBUG VARIABLES####

# Default game port
const DEFAULT_PORT = 25565

# Max number of players
const MAX_PLAYERS = 12

enum game_phases {IN_PROGRESS, INTERIM}

# STATIC REFERENCES 
var game_phase = game_phases.INTERIM setget set_game_phase 

var world_node : WorldManager
var blocks_node : Blocks#child of World.tcsn, set on onready by Blocks node
var chatbox_node 
var broadcast_node : Broadcast

#variables
const block_size = 200.0

#signals
signal on_match_begin
signal on_match_end



func _ready():
	assert(get_tree().connect("network_peer_connected", self, "_player_connected") == OK)
	assert(get_tree().connect("network_peer_disconnected", self,"_player_disconnected") == OK)
	
	var host = NetworkedMultiplayerENet.new()
	host.create_server(DEFAULT_PORT, MAX_PLAYERS)
	get_tree().set_network_peer(host)

#### GAME PHASES
func set_game_phase(gp : int) -> void: 
	game_phase = gp
	
	if game_phase == game_phases.INTERIM:
		emit_signal("on_match_end")
	if game_phase == game_phases.IN_PROGRESS:
		emit_signal("on_match_begin")
	
# Callback from SceneTree, called when client connects
func _player_connected(_id):
	print("Client ", _id, " connected")


# Callback from SceneTree, called when client disconnects
func _player_disconnected(id):
		world_node.rpc("remove_player", id)
		
# Player management functions
remote func register_player(new_player_name):
	var caller_id = get_tree().get_rpc_sender_id()
	
	print("Client ", caller_id, " registered as ", new_player_name)
	
	world_node.instantiate_player(caller_id, new_player_name)
	world_node.spawn_everything_in(caller_id)

func get_player(id):
	return world_node.get_node("Players/" + str(id))

func get_players():
	return world_node.get_children()
