extends Control

onready var status = $UIVBox/Status

func _ready():
	pass


func _on_JoinButton_pressed():
	var result = yield(ServerConnection.register_async($UIVBox/NameHBox/NameEdit.text), "completed")
	if result == OK:
		print('connected')
	else:
		print("Error code %s: %s" % [result, ServerConnection.error_message])
		
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
