extends Character

var last_direction := Vector2.ZERO

var is_active := true setget set_is_active

onready var timer: Timer = $Timer
onready var camera_2d: Camera2D = $Camera2D


func _get_direction() -> Vector2:
	if not is_processing_unhandled_input():
		return Vector2.ZERO

	var new_direction := Vector2(
		Input.get_action_strength("move_right") - Input.get_action_strength("move_left"), 0
	)
	if new_direction != last_direction:
		ServerConnection.send_direction_update(new_direction.x)
		last_direction = new_direction
	return new_direction

func set_is_active(value: bool) -> void:
	is_active = value
	set_process(value)
	set_process_unhandled_input(value)
	timer.paused = not value
	
func setup(hp : int, gold : int, state : String, level_limits : Rect2) -> void:
	self.username = username
	global_position = position
	spawn()
	camera_2d.set_limits(level_limits)
	set_process(true)
	show()

func spawn() -> void:
	set_process_unhandled_input(false)
	.spawn()
	yield(self, "spawned")
	set_process_unhandled_input(true)
	

#todo connect these 
func _on_Timer_timeout() -> void:
	ServerConnection.send_position_update(global_position)


func _on_GameUI_chat_edit_started() -> void:
	self.is_active = false


func _on_GameUI_chat_edit_ended() -> void:
	self.is_active = true
	
