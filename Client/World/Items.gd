extends Node
#items preload
const item_resources = {
	"Shovel": preload("res://Weapons/Shovel/Shovel.tscn")
}

remote func spawn(file_name, node_name, transform_dict):
	if node_name in get_children():
		return
		
	var instancedItem = item_resources[file_name].instance()
	instancedItem.name = node_name
	add_child(instancedItem) 
	
	instancedItem.global_position = transform_dict["pos"]
	instancedItem.global_rotation = transform_dict["rot"]
	instancedItem.global_scale = transform_dict["sca"]
