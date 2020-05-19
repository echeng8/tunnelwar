extends Area2D

############ MECHANICS 
export(float) var speed = 1000
export(float) var damage = 10

var projectile_lifespan = 5 #seconds 

#knockback stats
var knockback_speed = 2000 #game units per second
var knockback_duration = 0.1 #in seconds

#IMPLEMENTATION VARIABLES
var inShovelGun = true 

signal _pick_up

var direction = 0
var velocity = Vector2(0,0)
var player_id

func _ready(): 
	show_behind_parent = true

func setup():
	player_id = get_parent().player_id
	name = name + player_id
	add_to_group("shovels")

func _physics_process(delta):
	if not inShovelGun: 
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
	inShovelGun = false


#####COLLISIONS
func _on_body_entered(body):
	if body.is_in_group("Players") and body.has_method("get_struck_by"):
		if inShovelGun and get_parent().get_node("StateMachine").state.name == "SGStabState":
			body.get_struck_by(self)
		else: 
			if velocity == Vector2(0,0):
				if not body.ShovelGun.isLoaded(): # pick up 
					body.ShovelGun.rpc("reload")	 #todo use get_node
					rpc("destroy")
			else:
				body.get_struck_by(self)
				velocity = Vector2(0,0)

func _on_Shovel_area_entered(area): #hit an activated ShovelGun
	if not inShovelGun: #reflected 
		velocity = -velocity 
		#TODO flip shovel sprite on client
