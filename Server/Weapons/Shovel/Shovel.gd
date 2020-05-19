extends Area2D

############ MECHANICS 

export(float) var speed = 1000 #as projectile
export(float) var damage = 10

var projectile_lifespan = 5 #seconds 

#knockback stats
var knockback_speed = 2000 #game units per second
var knockback_duration = 0.1 #in seconds



var direction = 0
var velocity = Vector2(0,0)

func _ready(): 
	show_behind_parent = true
	add_to_group("Shovels")

func _physics_process(delta):
	if not get_parent().name == "ShovelGun": 
		projectile_lifespan -= delta
		
	if velocity.length() > 0:
		position +=  velocity * delta
		rpc_unreliable("_update_shovel_position", position)
	
	if projectile_lifespan < 0:
		rpc("destroy")
		
remotesync func destroy():
	queue_free()

func fire():
	velocity = Vector2(1,0).rotated(rotation) * speed

#####COLLISIONS
#TODO LET SHOVELGUN TOGGLE THESE PROPERTIES
func _on_body_entered(body):
	if body.is_in_group("Players") and body.has_method("get_struck_by"):
		if get_parent().name == "ShovelGun": #melee hit
			if get_parent().get_node("StateMachine").state.name == "SGStabState": 
				body.get_struck_by(self)
		else: 
			if velocity == Vector2(0,0):
				if not body.get_node("ShovelGun").isLoaded(): # pick up 
					body.get_node("ShovelGun").rpc("reload")	 
					rpc("destroy")
			else: #ranged hit
				body.get_struck_by(self)
				velocity = Vector2(0,0)

func _on_Shovel_area_entered(area): #hit an activated ShovelGun
	if not get_parent().name == "ShovelGun" and area.get_parent().get_node("StateMachine").state.name == "SGPulledState": #reflected
		velocity = -velocity 
		#TODO flip shovel sprite on client
