extends State

func enter():
	fsm_root.connect("struck_by", self, "_on_struck_by")

func process(_delta):
	#pass input to Shovelgun
	fsm_root.get_node("ShovelGun/Model").input_aim_pos = fsm_root.input_aim_pos
	fsm_root.get_node("ShovelGun/Model").input_pull_p = fsm_root.input_pull_p

func physics_process(_delta):
	#TODO change speed based on ShovelGun 
	pass
#	var isSGAtRest = fsm_root.get_node("ShovelGun/Model").find_node("StateMachine").state.name == "SGDefaultState"
#	fsm_root.velocity = fsm_root.input_direction.normalized() * fsm_root.speed
#	if not isSGAtRest:
#		fsm_root.velocity = fsm_root.velocity * fsm_root.get_node("ShovelGun/Model").slowed_move_rate  

	fsm_root.move_and_slide(fsm_root.velocity, Vector2(0,0))
		
func _on_struck_by(source):# : Shovel):
	pass
#	#only damaged when game is in-progress
#	if gamestate.game_phase == gamestate.game_phases.IN_PROGRESS: 
#		player.health_points -= source.damage
#		player.rpc("set_health", player.health_points)
#
#		if player.health_points <= 0:
#			fsm_root.broadcast_death(source.last_owner_id)
#			exit("PDeadState")
#			return 
#
#	if "knockback_speed" in source: #todo make speed or duration agnostic?  
#		var kb_state = get_node("../PKnockbackedState")
#		kb_state.knockback_source = source #todo no moore references fuck references
#		exit("PKnockbackedState")
		
func exit(next_state):
	fsm_root.disconnect("struck_by", self, "_on_struck_by")
	.exit(next_state)
