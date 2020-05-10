extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

signal shoot

#gameplay values 
export var stab_charge_time = 0.5 #seconds you need to pull back in order to stab
export var vulnerability_time = 1 #seconds you are vulnerable after the stab TODO
export var stab_speed_reduct_rate = .5

#pull back animation 
export var pull_back_dist = -2500

#stabbing animation
export var stabbing_dist = 5000
export var init_position = Vector2(-10,17)

#animation duration (how long it takes for the anim to complete)
export var pull_dur = 1.5
export var stab_dur = 0.5
export var reset_dur = 1

#implementation global var
var normal_speed_rate = 1
var player_id

#Client Set Variables 
puppet var mousepos = Vector2(0,0) #todo cheater check
puppet var stab_btn_jp = false # stab button just pressed
puppet var stab_btn_jr = false # stab button just released
puppet var shoot_btn_p = false # shoot button pressed

onready var TweenNode = get_node("Tween")
var ShovelNode

func setup():
	player_id = get_parent().name
	name = name + player_id

	_disable_collision(ShovelNode, true)
	
	#shooting todo make state
	$Projectile.setup()
	ShovelNode = get_node("Projectile" + player_id)
	ShovelNode.connect("_pick_up", self, "_on_shovel_pick_up")


func _ready():
	#set animation durations on client
	rset("pull_dur", pull_dur)
	rset("stab_dur", stab_dur)
	rset("reset_dur", reset_dur)
	
var velocity = Vector2.ZERO
var newPos = Vector2.ZERO	

func _disable_collision(obj, disable):
	if obj != null:
		obj.get_node("CollisionShape2D").disabled = disable
	
	
######ANIMATION FUNCTIONS to be called by states
		
remotesync func _pre_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", self.position, newPos, pull_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()

remotesync func _stabbing(player_id, currPos, newPos):
	TweenNode.interpolate_property(self, "position", currPos, newPos, stab_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	
remotesync func _after_stabbing(player_id, currPos, newPos):
	_disable_collision(ShovelNode, true)
	TweenNode.interpolate_property(self, "position", currPos, newPos, reset_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT) #todo fix pull-back duration not actually working
	TweenNode.start()


##SHOOTING STUFF #########################################################
func _on_Reload_timeout():
	$Reload.stop()
	if !has_node("Projectile" + player_id):
		rpc("_reload", player_id)

func _on_shovel_pick_up (player_id):
	if self.player_id == player_id:
		var shovel = Shovel.instance()
		call_deferred("add_child", shovel)
		shovel.call_deferred("setup")
	
	
remote func shoot():
	if has_node("Projectile" + player_id):
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
#export var dash_speed_rate = 3
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
