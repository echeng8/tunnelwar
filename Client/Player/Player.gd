extends KinematicBody2D

export var health_points = 20

var player_position

var cameraReference

# Called when the node enters the scene tree for the first time.
func _ready():
	update_GUI()
	_update_health_bar()
	#When player is spawned (AKA ready) then only parent the camera to self, and not any
	#	of the other players on the client's computah
	if is_network_master():
		_parent_camera_to_me()

func setup():
	$ShovelGun.setup()
	
func update_GUI():
	if is_network_master():
		$GUI/PlayerName.text = "You"
	else:
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
		rpc_unreliable_id(1, '_update_player_movement', movementValuesMerged)
	if(player_position != null):
		position = player_position
	
	#the align function HAS to be called AFTER the position is changed, because
	#	or else it will be aligned with the previous position, dum dum
	if is_network_master():
		_align_camera_to_player()


#### HEALTH
remote func _update_health(player_id, health_points):
	if name == player_id:
		self.health_points = health_points
		_update_health_bar()
	
func _update_health_bar():
	$GUI/HealthBar.value = self.health_points
	
	
	
#### CAMERA
func _parent_camera_to_me():
	cameraReference = get_node("/root/World/Camera2D")
	_reparent(cameraReference, self)

#The align function is a Camera2D node-specific function that:
#	"Align(s) the camera to the tracked node"; tracked node being
#	the parent, I guess.
func _align_camera_to_player():
	cameraReference.align()




#### MOVEMENT 
remote func _update_player_movement(player_id, player_pos):
	if name == player_id:
		player_position = player_pos


#### HELPER FUNCTIONS
func _reparent(var nodeToReparent, var newParent):
  nodeToReparent.get_parent().remove_child(nodeToReparent)
  newParent.add_child(nodeToReparent) 

#func damage(value): 
#	health_points -= value
#	if health_points <= 0:
#		health_points = 0
##		rpc('_die')
#	_update_health_bar()
	
#sync func _die():
#	set_physics_process(false)
#	$Rifle.set_process(false)
#	for child in get_children():
#		if child.has_method('hide'):
#			child.hide()
#	$CollisionShape2D.disabled = true



