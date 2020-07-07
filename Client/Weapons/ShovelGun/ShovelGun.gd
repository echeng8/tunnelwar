extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

#server set animation durations
remote var pull_dur = 1
remote var stab_dur = 1
remote var reset_dur = 1

onready var TweenNode = get_node("Tween")
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
		"PDefaultState":
			visible = true
		"PDeadState":
			visible = false  
			
remotesync func _pre_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", position, newPos, pull_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT)
	TweenNode.interpolate_callback(self, pull_dur + 0.1, "make_scared") #todo refactoor
	TweenNode.start()
	#todo refactoor
func make_scared():
	get_parent().get_node("PlayerSprites").switch_face("scared") 
	
remote func _stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", position, newPos, stab_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT)
	TweenNode.start()

remotesync func _after_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", position, newPos, reset_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT) #todo fix pull-back duration not actually working
	TweenNode.start()

	#TODO refactor this in player 
	get_parent().get_node("PlayerSprites").switch_face("normal") 

##SHOOTING STUFF ###############
remotesync func shoot():
	$Shovel.visible = false  
remotesync func reload():
	$Shovel.visible = true

## DEBUG
func check_for_command(message: String):
	if(message == "/toggle_pulled"):
		debug_pulled = not debug_pulled
