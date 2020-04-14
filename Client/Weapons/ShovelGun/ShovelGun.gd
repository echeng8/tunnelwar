extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

signal shoot

var player_id
var stabbing = false


#onready var animationPlayer = $AnimationPlayer
onready var TweenNode = get_node("Tween")

func _ready():
	player_id = get_parent().name
	var shovel = get_node("Projectile")
	shovel.connect("_pick_up", self, "_on_shovel_pick_up")

func _process(delta):
	if is_network_master():
		rpc_unreliable_id(1, "_update_weapon_position", get_global_mouse_position())
		if Input.is_action_pressed('shoot') and $Timer.is_stopped():
			rpc_id(1, "shoot")
		if  Input.is_action_pressed('stab'):
			rpc_id(1, "pre_stab")
		if Input.is_action_just_released('stab'):
			rpc_id(1, "stab")
			if !stabbing:
				stabbing = true
				var currentPos = position
				var velocity = Vector2(1, 0).rotated(rotation) * 5000
				var newPos = currentPos + (velocity * delta)
	
				TweenNode.interpolate_property(self, "position", self.position, newPos, 1.0, Tween.TRANS_LINEAR, Tween.EASE_OUT)
				TweenNode.start()
				yield(TweenNode, "tween_completed")
				TweenNode.interpolate_property(self, "position", self.position, currentPos, 1.0, Tween.TRANS_BACK, Tween.EASE_OUT)
				TweenNode.start()
				yield(TweenNode, "tween_completed")
				stabbing = false

func _on_shovel_pick_up (player_id):
	if self.player_id == player_id:
		var shovel = Shovel.instance()
		call_deferred("add_child", shovel)

func _on_Timer_timeout():
	$Timer.stop()

remote func _update_weapon_position(player_id, mouse_position):
	if self.player_id == player_id:
		look_at(mouse_position)
	
remotesync func _reload():
	var shovel = Shovel.instance()
	add_child(shovel)
	
remotesync func shooting(pos, dir):
	var shovel = get_node("Projectile")
	shovel.get_node("Reload").start()
	emit_signal('shoot', shovel, pos, dir)	
