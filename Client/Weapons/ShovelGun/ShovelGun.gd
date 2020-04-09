extends Sprite

const Bullet = preload("res://Weapons/Bullet/Bullet.tscn")

signal shoot

export var ammo_count = 1
#puppet 
var puppet_look_at_pos = Vector2()

func _process(delta):
	if is_network_master():
		var look_at_pos = get_global_mouse_position()
		look_at(puppet_look_at_pos)
		rpc_unreliable_id(1,"_update_weapon_position" ,look_at_pos)
		#rset_unreliable("puppet_look_at", get_global_mouse_position())
		if Input.is_action_pressed('shoot') and $Timer.is_stopped() and ammo_count > 0:
			rpc("shoot")
			#$Timer.start()
			ammo_count = ammo_count - 1
	else:
		look_at(puppet_look_at_pos)	
		
	if ammo_count <= 0:
		texture = load("res://Art/ShovelGun_RifleStock_v1.png")
	else:
		texture = load("res://Art/shovel gun.png")


remote func _update_weapon_position(position):
	puppet_look_at_pos = position
	
func _on_Timer_timeout():
	$Timer.stop()
	
#sync func shoot():
#	var v1 = $Muzzle.global_position
#	emit_signal('shoot', Bullet,  v1 , Vector2(1, 0).rotated(self.global_rotation))
#	var b = Bullet.instance()
#	add_child(b)
#	b.start($Muzzle.position, Vector2(1, 0).rotated($Muzzle.rotation))
#	b.fire = true
	
#sync func _shoot():
#	var bullet = Bullet.instance()
#	add_child(bullet)
#	bullet.global_position = global_position
#	bullet.direction = -1 if flip_h else 1	
