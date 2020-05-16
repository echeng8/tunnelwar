extends Node


remotesync func reparent(node1_path, node2_path, keep_transform = true):
	var node1 = get_node(node1_path)
	var node2 = get_node(node2_path)
	
	var g_pos = node1.global_position
	var g_rot = node1.global_rotation 
	var g_scale = node1.global_scale
	
	node1.get_parent().remove_child(node1)
	node2.add_child(node1)
	
	node1.global_position = g_pos
	node1.global_rotation = g_rot
	node1.global_scale = g_scale
