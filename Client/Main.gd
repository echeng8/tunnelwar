extends Control

onready var status = get_node("VBox/Status")


func _ready():
	gamestate.connect("connection_failed", self, "_on_connection_failed")
	gamestate.connect("connection_succeeded", self, "_on_connection_success")
	gamestate.connect("server_disconnected", self, "_on_server_disconnect")
	gamestate.connect("players_updated", self, "update_players_list")
	
	$VBox/JoinButton.disabled = true
	
	status.text = "Connecting..."
	status.modulate = Color.yellow


func _on_JoinButton_pressed():
	gamestate.my_name = $VBox/HBox/LineEdit.text
	gamestate.pre_start_game()


func _on_connection_success():
	$VBox/JoinButton.disabled = false
	
	status.text = "Connected"
	status.modulate = Color.green


func _on_connection_failed():
	$VBox/JoinButton.disabled = true
	
	status.text = "Connection Failed, trying again"
	status.modulate = Color.red


func _on_server_disconnect():
	$VBox/JoinButton.disabled = true
	
	status.text = "Server Disconnected, trying to connect..."
	status.modulate = Color.red
