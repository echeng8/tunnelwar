extends Node2D

onready var Player = load("res://Player/Player.tscn")

func _ready():
	#Server.initialize_world()
	pass
	
func _process(delta):
	#Server.update_chunks()
	pass
	
	
remotesync func spawn_player(spawn_pos, id):
	var player = Player.instance()
	
	player.position = spawn_pos
	player.name = String(id) # Important
	player.set_network_master(id) # Important
	player.setup()
	
	$Players.add_child(player)
	
	var Weapon = player.get_node("ShovelGun" + String(id))
	Weapon.connect("shoot", self, "_on_Weapon_shoot")


remotesync func remove_player(id):
	$Players.get_node(String(id)).queue_free()

func _on_Weapon_shoot(shovel, pos, dir):
	print("hereshot")
	shovel.get_parent().remove_child(shovel)
	$Projectiles.add_child(shovel)
	shovel.start(pos, dir)
	#shovel.fire = true

remote func _chat_box_received_message(var message: String):
	var caller_id = get_tree().get_rpc_sender_id()
	
	#is it a command?
	if message.begins_with("!"):
		var messageSplit:Array = message.split(" ", true, 0)
		if messageSplit[0] == "!set_s" && messageSplit.size() == 2:
			var targetPlayer = get_node("Players/" + str(caller_id))
			rpc_id(caller_id, "_chat_message", "[color=maroon][HAX][/color] prev_speed: " + str(targetPlayer.speed) + ", new_speed: " + messageSplit[1])
			targetPlayer.speed = int(messageSplit[1])
	else:
		var gameState = get_node("/root/gamestate")
		for p_id in gameState.players:
			var senderName = str(gameState.players[caller_id])
			var combinedMsg = senderName + ": " + message
			rpc_id(p_id, "_chat_message", combinedMsg)




