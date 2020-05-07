extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

signal shoot

export var stab_speed_reduct_rate = .5
export var dash_speed_rate = 3
export var stabbing_dist = 10000
export var pull_back_dist = -2500
export var pull_back_dur = 1;
export var init_position = Vector2(-10,17)
var normal_speed_rate = 1
var pre_stab = false
var can_stab = false
var stabbing = false
var after_stabbing = false
#var stop_moving = false
var player_id


#Client Set Variables 
puppet var mousepos = Vector2(0,0) #todo cheater check
puppet var stab_btn_jp = false # stab button just pressed
puppet var stab_btn_jr = false # stab button just released
puppet var shoot_btn_p = false # shoot button pressed

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

var velocity = Vector2.ZERO
var newPos = Vector2.ZERO	

func _disable_collision(obj, disable):
	if obj != null:
		obj.get_node("CollisionShape2D").disabled = disable

func _on_shovel_pick_up (player_id):
	if self.player_id == player_id:
		var shovel = Shovel.instance()
		call_deferred("add_child", shovel)
		shovel.call_deferred("setup")
	
remote func no_stab():
	after_stabbing = true
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
	TweenNode.interpolate_property(self, "position", currPos, newPos, pull_back_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	yield(TweenNode, "tween_completed")
	#stop_moving = false
	get_parent().speed_rate = normal_speed_rate
	pre_stab = false


##SHOOTING STUFF #########################################################
func _on_Reload_timeout():
	$Reload.stop()
	if !has_node("Projectile" + player_id):
		rpc("_reload", player_id)
		
remote func shoot():
	if has_node("Projectile" + player_id) and $Vulnerable.is_stopped():
		rpc('shooting', player_id, $Muzzle.global_position, Vector2(1, 0).rotated(self.global_rotation))

remotesync func shooting(player_id, pos, dir):
	_disable_collision(ShovelNode, false)
	var shovel = get_node("Projectile" + player_id)
	$Reload.start()
	shovel.get_node("Reload").start()
	emit_signal('shoot', shovel, pos, dir)
	
remotesync func _reload(player_id):
	var shovel = Shovel.instance()
	add_child(shovel)
	ShovelNode = shovel
	ShovelNode.setup()


##DASHING OLD CODE
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
