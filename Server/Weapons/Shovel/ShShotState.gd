extends "res://Scripts_General/Base_Classes/FSM/State.gd"

var velocity = Vector2(0,0)
var duration = 0
var ShovelNode
func enter():
	ShovelNode = fsm.get_parent()
	assert(ShovelNode is Shovel)
	
	velocity = Vector2(1,0).rotated(ShovelNode.rotation) * ShovelNode.speed
	duration = 0
	
func process(delta):
		duration += delta
		
		if(duration > ShovelNode.pickup_lifespan):
			ShovelNode.rpc("destroy")
		
func physics_process(delta):
	if velocity.length() > 0:
		ShovelNode.position +=  velocity * delta
		ShovelNode.rpc_unreliable("_update_shovel_position", ShovelNode.position)
		

func on_body_entered(body):
	if body.is_in_group("Players") and body.has_method("get_struck_by") or body.is_in_group("Blocks"):
		body.get_struck_by(ShovelNode)
		velocity = Vector2(0,0)
		exit("ShPickUpState")

func on_Shovel_area_entered(area): #hit an activated ShovelGun
	if area.get_node("StateMachine").state.name == "ShChargedState": #reflected
		velocity = -velocity 
		#TODO flip shovel sprite on client