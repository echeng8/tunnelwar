extends Node


func add_item(item, reparent = true):
	if(reparent):
		HelperFunctions.reparent(item.get_path(), get_path(), true) 
		
	print(item.filename.get_file())
	rpc("spawn", item.filename.get_file().get_basename(), item.name, get_transform_dict(item))
	
func spawn_items_in(clientID): 
	for item in get_children():
		print("spawning")
		rpc_id(clientID, "spawn",  item.filename.get_file().get_basename(), item.name, get_transform_dict(item))
		
func get_transform_dict(item):
	var transform_dict = {
		"pos" : item.global_position,
		"rot" : item.global_rotation,
		"sca" : item.global_scale
	}
	return transform_dict
