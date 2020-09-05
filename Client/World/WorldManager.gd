extends Node2D

class_name WorldManager

#RESOURCES LOAD
var Player = preload("res://Player/Player.tscn")

var load_completed = true

const resources = {
	#scenes
	"Shovel": preload("res://Weapons/Shovel/Shovel.tscn"), 
}

signal on_load_complete

### PLAYER STUFF 
func _init():
	gamestate.world_node = self

puppet func emit_load_complete():
	emit_signal("on_load_complete") 
	print('emiting')
	
remote func instantiate_player(id):
	if $Players.has_node(str(id)):
		return 
	
	var p = Player.instance()
	p.name = String(id)
	p.set_network_master(int(id))
	$Players.add_child(p) 

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
