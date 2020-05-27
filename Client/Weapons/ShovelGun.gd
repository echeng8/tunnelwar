extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

signal shoot
#server set animation durations
remote var pull_dur = 1
remote var stab_dur = 1
remote var reset_dur = 1

onready var TweenNode = get_node("Tween")

func _ready():
	var shovel = get_node("Shovel")

func _process(delta):
	if is_network_master():
		#set variables on server
		rset_unreliable_id(1, "mousepos", get_global_mouse_position()) #todo check for cheating potential 
		rset_id(1, "stab_btn_p", Input.is_action_pressed('stab'))
		rset_unreliable_id(1, "shoot_btn_p", Input.is_action_pressed('shoot')) 

remote func _update_weapon_position(mouse_position):
		look_at(mouse_position)

###ANIMATION ##################
remotesync func _pre_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", position, newPos, pull_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	
remote func _stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", position, newPos, stab_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	
	get_parent().get_node("VulBod/exclam_mark").visible = true #TODO make conditional based on state 
	get_parent().get_node("VulBod/norm_face").visible = false
	get_parent().get_node("VulBod/vul_face").visible = true

remotesync func _after_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", position, newPos, reset_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT) #todo fix pull-back duration not actually working
	TweenNode.start()
	
	get_parent().get_node("VulBod/exclam_mark").visible = false
	get_parent().get_node("VulBod/norm_face").visible = true
	get_parent().get_node("VulBod/vul_face").visible = false

##SHOOTING STUFF ###############
remotesync func shoot():
	$Shovel.visible = false  
remotesync func reload():
	$Shovel.visible = true
	#todo toggle shovel visible
