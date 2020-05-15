extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var player

func enter():
	player = get_parent().get_parent()
	assert(player is KinematicBody2D) 
	
	player.connect("struck_by", self, "_on_struck_by")
	
func physics_process(delta):
	#TODO change speed based on shovelgun 
	var isShovelgunAttacking = not player.shovelgun.find_node("StateMachine").state.name == "SGDefaultState"
	var velocity = player.input_direction.normalized() * player.speed
	if isShovelgunAttacking:
		velocity = velocity * player.shovelgun.slowed_move_rate  
		
	player.move_and_slide(velocity, Vector2(0,0))
	player.rpc_unreliable("set_player_position", player.position)
		
func _on_struck_by(source):
	if "damage" in source: 
		player.health_points -= source.damage
		player.rpc("set_health", player.health_points)
		if player.health_points <= 0:
			fsm.change_to("PDeadState")
	if "knockback_speed" in source: #todo make speed or duration agnostic?  
		var kb_state = get_node("../PKnockbackedState")
		kb_state.knockback_source = source
		fsm.change_to("PKnockbackedState")
