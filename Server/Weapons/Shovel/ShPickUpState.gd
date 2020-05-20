extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var duration = 0
var Shovel 
func enter():
	Shovel = fsm.get_parent()

	duration = 0
	
	
# Opttttional handler functions for game loop events
func process(delta):
	duration += delta 
	if duration > Shovel.pickup_lifespan:
		Shovel.rpc("destroy")

func on_body_entered(body):
	if body.is_in_group("Players") and not body.get_node("ShovelGun").isLoaded() :
		body.get_node("ShovelGun").rpc("reload")	 
		rpc("destroy")
