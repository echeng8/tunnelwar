extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var ShovelGun 

func enter():
	ShovelGun = get_parent().get_parent()
	assert("ShovelGun" in ShovelGun.name) 
	
	ShovelGun.get_parent().speed_rate = ShovelGun.stab_speed_reduct_rate

	ShovelGun.velocity = Vector2(1, 0).rotated(ShovelGun.rotation) * ShovelGun.pull_back_dist
	ShovelGun.newPos = ShovelGun.position + (ShovelGun.velocity * 1/60) #1/60 to simulate delta
	ShovelGun.rpc("_pre_stabbing", ShovelGun.position, ShovelGun.newPos)
	
	
# Optional handler functions for game loop events
func process(delta):
		
	#todo if not enough time go back old one pull back
		
	if ShovelGun.stab_btn_jr:  #todo check timer for stab potential
		fsm.change_to("SGStabState")

