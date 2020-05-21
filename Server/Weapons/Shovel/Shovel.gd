#note: state machine changed by parent shovelgun node
extends Area2D

class_name Shovel

############ MECHANICS 

export(float) var speed = 1000 #as projectile
export(float) var damage = 10

var pickup_lifespan = 5 #seconds 

#knockback stats
var knockback_speed = 2000 #game units per second
var knockback_duration = 0.1 #in seconds

func _ready(): 
	show_behind_parent = true

func get_class():
	return 'Shovel'

	
remotesync func destroy():
	queue_free()
	
#####COLLISIONS
func _on_body_entered(body):
	if $StateMachine.state.has_method("on_body_entered"):
		$StateMachine.state.on_body_entered(body)  
		

func _on_Shovel_area_entered(area): #hit an activated ShovelGun
	if $StateMachine.state.has_method("on_Shovel_area_entered"):
		$StateMachine.state.on_Shovel_area_entered(area)  
