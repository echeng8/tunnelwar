extends Node2D

const Bullet = preload("res://Weapons/Bullet/Bullet.tscn")

signal shoot

export var ammo_count = 1
export var stab_speed_reduct_rate = .5
export var dash_speed_rate = 3
var normal_speed_rate = 1
var player_id
var can_stab = false

func _ready():
	player_id = get_parent().name
	rpc('update_ammo', ammo_count)

#func _process(delta):
#	if ammo_count != null and ammo_count <= 0:
#		texture = load("res://Art/ShovelGun_RifleStock_v1.png")
#	else:
#		texture = load("res://Art/shovel gun.png")

func decrement_ammo(count): 
	ammo_count = ammo_count - count
	
func increment_ammo(count):
	ammo_count = ammo_count + count
	
remote func _update_weapon_position(mouse_position):
	var id = get_tree().get_rpc_sender_id()
	if get_parent().name == String(id):
		look_at(mouse_position)
		rpc_unreliable("_update_weapon_position", name, mouse_position)
		
remote func shoot():
	if ammo_count > 0 and $Dash.is_stopped() and $Vulnerable.is_stopped():
		decrement_ammo(1)
		$ReloadTimer.start()
		rpc('update_ammo', ammo_count)
		rpc('shooting', $Muzzle.global_position, Vector2(1, 0).rotated(self.global_rotation))

remotesync func shooting(pos, dir):
	emit_signal('shoot', Bullet, pos, dir)

remote func pre_stab():
	if $Dash.is_stopped() and $Vulnerable.is_stopped():
		can_stab = true
		get_parent().speed_rate = stab_speed_reduct_rate

remote func stab():
	if can_stab:
		can_stab = false
		_dash(true)
		
func _on_ReloadTimer_timeout():
	$ReloadTimer.stop()
	increment_ammo(1)
	rpc('update_ammo', ammo_count)

func _dash(can_dash):
	if can_dash:
		get_parent()._dash(can_dash, dash_speed_rate, Vector2(1, 0).rotated(self.global_rotation))
		$Dash.start()
	else:
		get_parent()._dash(can_dash, normal_speed_rate)
		$Dash.stop()
		$Vulnerable.start()
	
func _on_Dash_timeout():
	_dash(false)
	
func _on_Vulnerable_timeout():
	$Vulnerable.stop()
