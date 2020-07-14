extends Node2D

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

#gameplay values 
export var pull_time = 0.15 #seconds you need to pull back in order to stab
export var shoot_charge_time = 2 #time it takes once pulled to charge up shoot 

export var vulnerability_time = 0.75 #seconds you are vulnerable after the stab TODO
export var slowed_move_rate = .5 #for player


#pull back animation 
export var pull_back_dist = -100

#stabbing animation
export var stabbing_dist = 30
var init_position : Vector2

#animation duration (how long it takes for the anim to complete)
export var stab_dur = 0.08
export var reset_dur = 0.15

#implementation
var breaking_blocks : bool 
var current_cell : Vector2
var player : Player

#Player-Node Set Variables - only on PDefaultState
var input_aim_pos = Vector2(0,0) 
var input_pull_p = false # pull button just pressed

remote func initialize_rpc_sender(): 
	#not currently in use
#	rset_id(get_tree().get_rpc_sender_id(), "stab_dur", stab_dur)
#	rset_id(get_tree().get_rpc_sender_id(), "reset_dur", reset_dur)
	
	#implementation values 
	rset_id(get_tree().get_rpc_sender_id(), "shoot_charge_time", shoot_charge_time)
	rset_id(get_tree().get_rpc_sender_id(), "pull_time", pull_time)
	
func _ready():
	assert($StateMachine.connect("on_state_change", self, "update_client_state") == OK) 
	
	player = HelperFunctions.get_parent_player_node(self)
	init_position = position
	
func _process(_delta):
	rpc_unreliable("server_set_transform", global_rotation, global_position)

func update_client_state() -> void:
	rpc("update_client_state", $StateMachine.state.name)

######ANIMATION FUNCTIONS to be called by states (todo put in states?)
func move_to(destination : Vector2, duration : float, callback = ""):
	$Tween.interpolate_property(self, "position", null, destination, duration)
	if not callback == "":
		$Tween.interpolate_callback($StateMachine.state, duration, callback) 
	$Tween.start()
	
func point_to(destination: Vector2, rotate_percent = 1):
	get_parent().global_rotation += get_parent().to_local(destination).angle() * rotate_percent

##SHOOTING STUFF #########################################################
func is_loaded(): 
	return has_node("Shovel")
	
remotesync func shoot():
	$Reload.start()
	var ShovelNode = get_node("Shovel")
	gamestate.world_node.add_item(ShovelNode)
	ShovelNode.get_node("StateMachine").call_deferred("change_to", "ShShotState")

remotesync func reload():
	var shovel = Shovel.instance()
	call_deferred("add_child", shovel)

func _on_Reload_timeout():
	$Reload.stop()
	if not is_loaded():
		rpc("reload")
	


