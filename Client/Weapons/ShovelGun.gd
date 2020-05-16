extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

signal shoot

var player_id


#server set animation durations
remote var pull_dur = 1
remote var stab_dur = 1
remote var reset_dur = 1

onready var TweenNode = get_node("Tween")

func _ready():
	var shovel = get_node("Shovel" + player_id)
	shovel.connect("_pick_up", self, "_on_shovel_pick_up")
	print(shovel.name)

func setup():
	player_id = get_parent().name
	name = name + player_id
	$Shovel.setup()

func _process(delta):
	if is_network_master():
		#set variables on server
		rset_unreliable_id(1, "mousepos", get_global_mouse_position()) #todo check for cheating potential 
		rset_id(1, "stab_btn_p", Input.is_action_pressed('stab'))
		rset_unreliable_id(1, "shoot_btn_p", Input.is_action_pressed('shoot')) 

remote func _update_weapon_position(player_id, mouse_position):
	if self.player_id == player_id:
		look_at(mouse_position)
		
remotesync func _pre_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", self.position, newPos, pull_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	
remote func _stabbing(player_id, currPos, newPos):
	if self.player_id == player_id:
		TweenNode.interpolate_property(self, "position", self.position, newPos, stab_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
		TweenNode.start()
		
		get_parent().get_node("VulBod/exclam_mark").visible = true #TODO make conditional based on state 
		get_parent().get_node("VulBod/norm_face").visible = false
		get_parent().get_node("VulBod/vul_face").visible = true

remotesync func _after_stabbing(player_id, currPos, newPos):
	if self.player_id == player_id:
		TweenNode.interpolate_property(self, "position", currPos, newPos, reset_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT) #todo fix pull-back duration not actually working
		TweenNode.start()
		
		get_parent().get_node("VulBod/exclam_mark").visible = false
		get_parent().get_node("VulBod/norm_face").visible = true
		get_parent().get_node("VulBod/vul_face").visible = false

##SHOOTING STUFF ###############
remote func reparent_shvl_to_projectiles():
	var ShovelNode = get_node("Shovel"+player_id)
	HelperFunctions.reparent(ShovelNode, get_node("/root/World/Projectiles"), true)
	
func _on_shovel_pick_up (player_id):
	if self.player_id == player_id:
		var shovel = Shovel.instance()
		call_deferred("add_child", shovel)
		shovel.call_deferred("setup")

remotesync func _reload():
		var shovel = Shovel.instance()
		add_child(shovel)
		shovel.setup()
