extends Node2D

var ammo_count = 1
#puppet var puppet_look_at = Vector2() 
#
#func _on_Timer_timeout():
#	$Timer.stop()
#
##sync func _shoot():
##	var bullet = Bullet.instance()
#	add_child(bullet)
#	bullet.global_position = global_position
#	bullet.direction = -1 if flip_h else 1


remote func _update_weapon_position(mouse_position):
	var id = get_tree().get_rpc_sender_id()
	if get_parent().name == String(id):
		look_at(mouse_position)
		rpc_unreliable("_update_weapon_position", name, mouse_position)
		
remote func shoot():
	if ammo_count > 0:
		var v1 = $Muzzle.global_position
		ammo_count = ammo_count - 1
		rpc('shoot', v1, Vector2(1, 0).rotated(self.global_rotation))
#	var b = Bullet.instance()
#	add_child(b)
#	b.start($Muzzle.position, Vector2(1, 0).rotated($Muzzle.rotation))
#	b.fire = true
