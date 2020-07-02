extends State

var player

func enter():
	player = fsm_root
	assert(player is Player) 
	
	player.connect("struck_by", self, "_on_struck_by")

func process(delta):
	#pass input to Shovelgun
	fsm_root.get_node("ShovelGun").input_aim_pos = fsm_root.input_aim_pos
	fsm_root.get_node("ShovelGun").input_pull_jp = fsm_root.input_pull_jp

func physics_process(delta):
	#TODO change speed based on ShovelGun 
	var isSGAtRest = player.get_node("ShovelGun").find_node("StateMachine").state.name == "SGDefaultState"
	var velocity = player.input_direction.normalized() * player.speed
	if not isSGAtRest:
		velocity = velocity * player.get_node("ShovelGun").slowed_move_rate  
		
	player.move_and_slide(velocity, Vector2(0,0))
	player.rpc_unreliable("set_player_position", player.position)
		
func _on_struck_by(source):
	#only damaged when game is in-progress
	if "damage" in source and gamestate.game_phase == gamestate.game_phases.IN_PROGRESS: 
		player.health_points -= source.damage
		player.rpc("set_health", player.health_points)
		if player.health_points <= 0:
			exit("PDeadState")
			return 
			
	if "knockback_speed" in source: #todo make speed or duration agnostic?  
		var kb_state = get_node("../PKnockbackedState")
		kb_state.knockback_source = source
		exit("PKnockbackedState")
		
func exit(next_state):
	player.disconnect("struck_by", self, "_on_struck_by")
	fsm.change_to(next_state) 
