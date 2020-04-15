extends Node2D

onready var Player = load("res://Player/Player.tscn")

func _ready():
	Server.initialize_world()


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
	var b = bullet.instance()
	add_child(b)
	b.start(pos, dir)
	b.fire = true
