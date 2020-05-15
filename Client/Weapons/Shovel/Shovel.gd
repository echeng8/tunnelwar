extends Area2D

export(Vector2) var init_position = Vector2(335, -23)
export(Vector2) var firing_scale = Vector2(.5, .5)

signal _pick_up

var player_id

func _ready():
	position = init_position
	show_behind_parent = true

func setup():
	player_id = get_parent().player_id
	name = name + player_id


remotesync func destroy():
	queue_free()

remote func _update_shovel_position(shovel_position):
		self.position = shovel_position

#remote func _pick_up(shovel_player_id, player_id):
#	if self.player_id == shovel_player_id:
#		emit_signal('_pick_up', player_id)
#		print("queue4")
#		queue_free()                                        
	
#
