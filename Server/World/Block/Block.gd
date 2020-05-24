extends StaticBody2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


func get_struck_by(body):
	$CollisionShape2D.call_deferred("Disabled", true)
	rpc("set_broke", true)
