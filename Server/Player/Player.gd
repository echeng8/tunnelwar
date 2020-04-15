extends KinematicBody2D

export var speed = 300
export var health_points = 20
var speed_rate = 1
#var dash = false
#var dash_dir = Vector2()

var player_direction =  Vector2.ZERO

func _process(delta):
	var move_dir = Vector2()
#	if(dash):
#		move_dir = dash_dir
#	else:	
	move_dir = player_direction.normalized()
	var velocity = move_dir * (speed * speed_rate)
	position += velocity * delta
	rpc_unreliable("_update_player_movement", name, position)

func damage(value): 
	health_points -= value
	if health_points <= 0:
		health_points = 0
#		rpc('_die')
	rpc("_update_health", health_points)

#
#func _dash(dash, speed_rate, dash_dir = null):
#	self.dash = dash
#	self.speed_rate = speed_rate
#	self.dash_dir = dash_dir

remote func _update_player_movement(var direction:Vector2):
	var id = get_tree().get_rpc_sender_id()
	if name == String(id):
		player_direction = direction


