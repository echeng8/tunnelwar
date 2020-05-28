extends Control


remote func _chat_box_received_message(var message: String):
	var caller_id = get_tree().get_rpc_sender_id()
	
	#command handling
	if message.begins_with("/"):
		var messageSplit:Array = message.split(" ", true, 0)
		var targetPlayer = get_node("/root/World/Players/" + str(caller_id))
		
		
		if messageSplit[0] == "/movespeed": #/speed [units]
			rpc("add_message", "[color=maroon][HAX][/color] speed before: " + str(targetPlayer.speed) + ", after: " + messageSplit[1])
			targetPlayer.speed = float(messageSplit[1])
			
		if messageSplit[0] == "/stabtime": 
			rpc("add_message", "[color=maroon][HAX][/color] speed before: " + str(targetPlayer.get_node("ShovelGun").stab_dur) + ", after: " + messageSplit[1])
			targetPlayer.get_node("ShovelGun").stab_dur = float(messageSplit[1])
			targetPlayer.get_node("ShovelGun").update_client_settings() 
			
		if messageSplit[0] == "/kill":
			targetPlayer.respawn()
			
		if messageSplit[0] == "/vultime":
			var msg = "[color=maroon][HAX][/color] vultime before: " + str(targetPlayer.get_node("ShovelGun").vulnerability_time) + ", after: : " + messageSplit[1]
			rpc("add_message", msg)
			targetPlayer.get_node("ShovelGun").vulnerability_time = float(messageSplit[1])
	
	#message handling
	else:
		var senderName = str(gamestate.players[caller_id])
		var combinedMsg = senderName + ": " + message
		rpc("add_message", combinedMsg)
