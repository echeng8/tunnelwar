extends Node2D

export var ammo_count = 1
export var stab_speed_reduct_rate = .5
export var dash_speed_rate = 3
var normal_speed_rate = 1

func _ready():
	rpc('update_ammo', ammo_count)

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
	if ammo_count > 0:
		decrement_ammo(1)
		$ReloadTimer.start()
		rpc('update_ammo', ammo_count)
		rpc('shoot', $Muzzle.global_position, Vector2(1, 0).rotated(self.global_rotation))

remote func pre_stab():
	pass
	#get_parent().speed_rate = stab_speed_reduct_rate

remote func stab():
	pass
	#get_parent().dash()
	#get_parent().speed_rate = dash_speed_rate
		
func _on_ReloadTimer_timeout():
	$ReloadTimer.stop()
	increment_ammo(1)
	rpc('update_ammo', ammo_count)
