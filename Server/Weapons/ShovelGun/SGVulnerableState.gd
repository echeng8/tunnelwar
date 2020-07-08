extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var duration = 0

func enter():
	duration = 0
	if fsm_root.is_loaded():
		fsm_root.get_node("Shovel").get_node("StateMachine").change_to("ShDefaultState")
		
		
# Optional handler functions for game loop events
func process(delta):
	duration += delta
	
	if(duration > fsm_root.vulnerability_time):
		fsm_root.move_to(fsm_root.init_position, fsm_root.reset_dur)  
		fsm.change_to("SGDefaultState")

