extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var ShovelGun 
var a = true

func enter():
	ShovelGun = get_parent().get_parent()
	assert("ShovelGun" in ShovelGun.name) 
	
	ShovelGun._disable_collision(ShovelGun.ShovelNode, false)
	ShovelGun.velocity = Vector2(1, 0).rotated(ShovelGun.rotation) * ShovelGun.stabbing_dist
	ShovelGun.newPos = ShovelGun.position + (ShovelGun.velocity * 1/60)
	ShovelGun.rpc("_stabbing", ShovelGun.player_id, ShovelGun.position, ShovelGun.newPos)
	
# Optional handler functions for game loop events
func process(delta):
	return; 
 



func _on_Vulnerable_timeout():
	print("TEST")
	ShovelGun.rpc("_after_stabbing", ShovelGun.player_id, ShovelGun.position, ShovelGun.init_position)
	#yield(get_tree().create_timer(ShovelGun.pull_back_dur), "timeout") #toddo account for time the shovel retracts? 
	fsm.change_to("SGDefaultState")
