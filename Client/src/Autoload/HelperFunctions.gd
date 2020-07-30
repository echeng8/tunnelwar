extends Node


remotesync func reparent(node1_path, node2_path, _keep_transform = true):
	var node1 = get_node(node1_path)
	var node2 = get_node(node2_path)
	
	var g_pos = node1.global_position
	var g_rot = node1.global_rotation 
	var g_scale = node1.global_scale
	
	node1.get_parent().remove_child(node1)
	
	while node2.has_node(node1.name):
		var last_char = node1.name[node1.name.length() - 1]
		if last_char.is_valid_integer():
			node1.name = node1.name.substr(0,  node1.name.length() - 1) + str(last_char.to_int() + 1)
		else:
			node1.name += "1"
			
	node2.add_child(node1)
	
	node1.global_position = g_pos
	node1.global_rotation = g_rot
	node1.global_scale = g_scale

#returns null if no parent player node
func get_parent_player_node(node):
	var ancestor = node
	while(true):
		if ancestor is Player:
			return ancestor
		if ancestor == null:
			return null
		ancestor = ancestor.get_parent() 
