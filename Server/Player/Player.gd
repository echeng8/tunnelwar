extends KinematicBody2D

export var speed = 600
export var health_points = 40
export var knockback_time = 45

var knockback_timer = 0
var knockback_dir
var velocity
var speed_rate = 1

var input_direction =  Vector2.ZERO

func setup():
	$ShovelGun.setup()

func _physics_process(delta):
	var move_dir = Vector2()
	
	if knockback_timer > 0:
		move_dir = knockback_dir.normalized()
		velocity = move_dir * (speed * 1.5)
		knockback_timer -= 1
	else:	
		move_dir = input_direction.normalized()
		velocity = move_dir * (speed * speed_rate)
		
	move_and_slide(velocity, Vector2(0,0))
	rpc_unreliable("set_player_position", position)

func damage(damage_points, knockback_dir): 
	health_points -= damage_points
	if health_points <= 0:
		die()
	else: 
		rpc("_update_health", name, health_points)
		if knockback_dir != null:
			self.knockback_dir = knockback_dir
			knockback_timer = knockback_time
		
func die(): ##todo make this so its 1 call to respawn only w/out passing pos params
	rpc("respawn", Vector2(gamestate.DEV_SPAWN_X, gamestate.DEV_SPAWN_Y), 20)

remotesync func respawn(pos, health_points):
	rpc("set_player_position", pos)
	self.health_points = health_points
	rpc("_update_health", name, health_points)


puppet func set_input_direction(var direction:Vector2):
		print("di: ", direction, get_tree().get_rpc_sender_id())
		input_direction = direction

remotesync func set_player_position(pos):
	position = pos 
	
#
#func _dash(dash, speed_rate, dash_dir = null):
#	self.dash = dash
#	self.speed_rate = speed_rate
#	self.dash_dir = dash_dir
