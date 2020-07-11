extends State

#implementation variables 
var duration = 0

func enter():
	var dest = fsm_root.init_position + Vector2.RIGHT * fsm_root.pull_back_dist
	fsm_root.move_to(dest, fsm_root.pull_time)
	
	if fsm_root.is_loaded():
		fsm_root.get_node("Shovel").get_node("StateMachine").change_to("ShChargedState")
		 
	duration = 0
# Optional handler functions for game loop events
func process(delta):
	duration += delta
	fsm_root.point_to(fsm_root.input_aim_pos, 0.005)
	
	if not fsm_root.input_pull_p:
		if duration > fsm_root.pull_time:
			if fsm_root.is_loaded() and duration > fsm_root.pull_time + fsm_root.shoot_charge_time: 
				fsm_root.rpc("shoot")
			fsm.change_to("SGStabState")
			
		#go back
		else:
			fsm.change_to("SGDefaultState") 

