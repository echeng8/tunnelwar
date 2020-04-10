extends Node2D


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
