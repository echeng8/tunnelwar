extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var ShovelGun 

func enter():
	ShovelGun = get_parent().get_parent()
	assert("ShovelGun" in ShovelGun.name) 
	
	
# Optional handler functions for game loop events
func process(delta):
	ShovelGun.look_at(ShovelGun.mousepos)
	ShovelGun.rpc_unreliable("_update_weapon_position", ShovelGun.player_id, ShovelGun.mousepos)
	
	#pull-back detection
	if ShovelGun.stab_btn_jp: 
		 fsm.change_to("SGPulledState")
