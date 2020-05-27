extends Node

####DEBUG VARIABLES####
const DEV_SPAWN_X = 500
const DEV_SPAWN_Y = 500
####/DEBUG VARIABLES####

# Default game port
const DEFAULT_PORT = 25525

# Max number of players
const MAX_PLAYERS = 12

# Players dict stored as id:name
var players = {}


func _ready():
	get_tree().connect("network_peer_connected", self, "_player_connected")
	get_tree().connect("network_peer_disconnected", self,"_player_disconnected")
	
	create_server()


func create_server():
	var host = NetworkedMultiplayerENet.new()
	host.create_server(DEFAULT_PORT, MAX_PLAYERS)
	get_tree().set_network_peer(host)


# Callback from SceneTree, called when client connects
func _player_connected(_id):
	print("Client ", _id, " connected")


# Callback from SceneTree, called when client disconnects
func _player_disconnected(id):
	if players.has(id):
		get_node("/root/World").rpc("remove_player", id)
		
		players.erase(id)
		rset("players", players)
	
	print("Client ", id, " disconnected")


# Player management functions
remote func register_player(new_player_name):
	var caller_id = get_tree().get_rpc_sender_id()
	players[caller_id] = new_player_name
	rset("players", players) 
	
	print("Client ", caller_id, " registered as ", new_player_name)
	
	get_node("/root/World").rpc("spawn_player", Vector2(DEV_SPAWN_X, DEV_SPAWN_Y), caller_id)
	get_node("/root/World").spawn_everything_in(caller_id)

	
# Return random 2D vector inside bounds 0, 0, bound_x, bound_y
func random_vector2(bound_x, bound_y):
	return Vector2(randf() * bound_x, randf() * bound_y)


func get_player_info(id):
	return get_node("/root/World/Players/" + str(id))
