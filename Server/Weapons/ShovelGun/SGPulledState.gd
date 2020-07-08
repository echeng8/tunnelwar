extends "res://Scripts_General/Base_Classes/FSM/State.gd"


#MECHANICS VARIABLES
var shootable_time = 0.1 #if the player stabs during .5 seconds of finished pull, they shoot 

var duration = 0


func enter():
	fsm_root.velocity = Vector2(1, 0).rotated(fsm_root.global_rotation) * fsm_root.pull_back_dist
	fsm_root.newPos = fsm_root.position + (fsm_root.velocity * 1/60) #1/60 to simulate delta
	fsm_root._pre_stabbing(fsm_root.position, fsm_root.newPos)
	
	if fsm_root.isLoaded():
		fsm_root.get_node("Shovel").get_node("StateMachine").change_to("ShChargedState")
		 
	duration = 0
	
# Optional handler functions for game loop events
func process(delta):
	duration += delta
	
	if not fsm_root.input_pull_jp:  #todo check timer for stab potential
		if duration > fsm_root.stab_charge_time:
			if fsm_root.isLoaded() and duration < fsm_root.stab_charge_time + shootable_time: 
				fsm_root.rpc("shoot") 
				
			fsm.change_to("SGStabState")
		else:
			fsm_root._after_stabbing(fsm_root.position, fsm_root.init_position)
			fsm.change_to("SGDefaultState") 

