extends Area2D

export(Vector2) var init_position = Vector2(335, -23)
export(Vector2) var firing_scale = Vector2(.5, .5)

signal _pick_up

var fire = false
var player_id
var shovel_position

func _ready():
	player_id = get_parent().player_id
	position = init_position
	show_behind_parent = true

func _process(delta):
	if fire == true:	 
		if(shovel_position != null):
			global_position = shovel_position

func start(_position, _direction):
	scale = firing_scale
	global_position = _position
	global_rotation = _direction.angle()

#func _on_VisibilityNotifier2D_screen_exited():
#	fire = false
##	#queue_free()

func _on_Reload_timeout():
	$Reload.stop()
	queue_free()

remote func _update_shovel_movement(shovel_position):
	#if name == player_id:
	self.shovel_position = shovel_position
	
func _on_body_entered(body):
	#print(get_parent().name)
	if fire == false and get_parent().name != "ShovelGun":
		$Reload.stop()
		#emit_signal('_pick_up', body.name)
		#rpc('_pick_up', body.name)
		queue_free()
	#elif get_parent().name == "ShovelGun" :
	#	if body.has_method('damage'):
	#		body.damage(STAB_DAMAGE)
	else:
		#if body.has_method('damage'):
		#	body.damage(DAMAGE)
		queue_free()
	fire = false
#	#queue_free()

remote func _pick_up(player_id):
	$Reload.stop()
	emit_signal('_pick_up', player_id)
	queue_free()                                        
	
remote func _off_screen():
	fire = false
#
