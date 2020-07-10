extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

var server_state : String

#debug variables
var debug_pulled = false

func _ready():
	var shovel = get_node("Shovel")
	rpc_id(1, "initialize_rpc_sender")  
	
	#signal connnections  
	gamestate.world_node.get_node("ScreenHUD/ChatBox").connect("on_message_send", self, "check_for_command")

remote func server_set_transform(rot, pos):
	global_rotation = rot
	global_position = pos 

###ANIMATION ##################
remote func update_client_state(s_state : String):
	server_state = s_state
	emit_signal("on_server_state_change", server_state)
	match server_state:
		"SGDefaultState":
			HelperFunctions.get_parent_player_node(self).get_node("PlayerSprites").switch_face("normal")
		"SGPulledState":
			HelperFunctions.get_parent_player_node(self).get_node("PlayerSprites").switch_face("scared")
		"SGStabState":
			pass
		"SGVulnerableState": 
			pass 

##SHOOTING STUFF ###############
remotesync func shoot():
	$Shovel.visible = false  
remotesync func reload():
	$Shovel.visible = true

## DEBUG
func check_for_command(message: String):
	if(message == "/toggle_pulled"):
		debug_pulled = not debug_pulled
