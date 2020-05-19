extends Area2D

export(Vector2) var init_position = Vector2(335, -23)

func _ready():
	position = init_position
	show_behind_parent = true

remotesync func destroy():
	queue_free()

remote func _update_shovel_position(shovel_position):
		self.position = shovel_position
