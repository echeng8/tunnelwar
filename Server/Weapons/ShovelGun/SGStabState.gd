extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var ShovelGun 
var duration = 0

func enter():
	ShovelGun = get_parent().get_parent()
	assert("ShovelGun" in ShovelGun.name) 

	ShovelGun.velocity = Vector2(1, 0).rotated(ShovelGun.rotation) * ShovelGun.stabbing_dist
	ShovelGun.newPos = ShovelGun.position + (ShovelGun.velocity * 1/60)
	ShovelGun.rpc("_stabbing", ShovelGun.position, ShovelGun.newPos)
	
	if ShovelGun.isLoaded():
		ShovelGun.get_node("Shovel").get_node("StateMachine").change_to("ShDamagingState")

		
	duration = 0
# Optional handler functions for game loop events
func process(delta):
	duration += delta
	
	if(duration > ShovelGun.stab_dur): 
		fsm.change_to("SGVulnerableState")

