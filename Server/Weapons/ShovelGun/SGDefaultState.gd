extends State

var ShovelGun 

func enter():
	ShovelGun = fsm_root
	assert("ShovelGun" in ShovelGun.name)
	
	if ShovelGun.isLoaded():
		ShovelGun.get_node("Shovel").get_node("StateMachine").call_deferred("change_to", "ShDefaultState")
		
# Optional handler functions for game loop events
func process(delta):
	ShovelGun.look_at(ShovelGun.input_aim_pos)
	ShovelGun.rpc_unreliable("_update_weapon_position", ShovelGun.input_aim_pos)

	#pull-back detection
	if ShovelGun.input_pull_jp: 
		 exit("SGPulledState")
	
