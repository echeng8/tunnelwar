extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var ShovelGun 
var a = true

func enter():
	ShovelGun = get_parent().get_parent()
	assert("ShovelGun" in ShovelGun.name) 
	
	ShovelGun.get_parent().speed_rate = ShovelGun.stab_speed_reduct_rate
	
	
# Optional handler functions for game loop events
func process(delta):
	if(a):
		ShovelGun.velocity = Vector2(1, 0).rotated(ShovelGun.rotation) * ShovelGun.pull_back_dist
		ShovelGun.newPos = ShovelGun.position + (ShovelGun.velocity * delta)
		ShovelGun.rpc("_pre_stabbing", ShovelGun.position, ShovelGun.newPos)
		a = false
		
	if ShovelGun.stab_btn_jr:  #todo check timer for stab potential
		fsm.change_to("SGStabState")

