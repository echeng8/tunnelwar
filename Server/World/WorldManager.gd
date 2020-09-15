extends Node2D

class_name WorldManager

const Player = preload("res://Player/Player.tscn")

func _init():
	gamestate.world_node = self

func get_instance_nodes() -> Dictionary: 
	var node_names = {"Items": []}
	for item in $Items.get_children():
		node_names["Items"].append(item.name)
	return node_names  

func add_item(item, reparent = true):
	if(reparent):
		HelperFunctions.reparent(item.get_path(), $Items.get_path(), true) 
	
	rpc("spawn", item.filename.get_file().get_basename(), item.name, HelperFunctions.get_transform_dict(item))
