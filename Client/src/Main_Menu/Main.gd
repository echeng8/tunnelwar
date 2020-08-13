extends Control

onready var status = $UIVBox/Status
onready var name_edit := $NameHBox/NameEdit

func _ready():
	_register() 

func _on_JoinButton_pressed():
	_set_display_name()
	
	var result: int = yield(ServerConnection.connect_to_server_async(), "completed")
	if result == OK:
		result = yield(ServerConnection.join_world_async(), "completed")
	if result == OK:
		# warning-ignore:return_value_discarded
		ServerConnection.send_spawn(name_edit.text)
		get_tree().change_scene_to(load("res://src/World/World.tscn"))
		
	return result

func _register():
	var result = yield(ServerConnection.register_async(), "completed")
	if result == OK:
		print('connected')
	else:
		print("Error code %s: %s" % [result, ServerConnection.error_message])
		
func _set_display_name() -> void:
	var result = yield(ServerConnection.set_user_display_name_async(name_edit.text), "completed")
	if not result == OK: 
		print(ServerConnection.error_message)
