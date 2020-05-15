extends Area2D

############ MECHANICS 
export(float) var speed = 1000
export(float) var damage = 10

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
	print("Shovel:" + player_id)

func _process(delta):
	if fire == true:
		if reflect:
			velocity = -velocity
			reflect = false 
		global_position +=  velocity * delta
		rpc_unreliable("_update_shovel_movement", player_id, global_position)

func reflecting():
	reflect = true

func start(_position, _direction):
	#scale = firing_scale
	global_position = _position
	global_rotation = _direction.angle()
	velocity = _direction * speed
	fire = true
		
remotesync func destory_shovel(player_id):
	queue_free()
	
func _on_body_entered(body):
	if(body.has_method("get_struck_by")):
		body.get_struck_by(self) 
		
#toddo destroy shovel when shot at someone

func _on_Reload_timeout():
	$Reload.stop()
	rpc("_on_Reload_timeout", player_id)
	rpc("destory_shovel", player_id)

func _on_Projectile_area_shape_entered(area_id, area, area_shape, self_shape):
	print("ShovelArea")
	if is_in_group("shovels"):
		if get_parent().name == "ShovelGun" + player_id and area.get_parent().name != "ShovelGun" + area.player_id:
			if area.has_method('reflecting'):
				area.reflecting()
