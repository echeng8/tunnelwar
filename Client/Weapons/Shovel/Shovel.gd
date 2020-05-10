extends Area2D

export(Vector2) var init_position = Vector2(335, -23)
export(Vector2) var firing_scale = Vector2(.5, .5)

signal _pick_up

var fire = false
var player_id
var shovel_position

func _ready():
	#player_id = get_parent().player_id
	#print(player_id)
	position = init_position
	show_behind_parent = true

func setup():
	player_id = get_parent().player_id
	name = name + player_id
	print("Shovel:" + player_id)

func _process(_delta):
	if fire == true:	 
		if(shovel_position != null):
			global_position = shovel_position

func start(_position, _direction):
	scale = firing_scale
	global_position = _position
	global_rotation = _direction.angle()
	fire = true

#func _on_VisibilityNotifier2D_screen_exited():
#	fire = false
##	#queue_free()

remote func _on_Reload_timeout(player_id):
	if self.player_id == player_id:
		print("queue1")
		queue_free()

remote func _update_shovel_movement(player_id, shovel_position):
	if self.player_id == player_id:
		self.shovel_position = shovel_position

remotesync func destory_shovel(player_id):
	if self.player_id == player_id:
		queue_free()

#func _on_body_entered(body):
#	#print(get_parent().name)
#	#if fire == false and get_parent().name != "ShovelGun" + body.name:
#		#emit_signal('_pick_up', body.name)
#		#rpc('_pick_up', body.name)
#	#	print("queue2")
#	#	queue_free()
#	#elif get_parent().name == "ShovelGun" :
#	#	if body.has_method('damage'):
#	#		body.damage(STAB_DAMAGE)
#	#else:
#		#if body.has_method('damage'):
#		#	body.damage(DAMAGE)
#	print("queue3")
#	queue_free()
#	fire = false
##	#queue_free()

remote func _pick_up(shovel_player_id, player_id):
	if self.player_id == shovel_player_id:
		emit_signal('_pick_up', player_id)
		print("queue4")
		queue_free()                                        
	
remote func _off_screen():
	fire = false
#
