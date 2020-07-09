extends State

func enter():
	if fsm_root.isLoaded():
		fsm_root.get_node("Shovel").get_node("StateMachine").call_deferred("change_to", "ShDefaultState")
	if not fsm_root.position == fsm_root.init_position:
		fsm_root.move_to(fsm_root.init_position, fsm_root.reset_dur)
# Optional handler functions for game loop events
func process(_delta):
	var rotate_speed_percent = 1
	if fsm_root.isLoaded():
		rotate_speed_percent = 1 - fsm_root.get_node("Shovel").get_buried_percent()
	fsm_root.global_rotation += fsm_root.to_local(fsm_root.input_aim_pos).angle() * rotate_speed_percent

	#pull-back detection
	if fsm_root.input_pull_jp: 
		if fsm_root.get_node("Tween").is_active():
			yield(fsm_root.get_node("Tween"), "tween_all_completed")
		exit("SGPulledState")
