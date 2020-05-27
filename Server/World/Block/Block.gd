extends StaticBody2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var broken = false

remote func init_on_client(client_id): 
	rpc_id(client_id, "set_broken", broken)
	 
func get_struck_by(body):
	broken = true  
	$CollisionShape2D.set_deferred("disabled", true)
	rpc("break_block") 
