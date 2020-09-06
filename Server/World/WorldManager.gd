extends Node2D

class_name WorldManager

const Player = preload("res://Player/Player.tscn")

signal on_player_load(p_id)

func _init():
	gamestate.world_node = self

#remote func initialize_rpc_sender(caller_username : String):
#	var caller_id = get_tree().get_rpc_sender_id() 
#	for player in get_node("Players").get_children():
#		rpc_id(caller_id, "instantiate_player", player.position, player.get_network_master(), player.username)

	#TODO ITEM  SPAWWNING
#	for item in $Items.get_children():
#		rpc_id(caller_id, "spawn",  item.filename.get_file().get_basename(), item.name, HelperFunctions.get_transform_dict(item))

#	instantiate_player(caller_id, caller_username)
#	rpc_id(caller_id, "emit_load_complete")

func get_instance_nodes() -> Dictionary : 
	var node_names = {"Players" : [], "Items": []}
	for child in $Players.get_children():
		node_names["Players"].append(child.name) 
	for item in $Items.get_children():
		node_names["Items"].append(item.name)
	return node_names  
	
###PLAYERS ###############
func instantiate_player(id, username):
	if $Players.has_node(str(id)): #todo look into deleting this
		return 

	rpc("instantiate_player", id)
	
	var player = Player.instance()
	player.name = String(id)
	player.username = username 
	player.set_network_master(id) # Important
	$Players.add_child(player)

	player.respawn()
	
	

	emit_signal("on_player_load", id)

func add_item(item, reparent = true):
	if(reparent):
		HelperFunctions.reparent(item.get_path(), $Items.get_path(), true) 

	rpc("spawn", item.filename.get_file().get_basename(), item.name, HelperFunctions.get_transform_dict(item))
