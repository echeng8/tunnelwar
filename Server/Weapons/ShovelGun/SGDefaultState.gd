extends State

var stuck_position = Vector2.INF #infinity = null 

func enter():
	if fsm_root.is_loaded():
		fsm_root.get_node("Shovel").get_node("StateMachine").call_deferred("change_to", "ShDefaultState")
		
# Optional handler functions for game loop events
func process(_delta):
	var point_to = fsm_root.input_aim_pos
	if fsm_root.is_loaded():
		if stuck_position == Vector2.INF and fsm_root.get_node("Shovel").get_buried_percent() == 1:
			stuck_position = fsm_root.get_node("Shovel").global_position 
			
		if fsm_root.get_node("Shovel").get_buried_percent() == 0:
			stuck_position = Vector2.INF 
	
		if not stuck_position == Vector2.INF:
			point_to = stuck_position
		
	fsm_root.global_rotation += fsm_root.to_local(point_to).angle() 

	#pull-back detection
	if fsm_root.input_pull_jp: 
		 exit("SGPulledState")
