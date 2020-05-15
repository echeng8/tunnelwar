extends Area2D

############ MECHANICS 
export(float) var speed = 1000
export(float) var damage = 10

export var projectile_lifespan = 5 #seconds
#knockback stats
var knockback_speed = 2000 #game units per second
var knockback_duration = 0.1 #in seconds

############ IMPLEMENTATION
export(Vector2) var firing_scale = Vector2(.5, .5)

var reflect = false

signal _pick_up

var fire = false
var direction = 0
var velocity = 0
var player_id

func _ready(): 
	show_behind_parent = true

func setup():
	player_id = get_parent().player_id
	name = name + player_id
	add_to_group("shovels")

func _physics_process(delta):
	if fire == true:
		projectile_lifespan -= delta
		
		if reflect:
			velocity = -velocity
			reflect = false 
			
		position +=  velocity * delta
		rpc_unreliable("_update_shovel_position", position)
		
		if(projectile_lifespan <= 0):
			rpc("destroy")
			
remotesync func destroy():
	queue_free()

func reflecting():
	reflect = true

func start():
	velocity = Vector2(1,0).rotated(rotation) * speed
	fire = true

func _on_body_entered(body):
	if(body.has_method("get_struck_by")):
		body.get_struck_by(self) 
		
#toddo destroy shovel when shot at someone

func _on_Projectile_area_shape_entered(area_id, area, area_shape, self_shape):
	if is_in_group("shovels"):
		if get_parent().name == "ShovelGun" + player_id and area.get_parent().name != "ShovelGun" + area.player_id:
			if area.has_method('reflecting'):
				area.reflecting()
