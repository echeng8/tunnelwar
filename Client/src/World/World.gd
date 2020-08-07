extends Node2D

export var PlayerScene: PackedScene
export var CharacterScene: PackedScene

var characters := {}

onready var screen_hud := $ScreenHUD

#temp 
var names = {}

#RESOURCES LOAD

#old shit
var load_completed = false
const resources = {
	#scenes
	#"Shovel": preload("res://Weapons/Shovel/Shovel.tscn"), 
}
signal on_load_complete

func _ready():
	#warning-ignore: return_value_discarded
	ServerConnection.connect(
		"initial_state_received", self, "_on_ServerConnection_initial_state_received"
	)


### PLAYER STUFF 

func _on_ServerConnection_initial_state_received(
	names: Dictionary
) -> void:
	#warning-ignore: return_value_discarded
	ServerConnection.disconnect(
		"initial_state_received", self, "_on_ServerConnection_initial_state_received"
	)
	join_world(names)

# The main entry point. Sets up the client player and the various characters that
# are already logged into the world, and sets up the signal chain to respond to
# the server.
func join_world(
	state_names: Dictionary
) -> void:
	var user_id := ServerConnection.get_user_id()
	var username: String = state_names.get(user_id)
	
	self.names = state_names  
	print('NAMES', names)

	#warning-ignore: return_value_discarded
	ServerConnection.connect(
		"chat_message_received", self, "_on_ServerConnection_chat_message_received"
	)
	#warning-ignore: return_value_discarded
	ServerConnection.connect("character_spawned", self, "_on_ServerConnection_character_spawned")
	

func _on_ServerConnection_chat_message_received(sender_id: String, message: String) -> void:
	var sender_name := "User"
	if sender_id in characters:
		sender_name = characters[sender_id].username
	elif sender_id == ServerConnection.get_user_id():
		sender_name = "myself"

	screen_hud.add_chat_reply(message, sender_name)

func _on_ServerConnection_character_spawned(id: String, color: Color, name: String) -> void:
	if id in characters:
		characters[id].username = name


#old shit below 

	
puppet func emit_load_complete():
	load_completed = true  
	emit_signal("on_load_complete") 
	
#remotesync func instantiate_player(spawn_pos, id, username):
#	if $Players.has_node(str(id)):
#		return 
#
#	var player = Player.instance()
#	player.position = spawn_pos
#	player.name = String(id) # Important
#	player.set_network_master(id) # Important
#	player.username = username 
#	$Players.add_child(player)



### BLOCKS AND SHOVELS 
remote func spawn(file_name, node_name, transform_dict):
	if node_name in $Items.get_children():
		return
		
	var instancedThing = resources[file_name].instance()
	instancedThing.name = node_name
	
	if file_name == "Block":
		$Blocks.add_child(instancedThing) 
	if file_name == "Shovel":
		$Items.add_child(instancedThing) 
	
	instancedThing.global_position = transform_dict["pos"]
	instancedThing.global_rotation = transform_dict["rot"]
	instancedThing.global_scale = transform_dict["sca"]

func _on_ScreenHUD_text_sent(text) -> void:
	ServerConnection.send_text_async(text)
