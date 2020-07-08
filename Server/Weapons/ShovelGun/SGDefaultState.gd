extends State

func enter():
	if fsm_root.isLoaded():
		fsm_root.get_node("Shovel").get_node("StateMachine").call_deferred("change_to", "ShDefaultState")
		
# Optional handler functions for game loop events
func process(_delta):
	var rotate_speed_percent = 1
	if fsm_root.isLoaded():
		rotate_speed_percent = 1 - fsm_root.get_node("Shovel").get_buried_percent()
	fsm_root.global_rotation += fsm_root.to_local(fsm_root.input_aim_pos).angle() * rotate_speed_percent

	#pull-back detection
	if fsm_root.input_pull_jp: 
		 exit("SGPulledState")
