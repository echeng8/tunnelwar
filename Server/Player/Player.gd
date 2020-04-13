extends KinematicBody2D

export var speed = 300
export var health_points = 20
var speed_rate = 1
var dash = false
var dash_dir = Vector2()
enum MoveDirection { UP, DOWN, LEFT, RIGHT, NONE }
#func _ready():
#machineman1357 changed this...
#var player_direction =  MoveDirection.NONE # Just making sure we initilize it
#to this.
var player_direction =  Vector2.ZERO

func _process(delta):
	var move_dir = Vector2()
	if(dash):
		move_dir = dash_dir
	else:	
		#machineman1357 changed this...
#		match player_direction:
#			MoveDirection.NONE:
#				return
#			MoveDirection.UP:
#				move_dir.y -= 1
#			MoveDirection.DOWN:
#				move_dir.y += 1
#			MoveDirection.LEFT:
#				move_dir.x -= 1
#			MoveDirection.RIGHT:
#				move_dir.x += 1
		#to this.
		move_dir = player_direction.normalized()
			
	var velocity = move_dir * (speed * speed_rate)
	position += velocity * delta
	rpc_unreliable("_update_player_movement", name, position)

func _dash(dash, speed_rate, dash_dir = null):
	self.dash = dash
	self.speed_rate = speed_rate
	self.dash_dir = dash_dir

#machineman1357 changed this...
#remote func _update_player_movement(direction):
#	var id = get_tree().get_rpc_sender_id()
#	if name == String(id):
#		player_direction = direction

#to this.
remote func _update_player_movement(var direction:Vector2):
	var id = get_tree().get_rpc_sender_id()
	if name == String(id):
		player_direction = direction

func damage(value): 
	health_points -= value
	if health_points <= 0:
		health_points = 0
#		rpc('_die')
	rpc("_update_health", health_points)
