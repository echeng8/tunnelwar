extends KinematicBody2D
class_name Player
var cameraReference

remotesync var username

#PSEUDO PUPPETS - set by server 
var health_points
var server_state : String
remote var gold 

# Called when the node enters the scene tree for the first time.
func _ready():
	$GUI/PlayerName.text = username  
	if is_network_master():
		gamestate.user_player = self
		
	rpc_id(1, "on_client_node_connect")

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	#set variables on server
	if is_network_master():
		var leftValue = -Input.get_action_strength("left")
		var rightValue = Input.get_action_strength("right")
		var upValue = -Input.get_action_strength("up")
		var downValue = Input.get_action_strength("down")
		var movementValuesMerged = Vector2(leftValue + rightValue, upValue + downValue)
		rset_unreliable_id(1, 'input_direction', movementValuesMerged)
		
		#Shovel
		rset_unreliable_id(1, "input_aim_pos", get_global_mouse_position()) #todo check for cheating potential 
		rset_unreliable_id(1, "input_pull_jp", Input.is_action_pressed('pull')) 


#### HEALTH
remote func set_health(health_points):
	health_points = health_points 
	$GUI/HPNumDisplay.text = str(int(health_points / 10))

remotesync func set_player_position(pos):
	position = pos 
