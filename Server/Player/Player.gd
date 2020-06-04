extends KinematicBody2D

class_name Player
#STATS 
export var gold = 0

#MECHANICS 
export var speed = 600
export var base_hp = 40
export var health_points = 40


puppet var input_direction =  Vector2.ZERO

signal struck_by(node) 

remote func on_client_node_connect():	#called when client node is ready
	rpc("set_health", health_points)

func get_struck_by(source):
	emit_signal("struck_by", source)

func respawn():
	self.health_points = base_hp 
	rpc("set_player_position", Vector2(gamestate.DEV_SPAWN_X, gamestate.DEV_SPAWN_Y))
	rpc("set_health", health_points)

remotesync func set_player_position(pos):
	position = pos 
