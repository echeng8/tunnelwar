extends Node

####DEBUG VARIABLES####
const DEV_SPAWN_X = 0
const DEV_SPAWN_Y = 0
var time_passed = 0
####/DEBUG VARIABLES####

# Default game port
const DEFAULT_PORT = 25525

# Max number of players
const MAX_PLAYERS = 12

# STATIC REFERENCES 
var world_node

#variables
const block_size = 200.0


func _ready():
	world_node = get_node("/root/World")\
	
	get_tree().connect("network_peer_connected", self, "_player_connected")
	get_tree().connect("network_peer_disconnected", self,"_player_disconnected")
	
	var host = NetworkedMultiplayerENet.new()
	host.create_server(DEFAULT_PORT, MAX_PLAYERS)
	get_tree().set_network_peer(host)

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
	
	get_node("/root/World").rpc("spawn_player", Vector2(DEV_SPAWN_X, DEV_SPAWN_Y), caller_id, new_player_name)
	get_node("/root/World").spawn_everything_in(caller_id)
	
# Return random 2D vector inside bounds 0, 0, bound_x, bound_y
func random_vector2(bound_x, bound_y):
	return Vector2(randf() * bound_x, randf() * bound_y)

func get_player(id):
	return get_node("/root/World/Players/" + str(id))

func get_players():
	return get_node("/root/World/Players/").get_children()
	
# COORDINATE SYSTEM
func get_pos(coord_vector : Vector2):
	print("p", Vector2(coord_vector.x * block_size, coord_vector.y * block_size))
	return Vector2(coord_vector.x * block_size, coord_vector.y * block_size)
	
func get_coord(pos_vector : Vector2):
	print("c",Vector2(round(pos_vector.x / block_size), round(pos_vector.y / block_size)) )
	return Vector2(round(pos_vector.x / block_size), round(pos_vector.y / block_size))
