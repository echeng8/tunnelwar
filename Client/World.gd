extends Node2D

onready var Player = load("res://Player/Player.tscn")


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
	
#SHOOTING STUFF
func _on_Weapon_shoot(shovel, pos, dir):
	print("hereshot")
	shovel.get_parent().remove_child(shovel)
	$Projectiles.add_child(shovel)
	shovel.start(pos, dir)
	#shovel.fire = true

remote func set_cell(x, y, tile):
	$TileCollision/TileMap.set_cell(x, y, tile)

remote func _chat_message(message):
	var chatBox = $ScreenHUD/ChatBox
	chatBox._update_chat_message_box(message)

func get_player_info(id):
	return get_node("/root/World/Players/" + str(id))



