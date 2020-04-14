extends KinematicBody2D

const Bullet = preload("res://Weapons/Bullet/Bullet.tscn")

export var speed = 300
export var health_points = 20

#machinemacn1357 commented this.
#enum MoveDirection { UP, DOWN, LEFT, RIGHT, NONE }
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
	
func update_GUI():
	if is_network_master():
		$GUI/PlayerName.text = "You"
		#$Camera.current = true
	else:
		var player_id = get_network_master()
		$GUI/PlayerName.text = gamestate.players[player_id]

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	#machineman1357 commented this.
	#var direction = MoveDirection.NONE
	if is_network_master():
		#machineman1357 changed this...
#		if Input.is_action_pressed('left'):
#			direction = MoveDirection.LEFT
#		elif Input.is_action_pressed('right'):
#			direction = MoveDirection.RIGHT
#		elif Input.is_action_pressed('up'):
#			direction = MoveDirection.UP
#		elif Input.is_action_pressed('down'):
#			direction = MoveDirection.DOWN
		#to this.
		var leftValue = -Input.get_action_strength("left")
		var rightValue = Input.get_action_strength("right")
		var upValue = -Input.get_action_strength("up")
		var downValue = Input.get_action_strength("down")
		var movementValuesMerged = Vector2(leftValue + rightValue, upValue + downValue)
		#/machineman's edits
		
		rpc_unreliable_id(1, '_update_player_movement', movementValuesMerged)
		#_move(direction)
		
	if(player_position != null):
		position = player_position
	
	#the align function HAS to be called AFTER the position is changed, because
	#	or else it will be aligned with the previous position, dum dum
	if is_network_master():
		_align_camera_to_player()


remote func _update_player_movement(player_id, player_pos):
	if name == player_id:
		player_position = player_pos
	
remote func _update_health(health_points):
	self.health_points = health_points
	_update_health_bar()

func _update_health_bar():
	$GUI/HealthBar.value = self.health_points


#camera stuff
func _parent_camera_to_me():
	cameraReference = get_node("/root/World/Camera2D")
	_reparent(cameraReference, self)

#The align function is a Camera2D node-specific function that:
#	"Align(s) the camera to the tracked node"; tracked node being
#	the parent, I guess.
func _align_camera_to_player():
	cameraReference.align()
#/camera stuff

#This is just a helper function I (machineman1357) created.
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
	
