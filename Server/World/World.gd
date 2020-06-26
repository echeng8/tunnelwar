extends Node2D

const Player = preload("res://Player/Player.tscn")

signal on_player_load(p_id)
signal on_player_unload(p_id)
	
func _process(delta):
	pass

###PLAYERS ###############
remotesync func remove_player(id):
	$Players.get_node(String(id)).queue_free()
	emit_signal("on_player_unload", id) 

func spawn_player(id, username):
	if $Players.has_node(str(id)):
		return 
		
	var player = Player.instance()
	#TODO logic in case no dirt remains
	player.position = $Blocks.get_random_block("Dirt").position 
	player.name = String(id) # Important
	player.username = username
	player.set_network_master(id) # Important
	
	$Players.add_child(player)
	
	rpc("spawn_player", player.position, player.get_network_master(), player.username)
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

#func gen_chunks_around_players():
#	for player in $Players.get_children():
#		var player_coord = gamestate.get_coord(player.position)
#		var nearest_chunk = Vector2(
#			stepify(player_coord.x,  $Blocks.chunk_length),
#			stepify(player_coord.y, $Blocks.chunk_length)
#		)
#
#		if not nearest_chunk in $Blocks.block_dict: 
#			$Blocks.generate_chunk(nearest_chunk)
#
#		var chunk_size = Vector2($Blocks.chunk_length, $Blocks.chunk_length)  
#
#		var surrounding_chunks = {}
#		surrounding_chunks['br'] = nearest_chunk + chunk_size
#		surrounding_chunks['b'] = nearest_chunk + chunk_size * Vector2(0,1)
#		surrounding_chunks['bl'] = nearest_chunk + chunk_size * Vector2(-1,1)
#		surrounding_chunks['l'] = nearest_chunk + chunk_size * Vector2(-1, 0)
#		surrounding_chunks['r'] = nearest_chunk + chunk_size * Vector2(1,0)
#		surrounding_chunks['tl'] = nearest_chunk + chunk_size * -1
#		surrounding_chunks['t'] = nearest_chunk + chunk_size * Vector2(0,-1)
#		surrounding_chunks['tr'] = nearest_chunk + chunk_size * Vector2(1,-1)
#
#		for coord in surrounding_chunks.values():
#			$Blocks.generate_chunk(coord)
