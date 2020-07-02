extends Node

 #localhost - 127.0.0.1
var ip = "" #Local Host
var port = 0

# Signal to let GUI know whats up
signal connection_failed()
signal connection_succeeded()
signal server_disconnected()

var my_name = "Client"

#references 
var world_node #set by World.tcsn when loaded
var user_player : Player setget _on_user_player_set

#signals
signal on_user_player_set

func _ready():
	assert(get_tree().connect("connected_to_server", self, "_connected_ok") == 0)
	assert(get_tree().connect("connection_failed", self, "_connected_fail") == 0)
	assert(get_tree().connect("server_disconnected", self, "_server_disconnected") == 0)
	
	# Try to connect right away
	#connect_to_server()

func connect_to_server():
	var host = NetworkedMultiplayerENet.new()
	host.create_client(ip, port)
	get_tree().set_network_peer(host)


# Callback from SceneTree, called when connect to server
func _connected_ok():
	emit_signal("connection_succeeded")
	pre_start_game()
	
func pre_start_game():	
	# Load world
	get_node("/root/Main").hide()
	var world = load("res://World/World.tscn").instance()
	get_tree().get_root().add_child(world)

	rpc_id(1, "register_player", my_name)

# Callback from SceneTree, called when server disconnect
func _server_disconnected():
	get_node("/root/World").queue_free()
	get_node("/root/Main").show()
	emit_signal("server_disconnected")
	
	# Try to connect again
	#connect_to_server()


# Callback from SceneTree, called when unabled to connect to server
func _connected_fail():
	get_tree().set_network_peer(null) # Remove peer
	emit_signal("connection_failed")
	
	# Try to connect again
	#connect_to_server()
	
func get_player(id) -> Player:
	return world_node.get_node("Players/" + str(id))
	
# SETTERS AND GETTERS
func _on_user_player_set(player_ref):
	user_player = player_ref
	emit_signal("on_user_player_set")
