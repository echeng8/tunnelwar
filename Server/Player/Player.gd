extends KinematicBody2D

export var speed = 600
export var health_points = 40
export var knockback_duration = 0.5 #in seconds
export var vul_speed_rate = 0.5 #by what % is speed reduced when SG is pulled bacck and thursted out? 
var knockback_dir
puppet var input_direction =  Vector2.ZERO

signal struck_by(node) 

var ShovelGun
#initialization
func setup():
	ShovelGun = $ShovelGun
	ShovelGun.setup()

remote func on_client_node_connect():	#called when client node is ready
	rpc("set_health", health_points)

func get_struck_by(source):
	emit_signal("struck_by", source)

func respawn(pos, health_points):
	rpc("set_player_position", pos)
	self.health_points = health_points 
	rpc("set_health", health_points)

remotesync func set_player_position(pos):
	position = pos 
