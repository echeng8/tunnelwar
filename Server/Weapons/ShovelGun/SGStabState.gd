extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var duration = 0

func enter():
	var dest = fsm_root.init_position + Vector2.RIGHT * fsm_root.stabbing_dist
	fsm_root.move_to(dest, fsm_root.stab_dur)
	
	if fsm_root.is_loaded():
		fsm_root.get_node("Shovel").get_node("StateMachine").change_to("ShDamagingState")

		
	duration = 0
# Optional handler functions for game loop events
func process(delta):
	duration += delta
	
	if(duration > fsm_root.stab_dur): 
		fsm.change_to("SGVulnerableState")

