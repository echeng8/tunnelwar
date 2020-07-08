extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var duration = 0

func enter():
	fsm_root.velocity = Vector2(1, 0).rotated(fsm_root.rotation) * fsm_root.stabbing_dist
	fsm_root.newPos = fsm_root.position + (fsm_root.velocity * 1/60)
	fsm_root._stabbing(fsm_root.position, fsm_root.newPos)
	
	if fsm_root.is_loaded():
		fsm_root.get_node("Shovel").get_node("StateMachine").change_to("ShDamagingState")

		
	duration = 0
# Optional handler functions for game loop events
func process(delta):
	duration += delta
	
	if(duration > fsm_root.stab_dur): 
		fsm.change_to("SGVulnerableState")

