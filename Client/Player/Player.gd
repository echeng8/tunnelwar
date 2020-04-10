extends KinematicBody2D

export var speed = 300
export var health_points = 20

enum MoveDirection { UP, DOWN, LEFT, RIGHT, NONE }
var player_position
# Called when the node enters the scene tree for the first time.
func _ready():
	update_GUI()
	_update_health_bar()
	
func update_GUI():
	if is_network_master():
		$GUI/PlayerName.text = "You"
		#$Camera.current = true
	else:
		var player_id = get_network_master()
		$GUI/PlayerName.text = gamestate.players[player_id]

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var direction = MoveDirection.NONE
	if is_network_master():
		if Input.is_action_pressed('left'):
			direction = MoveDirection.LEFT
		elif Input.is_action_pressed('right'):
			direction = MoveDirection.RIGHT
		elif Input.is_action_pressed('up'):
			direction = MoveDirection.UP
		elif Input.is_action_pressed('down'):
			direction = MoveDirection.DOWN
		rpc_unreliable_id(1, '_update_player_movement', direction)
		#_move(direction)
	if(player_position != null):
		position = player_position

remote func _update_player_movement(player_id, player_pos):
	if name == player_id:
		player_position = player_pos
	
func _update_health_bar():
	$GUI/HealthBar.value = health_points
	
func damage(value): 
	health_points -= value
	if health_points <= 0:
		health_points = 0
#		rpc('_die')
	_update_health_bar()
	
#sync func _die():
#	set_physics_process(false)
#	$Rifle.set_process(false)
#	for child in get_children():
#		if child.has_method('hide'):
#			child.hide()
#	$CollisionShape2D.disabled = true
	
