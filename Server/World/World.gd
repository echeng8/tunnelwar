extends Node2D

const Player = preload("res://Player/Player.tscn")

signal on_player_load(p_id)
signal on_player_unload(p_id)

func _process(delta):
	for player in $Players.get_children():
		var player_coord = gamestate.get_coord(player.position)
		var nearest_chunk = Vector2(
			stepify(player_coord.x,  $Blocks.chunk_length),
			stepify(player_coord.y, $Blocks.chunk_length)
		)
		if not nearest_chunk in $Blocks.block_dict: 
			$Blocks.generate_chunk(nearest_chunk)

###PLAYERS ###############
remotesync func remove_player(id):
	$Players.get_node(String(id)).queue_free()
	emit_signal("on_player_unload", id) 

remotesync func spawn_player(spawn_pos, id, username):
	if $Players.has_node(str(id)):
		return 
		
	var player = Player.instance()
	
	player.position = spawn_pos
	player.name = String(id) # Important
	player.username = username
	player.set_network_master(id) # Important
	
	$Players.add_child(player)
	emit_signal("on_player_load", id)

func spawn_everything_in(caller_id): 
	for player in get_node("Players").get_children():
		rpc_id(caller_id, "spawn_player", player.position, player.get_network_master(), player.username)
	for item in $Items.get_children():
		rpc_id(caller_id, "spawn",  item.filename.get_file().get_basename(), item.name, HelperFunctions.get_transform_dict(item))
	for block in $Blocks.get_children():
		block.spawn_on_client(caller_id)
	rpc_id(caller_id, "emit_load_complete")

func add_item(item, reparent = true):
	if(reparent):
		HelperFunctions.reparent(item.get_path(), $Items.get_path(), true) 

	rpc("spawn", item.filename.get_file().get_basename(), item.name, HelperFunctions.get_transform_dict(item))



