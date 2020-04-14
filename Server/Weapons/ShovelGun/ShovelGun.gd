extends Sprite

const Bullet = preload("res://Weapons/Bullet/Bullet.tscn")

signal shoot

export var ammo_count = 1
export var stab_speed_reduct_rate = .5
export var dash_speed_rate = 3
var normal_speed_rate = 1
var can_stab = false
var player_id

func _ready():
	player_id = get_parent().name
	var shovel = get_node("Projectile")
	shovel.connect("_pick_up", self, "_on_shovel_pick_up")

func decrement_ammo(count): 
	ammo_count = ammo_count - count
	
func increment_ammo(count):
	ammo_count = ammo_count + count
	
remote func _update_weapon_position(mouse_position):
	var id = get_tree().get_rpc_sender_id()
	if 	player_id == String(id):
		look_at(mouse_position)
		rpc_unreliable("_update_weapon_position", player_id, mouse_position)
		
remote func shoot():
	if has_node("Projectile") and $Dash.is_stopped() and $Vulnerable.is_stopped():
		#decrement_ammo(1)
		rpc('shooting', $Muzzle.global_position, Vector2(1, 0).rotated(self.global_rotation))

remotesync func shooting(pos, dir):
	var bullet = get_node("Projectile")
	$Reload.start()
	bullet.get_node("Reload").start()
	emit_signal('shoot', bullet, pos, dir)

remote func pre_stab():
	if $Dash.is_stopped() and $Vulnerable.is_stopped():
		can_stab = true
		get_parent().speed_rate = stab_speed_reduct_rate

remote func stab():
	if can_stab:
		can_stab = false
		_stab()#call remote func and server to use animation to stab
		#_dash(true)
		
func _stab():
	get_parent().speed_rate = normal_speed_rate

func _on_shovel_pick_up (player_id):
	if self.player_id == player_id:
		var bullet = Bullet.instance()
		call_deferred("add_child",bullet)
		
remotesync func _reload():
	var bullet = Bullet.instance()
	add_child(bullet)

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


func _on_Reload_timeout():
	$Reload.stop()
	if !has_node("Projectile"):
		rpc("_reload")
	#Reattach Projectile
