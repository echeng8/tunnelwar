extends Node
#debug
var browser = true

 #localhost - 127.0.0.1
var ip = "" 
var port = 0

#implementation  
var my_name = "Client"
const world = preload("res://World/World.tscn")

#references 
var world_node  #set by World.tcsn when loaded
var user_player : Player setget _on_user_player_set

#signals
signal on_user_player_set
var client = null

func _ready():
	assert(get_tree().connect("connected_to_server", self, "_connected_ok") == 0)
	assert(get_tree().connect("connection_failed", self, "_connected_fail") == 0)
	assert(get_tree().connect("server_disconnected", self, "_server_disconnected") == 0)

func _process(delta):
	if browser:
		if client == null:
			return 
			
		if (client.get_connection_status() == NetworkedMultiplayerPeer.CONNECTION_CONNECTED ||
			client.get_connection_status() == NetworkedMultiplayerPeer.CONNECTION_CONNECTING):
			client.poll();
	else:
		pass

func connect_to_server():
	if browser:
		#websocket multiplayer 
		client = WebSocketClient.new();
		var url = "ws://172.88.99.190:" + str(port) 
		var error = client.connect_to_url(url, PoolStringArray(), true);
		get_tree().set_network_peer(client);
	else:
		var host = NetworkedMultiplayerENet.new()
		host.create_client(ip, port)
		get_tree().set_network_peer(host)



# Callback from SceneTree, called when connect to server
func _connected_ok():
	get_node("/root/Main").hide()
	get_tree().get_root().add_child(world.instance())
	rpc_id(1, "register_player", my_name)
	
remote func instance_nodes(node_names : Dictionary):
	for player_name in node_names["Players"]:
		world_node.instantiate_player(player_name) 
	for item in node_names["Items"]:
		pass #todo 

# Callback from SceneTree, called when server disconnect
func _server_disconnected():
	world_node.queue_free()
	get_node("/root/Main").show()

# Callback from SceneTree, called when unabled to connect to server
func _connected_fail():
	get_tree().set_network_peer(null) # Remove peer
	
func get_player(id) -> Player:
	return world_node.get_node("Players/" + str(id))
	
# SETTERS AND GETTERS
func _on_user_player_set(player_ref):
	user_player = player_ref
	emit_signal("on_user_player_set")
