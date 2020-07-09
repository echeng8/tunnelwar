extends State

var stuck_position = Vector2.INF #infinity = null 

func enter():
	if fsm_root.is_loaded():
		fsm_root.get_node("Shovel").get_node("StateMachine").call_deferred("change_to", "ShDefaultState")
	if not fsm_root.position == fsm_root.init_position:
		fsm_root.move_to(fsm_root.init_position, fsm_root.reset_dur)
# Optional handler functions for game loop events
func process(_delta):
	#shovelgun stuck in dirt logic 
	var point = fsm_root.input_aim_pos
	if fsm_root.is_loaded():
		if stuck_position == Vector2.INF and fsm_root.get_node("Shovel").get_buried_percent() == 1:
			stuck_position = fsm_root.get_node("Shovel").global_position 
			
		if fsm_root.get_node("Shovel").get_buried_percent() == 0:
			stuck_position = Vector2.INF 
	
		if not stuck_position == Vector2.INF:
			point = stuck_position
		
	fsm_root.point_to(point)

	#pull-back detection
	if fsm_root.input_pull_jp: 
		if fsm_root.get_node("Tween").is_active():
			yield(fsm_root.get_node("Tween"), "tween_all_completed")
		exit("SGPulledState")
