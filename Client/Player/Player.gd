extends KinematicBody2D
class_name Player
var cameraReference



#PSEUDO PUPPETS - set by server 
var health_points
var server_state : String
remote var gold 
remotesync var username setget username_set 
remotesync var owner_id  = -1 setget owner_id_set

#signals
signal on_server_state_change(state) 

# Called when the node enters the scene tree for the first time.
func _ready():
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

func username_set(un: String):
	username = un 
	$GUI/PlayerName.text = username  
	
func owner_id_set(id : int):	
	owner_id = id
	if id == get_tree().get_network_unique_id():
		gamestate.user_player = self 
		set_network_master(get_tree().get_network_unique_id())
		
#### HEALTH
remote func set_health(hp):
	health_points = hp 
	$GUI/HPNumDisplay.text = str(int(health_points / 10))

remotesync func set_player_position(pos):
	position = pos 
