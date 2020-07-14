extends Control

onready var status = $UIVBox/Status


func _ready():
	assert(gamestate.connect("connection_failed", self, "_on_connection_failed") == 0)
	assert(gamestate.connect("connection_succeeded", self, "_on_connection_success") == 0)
	assert(gamestate.connect("server_disconnected", self, "_on_server_disconnect") == 0)

func _on_JoinButton_pressed():
	gamestate.my_name = $UIVBox/NameHBox/NameEdit.text
	
	#DEV TOOLS - SERVER IP SET
	gamestate.ip = $UIVBox/DEVServerConnect/ServerIPHBox/ServerIPEdit.text
	gamestate.port = int($UIVBox/DEVServerConnect/ServerPortHBox/ServerPortEdit.text)
	
	#/DEV TOOLS - SERVER IP SET
	
	gamestate.connect_to_server()
	
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
