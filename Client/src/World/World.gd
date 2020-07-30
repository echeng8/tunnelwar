extends Node2D

#RESOURCES LOAD
var Player #= preload("res://Player/Player.tscn")

var load_completed = false

const resources = {
	#scenes
	#"Shovel": preload("res://Weapons/Shovel/Shovel.tscn"), 
}

signal on_load_complete

### PLAYER STUFF 
	
puppet func emit_load_complete():
	load_completed = true  
	emit_signal("on_load_complete") 
	
remotesync func instantiate_player(spawn_pos, id, username):
	if $Players.has_node(str(id)):
		return 
		
	var player = Player.instance()
	player.position = spawn_pos
	player.name = String(id) # Important
	player.set_network_master(id) # Important
	player.username = username 
	$Players.add_child(player)

### BLOCKS AND SHOVELS 
remote func spawn(file_name, node_name, transform_dict):
	if node_name in $Items.get_children():
		return
		
	var instancedThing = resources[file_name].instance()
	instancedThing.name = node_name
	
	if file_name == "Block":
		$Blocks.add_child(instancedThing) 
	if file_name == "Shovel":
		$Items.add_child(instancedThing) 
	
	instancedThing.global_position = transform_dict["pos"]
	instancedThing.global_rotation = transform_dict["rot"]
	instancedThing.global_scale = transform_dict["sca"]
