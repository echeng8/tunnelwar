extends Sprite
class_name Compass 

var _node_a : Node2D 
var _node_b : Node2D

func point_to(node_a : Node2D, node_b : Node2D, duration = -1) -> void: 
	visible = true 
	
	_node_a = node_a
	_node_b = node_b
	
	if duration > 0: 
		yield(get_tree().create_timer(duration), "timeout")
	else: 
		return 

	visible = false

func stop_pointing() -> void: 
	visible = false  
	
func _process(delta) :
	if visible: 
		rotation = _node_a.position.angle_to_point(_node_b.position) 
