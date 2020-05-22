extends Node2D

const Player = preload("res://Player/Player.tscn")

###PLAYERS ###############
remotesync func remove_player(id):
	$Players.get_node(String(id)).queue_free()
	
remotesync func spawn_player(spawn_pos, id):
	if $Players.has_node(str(id)):
		return 
		
	var player = Player.instance()
	
	player.position = spawn_pos
	player.name = String(id) # Important
	player.set_network_master(id) # Important
	
	$Players.add_child(player)

func spawn_everything_in(caller_id): 
	for player in get_node("Players").get_children():
		rpc_id(caller_id, "spawn_player", player.position, player.get_network_master())
	for item in $Items.get_children():
		rpc_id(caller_id, "spawn",  item.filename.get_file().get_basename(), item.name, get_transform_dict(item))
	for block in $Blocks.get_children():
		rpc_id(caller_id, "spawn",  block.filename.get_file().get_basename(), block.name, get_transform_dict(block))


func add_item(item, reparent = true):
	if(reparent):
		HelperFunctions.reparent(item.get_path(), $Items.get_path(), true) 
		
	print(item.filename.get_file())
	rpc("spawn", item.filename.get_file().get_basename(), item.name, get_transform_dict(item))


func get_transform_dict(item):
	var transform_dict = {
		"pos" : item.global_position,
		"rot" : item.global_rotation,
		"sca" : item.global_scale
	}
	return transform_dict




######CHAT STUFF
remote func _chat_box_received_message(var message: String):
	var caller_id = get_tree().get_rpc_sender_id()
	
	#is it a command?
	if message.begins_with("/"):
		var messageSplit:Array = message.split(" ", true, 0)
		var targetPlayer = get_node("Players/" + str(caller_id))
		
		if messageSplit[0] == "/movespeed": #/speed [units]
			rpc_id(caller_id, "_chat_message", "[color=maroon][HAX][/color] speed before: " + str(targetPlayer.speed) + ", after: " + messageSplit[1])
			targetPlayer.speed = int(messageSplit[1])
		if messageSplit[0] == "/kill":
			targetPlayer.respawn()
		if messageSplit[0] == "/vultime":
			var msg = "[color=maroon][HAX][/color] vultime before: " + str(targetPlayer.get_node("ShovelGun").vulnerability_time) + ", after: : " + messageSplit[1]
			rpc_id(caller_id, "_chat_message", msg)
			targetPlayer.get_node("ShovelGun").vulnerability_time = float(messageSplit[1])
			
	else:
		var gameState = get_node("/root/gamestate")
		for p_id in gameState.players:
			var senderName = str(gameState.players[caller_id])
			var combinedMsg = senderName + ": " + message
			rpc_id(p_id, "_chat_message", combinedMsg)




