extends Control

func _ready():
	if is_instance_valid(gamestate.user_player):
		connect_player()
	else: 
		assert(gamestate.connect("on_user_player_set", self, "connect_player") == OK)

func connect_player(): 
	assert(gamestate.user_player.connect("on_server_state_change", self, "handle_player_state") == OK)
	
func handle_player_state(state : String) -> void: 
	match state:
		"PDeadState":
			visible = true 
		_:
			visible = false
  
func try_respawn():
	gamestate.user_player.rpc_id(1, "respawn")


func _on_Button_pressed():
	try_respawn() 
