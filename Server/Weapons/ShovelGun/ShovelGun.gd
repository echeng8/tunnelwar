extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

signal shoot

export var stab_speed_reduct_rate = .5
export var dash_speed_rate = 3
export var stabbing_dist = 7000
var normal_speed_rate = 1
var can_stab = false
var stabbing = false
var player_id

#onready var animationPlayer = $AnimationPlayer
onready var TweenNode = get_node("Tween")
onready var ShovelNode = get_node("Projectile" + player_id)

func _ready():
	_disable_collision(true)
	ShovelNode.connect("_pick_up", self, "_on_shovel_pick_up")

func setup():
	player_id = get_parent().name
	print("ShovelGun" + player_id)
	name = name + player_id
	print(name)
	$Projectile.setup()

func _process(delta):
	if stabbing == true: 
		var currPos = position
		var velocity = Vector2(1, 0).rotated(rotation) * stabbing_dist
		var newPos = position + (velocity * delta)
		rpc("_stabbing", player_id, position, newPos)
		stabbing = false
#	if can_stab == true:
#		print("here")
#		var velocity = Vector2(1, 0).rotated(rotation) * -1000
#		var newPos = position + (velocity * delta)
#		rpc("_pre_stabbing", position, newPos)
#		can_stab = false
func _disable_collision(disable):
	if ShovelNode != null:
		ShovelNode.get_node("CollisionShape2D").disabled = disable	

func _on_shovel_pick_up (player_id):
	if self.player_id == player_id:
		var shovel = Shovel.instance()
		call_deferred("add_child", shovel)
		shovel.call_deferred("setup")
#func _dash(can_dash):
#	if can_dash:
#		get_parent()._dash(can_dash, dash_speed_rate, Vector2(1, 0).rotated(self.global_rotation))
#		$Dash.start()
#	else:
#		get_parent()._dash(can_dash, normal_speed_rate)
#		$Dash.stop()
#		$Vulnerable.start()
#
#func _on_Dash_timeout():
#	_dash(false)
#
func _on_Vulnerable_timeout():
	$Vulnerable.stop()

func _on_Reload_timeout():
	$Reload.stop()
	if !has_node("Projectile" + player_id):
		rpc("_reload", player_id)
	
remote func _update_weapon_position(mouse_position):
	var id = get_tree().get_rpc_sender_id()
	if 	player_id == String(id):
		look_at(mouse_position)
		rpc_unreliable("_update_weapon_position", player_id, mouse_position)
		
remote func shoot():
	if has_node("Projectile" + player_id) and $Vulnerable.is_stopped():
		rpc('shooting', player_id, $Muzzle.global_position, Vector2(1, 0).rotated(self.global_rotation))

remotesync func shooting(player_id, pos, dir):
	_disable_collision(false)
	var shovel = get_node("Projectile" + player_id)
	$Reload.start()
	shovel.get_node("Reload").start()
	emit_signal('shoot', shovel, pos, dir)

remote func pre_stab():
	print("pre_stab")
	if $Vulnerable.is_stopped(): #and can_stab == false:
		can_stab = true
		get_parent().speed_rate = stab_speed_reduct_rate
		#pull back some

remote func no_stab():
	can_stab = false
	get_parent().speed_rate = normal_speed_rate

remote func stab():
	print("stab")
	_disable_collision(false)
	stabbing = true
	get_parent().speed_rate = normal_speed_rate
#	var velocity = Vector2(1, 0).rotated(rotation) * stabbing_dist
#	var newPos = position + (velocity * last_delta)
#	rpc("_stabbing", position, newPos)

remotesync func _pre_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", self.position, newPos, 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	yield(TweenNode, "tween_completed")

remotesync func _stabbing(player_id, currPos, newPos):
	TweenNode.interpolate_property(self, "position", currPos, newPos, 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	yield(TweenNode, "tween_completed")
	_disable_collision(true)
	TweenNode.interpolate_property(self, "position", self.position, currPos , 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	yield(TweenNode, "tween_completed")
	$Vulnerable.start()
	
remotesync func _reload(player_id):
	var shovel = Shovel.instance()
	add_child(shovel)
	ShovelNode = shovel
	ShovelNode.setup()

