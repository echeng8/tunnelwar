extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var ShovelGun 
var duration = 0

func enter():
	ShovelGun = get_parent().get_parent()
	assert("ShovelGun" in ShovelGun.name) 

	ShovelGun.velocity = Vector2(1, 0).rotated(ShovelGun.rotation) * ShovelGun.stabbing_dist
	ShovelGun.newPos = ShovelGun.position + (ShovelGun.velocity * 1/60)
	ShovelGun.rpc("_stabbing", ShovelGun.player_id, ShovelGun.position, ShovelGun.newPos)
	
	duration = 0
# Optional handler functions for game loop events
func process(delta):
	duration += delta
	
	if(duration > ShovelGun.vulnerability_time): 
		ShovelGun.rpc("_after_stabbing", ShovelGun.player_id, ShovelGun.position, ShovelGun.init_position)
		fsm.change_to("SGDefaultState")

