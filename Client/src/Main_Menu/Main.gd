extends Control

onready var status = $UIVBox/Status

func _ready():
	var result = yield(ServerConnection.register_async(), "completed")
	if result == OK:
		print('connected')
	else:
		print("Error code %s: %s" % [result, ServerConnection.error_message])
		


func _on_JoinButton_pressed():
	var result: int = yield(ServerConnection.connect_to_server_async(), "completed")
	if result == OK:
		result = yield(ServerConnection.join_world_async(), "completed")
	if result == OK:
		# warning-ignore:return_value_discarded
		get_tree().change_scene_to(load("res://src/World/World.tscn"))
		
	return result
	
func _on_connection_success():
	$UIVBox/JoinButton.disabled = true

func _on_connection_failed():
	$UIVBox/JoinButton.disabled = false
	
	status.text = "Connection Failed, trying again"
	status.modulate = Color.red

func _on_server_disconnect():
	$UIVBox/JoinButton.disabled = false
	
	status.text = "Server Disconnected, trying to connect..."
	status.modulate = Color.red
