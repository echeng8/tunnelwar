extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var velocity = Vector2(0,0)
var duration = 0
var Shovel 
func enter():
	Shovel = fsm.get_parent()

	velocity = Vector2(1,0).rotated(Shovel.rotation) * Shovel.speed
	duration = 0
	
func _process(delta):
		duration += delta
		
func physics_process(delta):
	if velocity.length() > 0:
		Shovel.position +=  velocity * delta
		Shovel.rpc_unreliable("_update_shovel_position", Shovel.position)
		

func on_body_entered(body):
	if body.is_in_group("Players") and body.has_method("get_struck_by"):
		body.get_struck_by(self)
		velocity = Vector2(0,0)
		exit("ShPickUpState")

func on_Shovel_area_entered(area): #hit an activated ShovelGun
	if area.get_node("StateMachine").state.name == "ShChargedState": #reflected
		velocity = -velocity 
		#TODO flip shovel sprite on client
