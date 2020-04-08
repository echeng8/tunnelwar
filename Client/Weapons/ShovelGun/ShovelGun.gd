extends Sprite

const Bullet = preload("res://Weapons/Bullet/Bullet.tscn")

signal shoot

export var ammo_count = 1
export var load_strength = 0
var stabbing = [false, 0]

func _process(delta):
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
		look_at(get_global_mouse_position())
		
		if Input.is_action_pressed('shoot') and $Timer.is_stopped() and ammo_count > 0:
			rpc("shoot")
			#$Timer.start()
			ammo_count = ammo_count - 1
		if Input.is_action_pressed("stab"):
			if load_strength < 15:
				load_strength += 1
			#$Player.speed = 100
		if Input.is_action_just_released("stab"):
			stab()
			#$Player.speed = 300
	if ammo_count <= 0:
		texture = load("res://Art/ShovelGun_RifleStock_v1.png")
	else:
		texture = load("res://Art/shovel gun.png")
		
		

func stab():
	stabbing = [true, load_strength]
	load_strength = 0

func _on_Timer_timeout():
	$Timer.stop()

sync func shoot():
	var v1 = $Muzzle.global_position
	emit_signal('shoot', Bullet,  v1 , Vector2(1, 0).rotated(self.global_rotation))
#	var b = Bullet.instance()
#	add_child(b)
#	b.start($Muzzle.position, Vector2(1, 0).rotated($Muzzle.rotation))
#	b.fire = true
	
#sync func _shoot():
#	var bullet = Bullet.instance()
#	add_child(bullet)
#	bullet.global_position = global_position
#	bullet.direction = -1 if flip_h else 1	
	
	
func _on_Area2D_area_entered(area):
	if stabbing[0] == true and area.name == "PlayerArea":
		$Player.hit(stabbing[1])
