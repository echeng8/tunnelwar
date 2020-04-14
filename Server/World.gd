extends Node2D

onready var Player = load("res://Player/Player.tscn")


remotesync func spawn_player(spawn_pos, id):
	var player = Player.instance()
	
	player.position = spawn_pos
	player.name = String(id) # Important
	player.set_network_master(id) # Important
	
	$Players.add_child(player)
	
	var Weapon = player.get_node("ShovelGun")
	Weapon.connect("shoot", self, "_on_Weapon_shoot")


remotesync func remove_player(id):
	$Players.get_node(String(id)).queue_free()

func _on_Weapon_shoot(bullet, pos, dir):
	bullet.get_parent().remove_child(bullet)
	add_child(bullet)
	bullet.start(pos, dir)
	bullet.fire = true
