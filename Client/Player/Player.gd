extends KinematicBody2D

var velocity
var player_position
var cameraReference


# Called when the node enters the scene tree for the first time.
func _ready():
	update_GUI()
	
	if is_network_master():
		_parent_camera_to_me()

func setup():
	$ShovelGun.setup()
	
func update_GUI():
	##SET NAME
	var player_id = get_network_master()
	$GUI/PlayerName.text = gamestate.players[player_id]

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if is_network_master():
		var leftValue = -Input.get_action_strength("left")
		var rightValue = Input.get_action_strength("right")
		var upValue = -Input.get_action_strength("up")
		var downValue = Input.get_action_strength("down")
		var movementValuesMerged = Vector2(leftValue + rightValue, upValue + downValue)
		rpc_unreliable_id(1, 'set_input_direction', movementValuesMerged)
	
	
	#the align function HAS to be called AFTER the position is changed, because
	#	or else it will be aligned with the previous position, dum dum
	if is_network_master():
		_align_camera_to_player()


#### HEALTH
remote func set_health(health_points):
	$GUI/HealthBar.value = health_points #TODO MAKE PUBLIC VARIABLE TO BE RSET BY SERVER

#The align function is a Camera2D node-specific function that:
#	"Align(s) the camera to the tracked node"; tracked node being
#	the parent, I guess.
func _align_camera_to_player():
	cameraReference.align()

remotesync func set_player_position(pos):
	position = pos 

###### HELPER FUNCTIONS
func _reparent(var nodeToReparent, var newParent):
  nodeToReparent.get_parent().remove_child(nodeToReparent)
  newParent.add_child(nodeToReparent) 


#### CAMERA ###################
func _parent_camera_to_me():
	cameraReference = get_node("/root/World/Camera2D")
	_reparent(cameraReference, self)

