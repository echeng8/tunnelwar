extends KinematicBody2D
class_name Player
var cameraReference



#PSEUDO PUPPETS - set by server 
var health_points
var server_state : String
remote var gold 
var username

#signals
signal on_server_state_change(state) 

# Called when the node enters the scene tree for the first time.
func _ready():

	if is_network_master():
		gamestate.user_player = self
		
	rpc_id(1, "initialize_rpc_sender")
	


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	#send server user input
	if is_network_master():
		#movement
		var leftValue = -Input.get_action_strength("left")
		var rightValue = Input.get_action_strength("right")
		var upValue = -Input.get_action_strength("up")
		var downValue = Input.get_action_strength("down")
		var movementValuesMerged = Vector2(leftValue + rightValue, upValue + downValue)
		rset_unreliable_id(1, 'input_direction', movementValuesMerged)
		
		#Combat 
		rset_unreliable_id(1, "input_aim_pos", get_global_mouse_position()) #todo check for cheating potential 
		rset_unreliable_id(1, "input_pull_p", Input.is_action_pressed('pull')) 

remote func update_client_state(s_state : String):
	server_state = s_state
	emit_signal("on_server_state_change", server_state)
	match server_state:
		"PDefaultState":
			visible = true
		"PDeadState":
			visible = false

remotesync func destroy() -> void:
	queue_free() 
	
remote func set_username(username : String) :
	username = username 
	$GUI/PlayerName.text = username  
	
#### HEALTH
remote func set_health(hp):
	health_points = hp 
	$GUI/HPNumDisplay.text = str(int(health_points / 10))

remotesync func set_player_position(pos):
	position = pos 
