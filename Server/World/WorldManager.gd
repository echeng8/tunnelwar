extends Node2D

class_name WorldManager

const Player = preload("res://Player/Player.tscn")

signal on_player_load(p_id)
signal on_player_unload(p_id)

func _init():
	gamestate.world_node = self

###PLAYERS ###############
remotesync func remove_player(id):
	$Players.get_node(String(id)).queue_free()
	emit_signal("on_player_unload", id) 

func instantiate_player(id, username):
	if $Players.has_node(str(id)): #todo look into deleting this
		return 
		
	var player = Player.instance()
	player.name = String(id) # Important
	player.username = username
	player.position = Vector2(0,0)
	player.set_network_master(id) # Important
	$Players.add_child(player)
	
	rpc("instantiate_player", player.position, player.get_network_master(), player.username)
	player.respawn()
	emit_signal("on_player_load", id)

func spawn_everything_in(caller_id): 
	for player in get_node("Players").get_children():
		rpc_id(caller_id, "instantiate_player", player.position, player.get_network_master(), player.username)
	for item in $Items.get_children():
		rpc_id(caller_id, "spawn",  item.filename.get_file().get_basename(), item.name, HelperFunctions.get_transform_dict(item))
	rpc_id(caller_id, "emit_load_complete")

func add_item(item, reparent = true):
	if(reparent):
		HelperFunctions.reparent(item.get_path(), $Items.get_path(), true) 

	rpc("spawn", item.filename.get_file().get_basename(), item.name, HelperFunctions.get_transform_dict(item))
