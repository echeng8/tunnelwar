extends Sprite

const Bullet = preload("res://Weapons/Bullet/Bullet.tscn")
export var load_strength = 0
var stabbing = [false, 0]

func _process(delta):
	look_at(get_global_mouse_position())
	match load_strength:
		0:
			position.x = 29
		5:
			position.x = 26
		10:
			position.x = 23
		15:
			position.x = 20
	if is_network_master():
		if Input.is_action_pressed('shoot') and $Timer.is_stopped():
			rpc('_shoot')
			#shoot()
			$Timer.start()
		if Input.is_action_pressed("stab"):
			if load_strength < 15:
				load_strength += 1
			#$Player.speed = 100
		if Input.is_action_just_released("stab"):
			stab()
			#$Player.speed = 300

func stab():
	stabbing = [true, load_strength]
	load_strength = 0

func _on_Timer_timeout():
	$Timer.stop()

sync func _shoot():
	$Projectile.fire = true
	#var v1 = Vector2(1, 0)
	#var dir = v1.rotated(global_position)
	$Projectile.position = $Muzzle.position
	#$Projectile.direction = dir
	
#sync func _shoot():
#	var bullet = Bullet.instance()
#	add_child(bullet)
#	bullet.global_position = global_position
#	bullet.direction = -1 if flip_h else 1	
	
	
func _on_Area2D_area_entered(area):
	if stabbing[0] == true and area.name == "PlayerArea":
		$Player.hit(stabbing[1])
