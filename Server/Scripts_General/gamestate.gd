extends Node

####DEBUG VARIABLES####
var browser = true
 
####/DEBUG VARIABLES####

# Default game port
const PORT = 25565

# Max number of players
const MAX_PLAYERS = 12

enum game_phases {IN_PROGRESS, INTERIM}

# STATIC REFERENCES 
var game_phase = game_phases.INTERIM setget set_game_phase 

var world_node : WorldManager
var blocks_node : Blocks #child of World.tcsn, set on onready by Blocks node
var chatbox_node 
var broadcast_node : Broadcast

#variables
const block_size = 200.0

#implementation
var players = [] 
var player_names = {}

#signals
signal on_match_begin
signal on_match_end
signal on_player_id_set(index, id)
signal on_player_remove(id) 

var server 

func _ready():
	assert(get_tree().connect("network_peer_disconnected", self, "remove_player") == OK)
	
	#players array, null = free, id = occupied, index = player node count
	for _c in range(MAX_PLAYERS):
		players.append(null) 
	
	if browser:
		#webgl multiplayer
		server = WebSocketServer.new();
		server.listen(PORT, PoolStringArray(), true);
		get_tree().set_network_peer(server)
	else: 
	#enet multiplayer
		var host = NetworkedMultiplayerENet.new()
		host.create_server(PORT, MAX_PLAYERS)
		get_tree().set_network_peer(host)
	
func _process(_delta):
	if browser and server.is_listening(): # is_listening is true when the server is active and listening
		server.poll();

remote func register_player(name : String) -> void: 
	var caller_id = get_tree().get_rpc_sender_id() 	
	
	#set player_names dict
	player_names[caller_id] = name
	
	set_player_master(get_open_player_index(), caller_id) 
	rpc_id(caller_id, "instance_nodes", world_node.get_instance_nodes()) 

func remove_player(id : int) -> void: 
	var index = get_player_index(id)
	players[index] = null  
	emit_signal("on_player_remove", index) 

#### GAME PHASES
func set_game_phase(gp : int) -> void: 
	game_phase = gp
	
	if game_phase == game_phases.INTERIM:
		emit_signal("on_match_end")
	if game_phase == game_phases.IN_PROGRESS:
		emit_signal("on_match_begin")

func get_player(id):
	return world_node.get_node("Players").get_player(get_player_index(id))

#HELPER FUNCTIONS
func get_open_player_index() -> int:
	for i in range(players.size()): 
		if players[i] == null:
			return i
	return -1

func get_player_index(id: int) -> int:
	for i in range(players.size()):
		if players[i] == id:
			return i
	return -1 

func set_player_master(index : int, id: int) -> void: 
	players[index] = id
	emit_signal("on_player_id_set", index, id) 
