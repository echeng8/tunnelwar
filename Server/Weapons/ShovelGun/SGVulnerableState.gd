extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var ShovelGun 
var duration = 0

func enter():
	ShovelGun = get_parent().get_parent()
	assert("ShovelGun" in ShovelGun.name) 

	duration = 0
	
	if ShovelGun.isLoaded():
		ShovelGun.get_node("Shovel").get_node("StateMachine").change_to("ShDefaultState")
		
		
# Optional handler functions for game loop events
func process(delta):
	duration += delta
	
	if(duration > ShovelGun.vulnerability_time):
		print(duration)
		ShovelGun.rpc("_after_stabbing", ShovelGun.position, ShovelGun.init_position)
		fsm.change_to("SGDefaultState")

