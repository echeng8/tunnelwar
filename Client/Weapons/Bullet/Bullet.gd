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

func start(_position, _direction):
	scale = firing_scale
	global_position = _position
	global_rotation = _direction.angle()

func _process(delta):
	if fire == true:	 
		if(shovel_position != null):
			global_position = shovel_position

remote func _update_shovel_movement(shovel_position):
	#if name == player_id:
	self.shovel_position = shovel_position
	
#func _on_body_entered(body):
#	if fire == false and get_parent().name != "ShovelGun":
#		emit_signal('pick_up', body.name, self)
#		queue_free()
##	else:
##		if body.has_method('damage'):
##			body.damage(DAMAGE)
#	fire = false
#	#queue_free()

remote func _pick_up(player_id):
	$Reload.stop()
	emit_signal('_pick_up', player_id)
	queue_free()                                        
	
remote func _off_screen():
	fire = false
#
func _on_VisibilityNotifier2D_screen_exited():
	fire = false
#	#queue_free()
	


func _on_Reload_timeout():
	$Reload.stop()
	queue_free()
