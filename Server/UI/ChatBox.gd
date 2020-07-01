extends Control

func _ready():
	gamestate.chatbox_node = self 

func say(text):
	rpc("add_message", text)
	
remote func _chat_box_received_message(var message: String):
	var caller_id = get_tree().get_rpc_sender_id()
	
	#universal command handling (nodes may implement their own command handling thru signals) 
	if message.begins_with("/"):
		var messageSplit:Array = message.split(" ", true, 0)
		var targetPlayer = get_node("/root/World/Players/" + str(caller_id))
		
		if messageSplit[0] == "/movespeed": #/speed [units]
			rpc("add_message", "[color=maroon][HAX][/color] speed before: " + str(targetPlayer.speed) + ", after: " + messageSplit[1])
			targetPlayer.speed = float(messageSplit[1])
			
		if messageSplit[0] == "/stabtime": 
			rpc("add_message", "[color=maroon][HAX][/color] speed before: " + str(targetPlayer.get_node("ShovelGun").stab_dur) + ", after: " + messageSplit[1])
			targetPlayer.get_node("ShovelGun").stab_dur = float(messageSplit[1])
			
		if messageSplit[0] == "/kill":
			targetPlayer.respawn()
			
		if messageSplit[0] == "/vultime":
			var msg = "[color=maroon][HAX][/color] vultime before: " + str(targetPlayer.get_node("ShovelGun").vulnerability_time) + ", after: : " + messageSplit[1]
			rpc("add_message", msg)
			targetPlayer.get_node("ShovelGun").vulnerability_time = float(messageSplit[1])
			
		if (messageSplit[0] == "/tp" 
			and messageSplit.size() == 3
			and messageSplit[1].is_valid_integer()
			and messageSplit[2].is_valid_integer()
			): 
			var pos = gamestate.get_pos(Vector2(int(messageSplit[1]), int(messageSplit[2])))
			targetPlayer.global_position = pos
		
		#spawns gold at player location 
		if (messageSplit[0] == "/spawn_gold" 
			and messageSplit.size() == 2
			and messageSplit[1].is_valid_integer()
			): 
			gamestate.world_node.get_node("Blocks").spawn_golds_at(gamestate.get_coord(targetPlayer.position), int(messageSplit[1]))
		
		if message == "/frb": #find reset block
			var blocks = gamestate.world_node.get_node("Blocks")
			if is_instance_valid(blocks.reset_block):
				rpc("add_message", blocks.reset_block.coord)
				
		if message == "/destroy_blocks":
			gamestate.world_node.get_node("Blocks").destroy_all_blocks()

	#message handling
	else:
		var senderName = str(gamestate.get_player(caller_id).username)
		var combinedMsg = senderName + ": " + message
		rpc("add_message", combinedMsg)
