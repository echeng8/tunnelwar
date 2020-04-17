extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

signal shoot

var player_id
var stabbing = false


#onready var animationPlayer = $AnimationPlayer
onready var TweenNode = get_node("Tween")

func _ready():
	#player_id = get_parent().name
	var shovel = get_node("Projectile" + player_id)
	shovel.connect("_pick_up", self, "_on_shovel_pick_up")
	print(shovel.name)

func setup():
	player_id = get_parent().name
	name = name + player_id
	print(name)
	$Projectile.setup()

func _process(delta):
	if is_network_master():
		rpc_unreliable_id(1, "_update_weapon_position", get_global_mouse_position())
		if Input.is_action_pressed('shoot') :
			rpc_id(1, "shoot")
		if  Input.is_action_just_pressed('stab'):
			$Timer.start()
			rpc_id(1, "pre_stab")
		if Input.is_action_just_released('stab') and $Timer.is_stopped():
			print("stab")
			rpc_id(1, "stab")
		elif Input.is_action_just_released('stab'):
			print("no stab")
			rpc_id(1, "no_stab")

func _on_shovel_pick_up (player_id):
	if self.player_id == player_id:
		var shovel = Shovel.instance()
		call_deferred("add_child", shovel)
		shovel.call_deferred("setup")

func _on_Timer_timeout():
	$Timer.stop()

remote func _update_weapon_position(player_id, mouse_position):
	if self.player_id == player_id:
		look_at(mouse_position)
	
remotesync func _reload(player_id):
	if self.player_id == player_id:
		var shovel = Shovel.instance()
		add_child(shovel)
		shovel.setup()
	
remotesync func shooting(player_id, pos, dir):
	if self.player_id == player_id:
		var shovel = get_node("Projectile" + player_id)
		emit_signal('shoot', shovel, pos, dir)	

remotesync func _stabbing(player_id, currPos, newPos):
	if self.player_id == player_id:
		TweenNode.interpolate_property(self, "position", currPos, newPos, 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
		TweenNode.start()
		yield(TweenNode, "tween_completed")
		TweenNode.interpolate_property(self, "position", self.position, currPos , 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
		TweenNode.start()
		yield(TweenNode, "tween_completed")

remotesync func _pre_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", currPos, newPos, 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	yield(TweenNode, "tween_completed")
