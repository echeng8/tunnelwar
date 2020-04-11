extends Sprite

const Bullet = preload("res://Weapons/Bullet/Bullet.tscn")

signal shoot

var ammo_count

func _process(delta):
	if is_network_master():
		rpc_unreliable_id(1, "_update_weapon_position", get_global_mouse_position())
		if Input.is_action_pressed('shoot') and $Timer.is_stopped():
			rpc_id(1, "shoot")
		if  Input.is_action_pressed('stab'):
			rpc_id(1, "pre_stab")
		if Input.is_action_just_released('stab'):
			rpc_id(1, "stab")
	if ammo_count <= 0:
		texture = load("res://Art/ShovelGun_RifleStock_v1.png")
	else:
		texture = load("res://Art/shovel gun.png")

remote func _update_weapon_position(player_id, mouse_position):
	if name == player_id:
		look_at(mouse_position)	
	
remote func shoot(pos, dir):
	emit_signal('shoot', Bullet, pos, dir)

remote func update_ammo(ammo_count): 
	self.ammo_count = ammo_count
		
func _on_Timer_timeout():
	$Timer.stop()


