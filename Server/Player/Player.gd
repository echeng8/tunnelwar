extends KinematicBody2D

export var speed = 600
export var health_points = 20
export var knockback_time = 45

var knockback_timer = 0
var knockback_dir
var velocity
var speed_rate = 1
#var dash = false
#var dash_dir = Vector2()

var player_direction =  Vector2.ZERO

func setup():
	$ShovelGun.setup()

func _process(delta):
	var move_dir = Vector2()
	
	if knockback_timer > 0:
		move_dir = knockback_dir.normalized()
		velocity = move_dir * (speed * 1.5)
		knockback_timer -= 1
#	else:
#	if(dash):
#		move_dir = dash_dir
	else:	
		move_dir = player_direction.normalized()
		velocity = move_dir * (speed * speed_rate)
	move_and_slide(velocity, Vector2(0,0))
	#position += velocity * delta
	rpc_unreliable("_update_player_movement", name, velocity)

func damage(damage_points, knockback_dir): 
	health_points -= damage_points
	if health_points <= 0:
		die()
	rpc("_update_health", name, health_points)
	if knockback_dir != null:
		self.knockback_dir = knockback_dir
		knockback_timer = knockback_time
		
func die():
	var pos = gamestate.random_vector2(500, 500)
	rpc("respawn", pos, 20)

remotesync func respawn(pos, health_points):
	position = pos
	self.health_points = health_points
#
#func _dash(dash, speed_rate, dash_dir = null):
#	self.dash = dash
#	self.speed_rate = speed_rate
#	self.dash_dir = dash_dir

remote func _update_player_movement(var direction:Vector2):
	var id = get_tree().get_rpc_sender_id()
	if name == String(id):
		player_direction = direction


