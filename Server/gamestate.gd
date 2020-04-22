extends Node

# Default game port
const DEFAULT_PORT = 44444

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
	#if Server.is_peer_connected(id):
		_chat_box_notify_disconnected(id)
		rpc("unregister_player", id)
		get_node("/root/World").rpc("remove_player", id)
	
	print("Client ", id, " disconnected")

func _chat_box_notify_disconnected(id):
	var world = get_node("/root/World")
	
	#Tell existing clients that player disconnected...
	var dcName = str(players[id])
	var message = "Player disconnected: " + dcName
	if dcName == "Machineman1357":
		message = "Our supreme [color=purple]Lead Artist[/color] [color=green]Machineman1357[/color] has departed!"
	for p_id in players:
		if p_id != id:
			world.rpc_id(p_id, "_chat_message", message)

# Player management functions
remote func register_player(new_player_name):
	# We get id this way instead of as parameter, to prevent users from pretending to be other users
	var caller_id = get_tree().get_rpc_sender_id()
	
	#Server.register_player(caller_id, new_player_name)
	
	# Add him to our list
	players[caller_id] = new_player_name
	
	# Add everyone to new player:
	for p_id in players:
		rpc_id(caller_id, "register_player", p_id, players[p_id]) # Send each player to new dude
	
	rpc("register_player", caller_id, players[caller_id]) # Send new dude to all players
	# NOTE: this means new player's register gets called twice, but fine as same info sent both times
	
	print("Client ", caller_id, " registered as ", new_player_name)


puppetsync func unregister_player(id):
	players.erase(id)
	#Server.unregister_player(id)
	
	print("Client ", id, " was unregistered")


remote func populate_world():
	var caller_id = get_tree().get_rpc_sender_id()
	var world = get_node("/root/World")
	
	# Spawn all current players on new client
	for player in world.get_node("Players").get_children():
		world.rpc_id(caller_id, "spawn_player", player.position, player.get_network_master())
	
	# Spawn new player everywhere
	world.rpc("spawn_player", random_vector2(500, 500), caller_id)
	
	_chat_box_notify_connection(caller_id)

func _chat_box_notify_connection(caller_id):
	var world = get_node("/root/World")
	
	#Welcome the new player...
	world.rpc_id(caller_id, "_chat_message", "Welcome to [color=red]Shovelgun[/color], " + str(players[caller_id]))
	
	#Tell existing clients of new client's arrival...
	var callerName = str(players[caller_id])
	var message = "Player joined: " + callerName
	if callerName == "Machineman1357":
		message = "Our supreme [color=purple]Lead Artist[/color] " + "[color=green]Machineman1357[/color] " + "has arrived!"
	for p_id in players:
		if p_id != caller_id:
			world.rpc_id(p_id, "_chat_message", message)

# Return random 2D vector inside bounds 0, 0, bound_x, bound_y
func random_vector2(bound_x, bound_y):
	return Vector2(randf() * bound_x, randf() * bound_y)


func get_player_info(id):
	return get_node("/root/World/Players/" + str(id))



