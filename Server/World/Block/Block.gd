extends StaticBody2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var _broken = false

remote func init_on_client(client_id): 
	rpc_id(client_id, "set_broken", is_broken())

func set_broken(value):
	
	
	_broken = value 
	rpc("set_broken", value)
	$CollisionShape2D.set_deferred("disabled", is_broken())
func is_broken():
	return _broken
	
func get_struck_by(body):
	set_broken(true) 
	rpc("break_block") 
