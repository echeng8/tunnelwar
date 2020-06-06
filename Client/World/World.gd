extends Node2D

#RESOURCES LOAD
var Player = preload("res://Player/Player.tscn")

const resources = {
	#scenes
	"Shovel": preload("res://Weapons/Shovel/Shovel.tscn"), 
	"Block" : preload("res://World/Block/Block.tscn"),
}

### PLAYER STUFF 
func _ready():
	gamestate.world_node = self
	
	
remotesync func spawn_player(spawn_pos, id, username):
	if $Players.has_node(str(id)):
		return 
		
	var player = Player.instance()
	player.position = spawn_pos
	player.name = String(id) # Important
	player.set_network_master(id) # Important
	player.username = username 
	$Players.add_child(player)

remotesync func remove_player(id):
	$Players.get_node(String(id)).queue_free()

### BLOCKS AND SHOVELS 
remote func spawn(file_name, node_name, transform_dict):
	if node_name in $Items.get_children() or node_name in $Blocks.get_children():
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
