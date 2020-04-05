extends Sprite

const Bullet = preload("res://Weapons/Bullet/Bullet.tscn")

func _on_Timer_timeout():
	$Timer.stop()

sync func _shoot():
	var bullet = Bullet.instance()
	add_child(bullet)
	bullet.global_position = global_position
	bullet.direction = -1 if flip_h else 1
