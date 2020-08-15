extends Sprite
class_name Compass 

var _node_b : Node2D


func point_to(node_b : Node2D, duration = -1) -> void:
	if(node_b == gamestate.user_player):
		visible = false 
		return 
		
	visible = true 
	_node_b = node_b
	
	if duration > 0: 
		yield(get_tree().create_timer(duration), "timeout")
	else: 
		return 

	visible = false

func stop_pointing() -> void: 
	visible = false  
	
func _process(delta) :
	if visible and is_instance_valid(gamestate.user_player) and is_instance_valid(_node_b):
		rotation = gamestate.user_player.position.angle_to_point(_node_b.position) 
