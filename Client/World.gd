extends Node2D

onready var Player = load("res://Player/Player.tscn")


remotesync func spawn_player(spawn_pos, id):
	var player = Player.instance()
	player.position = spawn_pos
	player.name = String(id) # Important
	player.set_network_master(id) # Important
	$Players.add_child(player)

remotesync func remove_player(id):
	$Players.get_node(String(id)).queue_free()
	
#SHOOTING STUFF
remote func set_cell(x, y, tile):
	$TileCollision/TileMap.set_cell(x, y, tile)

remote func _chat_message(message):
	var chatBox = $ScreenHUD/ChatBox
	chatBox._update_chat_message_box(message)

func get_player_info(id):
	return get_node("/root/World/Players/" + str(id))



