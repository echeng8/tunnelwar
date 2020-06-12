extends KinematicBody2D

class_name Player

remotesync var username 
#STATS 
var _gold = 0

#MECHANICS 
export var speed = 600
export var base_hp = 40
export var health_points = 40


puppet var input_direction =  Vector2.ZERO

signal struck_by(node) 
signal on_gold_change

remote func on_client_node_connect():	#called when client node is ready
	rpc("set_health", health_points)
	rset("gold", get_gold())

func get_struck_by(source):
	emit_signal("struck_by", source)

func respawn():
	self.health_points = base_hp 
	rpc("set_player_position", Vector2(gamestate.DEV_SPAWN_X, gamestate.DEV_SPAWN_Y))
	rpc("set_health", health_points)

remotesync func set_player_position(pos):
	position = pos 

#SETTERS AND GETTERS
func set_gold(amount):
	_gold = amount 
	rset("gold", _gold)
	emit_signal("on_gold_change")
	
func get_gold():
	return _gold 
	
func add_gold(amount = 1):
	set_gold(get_gold() + amount) 
