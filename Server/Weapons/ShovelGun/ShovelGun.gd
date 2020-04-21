extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

signal shoot

export var stab_speed_reduct_rate = .5
export var dash_speed_rate = 3
export var stabbing_dist = 7000
export var pull_back_dist = -1500
export var init_position = Vector2(-10,17)
var normal_speed_rate = 1
var pre_stab = false
var can_stab = false
var stabbing = false
var after_stabbing = false
#var stop_moving = false
var player_id

#onready var animationPlayer = $AnimationPlayer
onready var TweenNode = get_node("Tween")
var ShovelNode

func setup():
	player_id = get_parent().name
	name = name + player_id
	$Projectile.setup()
	ShovelNode = get_node("Projectile" + player_id)
	ShovelNode.connect("_pick_up", self, "_on_shovel_pick_up")
	_disable_collision(ShovelNode, true)
	
func _process(delta):
	var velocity = Vector2.ZERO
	var newPos = Vector2.ZERO
	if stabbing == true: 
		velocity = Vector2(1, 0).rotated(rotation) * stabbing_dist
		newPos = position + (velocity * delta)
		rpc("_stabbing", player_id, position, newPos)
		stabbing = false
	if after_stabbing == true:		
		rpc("_after_stabbing", player_id, position, init_position)
		after_stabbing = false
	if can_stab == true:
		velocity = Vector2(1, 0).rotated(rotation) * pull_back_dist
		newPos = position + (velocity * delta)
		rpc("_pre_stabbing", position, newPos)
		can_stab = false
		
func _disable_collision(obj, disable):
	if obj != null:
		obj.get_node("CollisionShape2D").disabled = disable

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
	after_stabbing = true
	$Vulnerable.stop()

func _on_Reload_timeout():
	$Reload.stop()
	if !has_node("Projectile" + player_id):
		rpc("_reload", player_id)
	
remote func _update_weapon_position(mouse_position):
	#if stop_moving == false:
	var id = get_tree().get_rpc_sender_id()
	if 	player_id == String(id):
		look_at(mouse_position)
		rpc_unreliable("_update_weapon_position", player_id, mouse_position)
		
remote func shoot():
	if has_node("Projectile" + player_id) and $Vulnerable.is_stopped():
		rpc('shooting', player_id, $Muzzle.global_position, Vector2(1, 0).rotated(self.global_rotation))

remotesync func shooting(player_id, pos, dir):
	_disable_collision(ShovelNode, false)
	var shovel = get_node("Projectile" + player_id)
	$Reload.start()
	shovel.get_node("Reload").start()
	emit_signal('shoot', shovel, pos, dir)

remote func pre_stab():
	if $Vulnerable.is_stopped() and pre_stab == false:
		#stop_moving = true
		pre_stab = true
		can_stab = true
		get_parent().speed_rate = stab_speed_reduct_rate
		#pull back some

remote func no_stab():
	after_stabbing = true
	get_parent().speed_rate = normal_speed_rate

remote func stab():
	if $Vulnerable.is_stopped() and pre_stab == true:
		_disable_collision(ShovelNode, false)
		stabbing = true
		get_parent().speed_rate = normal_speed_rate

remotesync func _pre_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", self.position, newPos, 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	yield(TweenNode, "tween_completed")

remotesync func _stabbing(player_id, currPos, newPos):
	TweenNode.interpolate_property(self, "position", currPos, newPos, 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	yield(TweenNode, "tween_completed")
	$Vulnerable.start()
	
remotesync func _after_stabbing(player_id, currPos, newPos):
	_disable_collision(ShovelNode, true)
	TweenNode.interpolate_property(self, "position", currPos, newPos, 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	yield(TweenNode, "tween_completed")
	#stop_moving = false
	pre_stab = false


#remotesync func _stabbing(player_id, currPos, newPos):
#	TweenNode.interpolate_property(self, "position", currPos, newPos, 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
#	TweenNode.start()
#	yield(TweenNode, "tween_completed")
#	_disable_collision(true)
#	TweenNode.interpolate_property(self, "position", self.position, currPos , 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
#	TweenNode.start()
#	yield(TweenNode, "tween_completed")
#	$Vulnerable.start()
	
remotesync func _reload(player_id):
	var shovel = Shovel.instance()
	add_child(shovel)
	ShovelNode = shovel
	ShovelNode.setup()

