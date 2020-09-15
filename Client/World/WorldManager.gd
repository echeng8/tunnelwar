extends Node2D

class_name WorldManager

#RESOURCES LOAD
var Player = preload("res://Player/Player.tscn")

var load_completed = true

const resources = {
	#scenes
	"Shovel": preload("res://Weapons/Shovel/Shovel.tscn"), 
}

### PLAYER STUFF 
func _init():
	gamestate.world_node = self

### BLOCKS AND SHOVELS 
remote func spawn(file_name, node_name, transform_dict):
	if node_name in $Items.get_children():
		return
		
	var instancedThing = resources[file_name].instance()
	instancedThing.name = node_name
	
	if file_name == "Shovel":
		$Items.call_deferred("add_child", instancedThing) 
	
	instancedThing.global_position = transform_dict["pos"]
	instancedThing.global_rotation = transform_dict["rot"]
	instancedThing.global_scale = transform_dict["sca"]
