extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

#gameplay values 
export var stab_charge_time = 0.5 #seconds you need to pull back in order to stab
export var vulnerability_time = 0.75 #seconds you are vulnerable after the stab TODO
export var slowed_move_rate = .5 #for player

#pull back animation 
export var pull_back_dist = -2500

#stabbing animation
export var stabbing_dist = 5000
export var init_position = Vector2(-10,17)

#animation duration (how long it takes for the anim to complete)
export var pull_dur = 0.1
export var stab_dur = 0.08
export var reset_dur = 0.15

#implementation
var breaking_blocks : bool 
var current_cell : Vector2

#Player-Node Set Variables - only on PDefaultState
var input_aim_pos = Vector2(0,0) 
var input_pull_jp = false # pull button just pressed

remote func initialize_rpc_sender(): 
	rset_id(get_tree().get_rpc_sender_id(), "pull_dur", pull_dur)
	rset_id(get_tree().get_rpc_sender_id(), "stab_dur", stab_dur)
	rset_id(get_tree().get_rpc_sender_id(), "reset_dur", reset_dur)

var player : Player
var velocity = Vector2.ZERO
var newPos = Vector2.ZERO	

func _ready():
	player = HelperFunctions.get_parent_player_node(self)
	
func _process(_delta):
	rpc("server_set_transform", global_rotation, global_position)

func point_to(destination: Vector2, rotate_percent = 1):
	global_rotation += to_local(destination).angle() * rotate_percent

######ANIMATION FUNCTIONS to be called by states (todo put in states?)
func _pre_stabbing(_currPos, newPos):
	$Tween.interpolate_property(self, "position", position, newPos, pull_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT)
	$Tween.start()

func _stabbing(currPos, newPos):
	$Tween.interpolate_property(self, "position", position, newPos, stab_dur)
	$Tween.start()
	
func _after_stabbing(currPos, newPos):
	$Tween.interpolate_property(self, "position", position, newPos, reset_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT) #todo fix pull-back duration not actually working
	$Tween.start()

#legacy 	
#remotesync func _pre_stabbing(_currPos, newPos):
#	$Tween.interpolate_property(self, "position", position, newPos, pull_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT)
#	$Tween.start()
#
#remotesync func _stabbing(currPos, newPos):
#	$Tween.interpolate_property(self, "position", position, newPos, stab_dur)
#	$Tween.start()
#
#remotesync func _after_stabbing(currPos, newPos):
#	$Tween.interpolate_property(self, "position", position, newPos, reset_dur, Tween.TRANS_LINEAR) #, Tween.EASE_OUT) #todo fix pull-back duration not actually working
#	$Tween.start()


		
##SHOOTING STUFF #########################################################
remotesync func shoot():
	$Reload.start()
	var ShovelNode = get_node("Shovel")
	gamestate.world_node.add_item(ShovelNode)
	ShovelNode.get_node("StateMachine").call_deferred("change_to", "ShShotState")

remotesync func reload():
	var shovel = Shovel.instance()
	call_deferred("add_child", shovel)

func is_loaded(): 
	return has_node("Shovel")
	
func _on_Reload_timeout():
	$Reload.stop()
	if not is_loaded():
		rpc("reload")
	


