extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

#server set animation durations
remote var pull_dur = 1
remote var stab_dur = 1
remote var reset_dur = 1

onready var TweenNode = get_node("Tween")

#debug variables
var debug_pulled = false 

func _ready():
	var shovel = get_node("Shovel")
	rpc_id(1, "initialize_rpc_sender")  
	
	#signal connnections  
	gamestate.world_node.get_node("ScreenHUD/ChatBox").connect("on_message_send", self, "check_for_command")

remote func _update_weapon_position(mouse_position):
		look_at(mouse_position)

###ANIMATION ##################
remotesync func _pre_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", position, newPos, pull_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT)
	TweenNode.start()
	
remote func _stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", position, newPos, stab_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT)
	TweenNode.start()
	
	get_parent().get_node("VulBod/exclam_mark").visible = true #TODO make conditional based on state 
	get_parent().get_node("VulBod/norm_face").visible = false
	get_parent().get_node("VulBod/vul_face").visible = true

remotesync func _after_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", position, newPos, reset_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT) #todo fix pull-back duration not actually working
	TweenNode.start()
	
	get_parent().get_node("VulBod/exclam_mark").visible = false
	get_parent().get_node("VulBod/norm_face").visible = true
	get_parent().get_node("VulBod/vul_face").visible = false

##SHOOTING STUFF ###############
remotesync func shoot():
	$Shovel.visible = false  
remotesync func reload():
	$Shovel.visible = true

## DEBUG
func check_for_command(message: String):
	if(message == "/toggle_pulled"):
		debug_pulled = not debug_pulled
