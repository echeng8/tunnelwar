extends KinematicBody2D

export var speed = 300
export var health_points = 20
var velocity = Vector2()

#puppet 
var puppet_pos
#puppet 
var puppet_vel = Vector2()

# Called when the node enters the scene tree for the first time.
func _ready():
	puppet_pos = position # Just making sure we initilize it
	update_GUI()
	_update_health_bar()
	
func update_GUI():
	if is_network_master():
		$GUI/PlayerName.text = "You"
		$Camera.current = true
	else:
		var player_id = get_network_master()
		$GUI/PlayerName.text = gamestate.players[player_id]

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if is_network_master():
		var move_dir = Vector2()
		
		if Input.is_action_pressed("up"):
			move_dir.y -= 1
		if Input.is_action_pressed("down"):
			move_dir.y += 1
		if Input.is_action_pressed("left"): 
			move_dir.x -= 1
		if Input.is_action_pressed("right"):
			move_dir.x += 1
		
		#velocity = move_dir.normalized() * speed
		rpc_unreliable_id(1, '_update_player_movement', position, move_dir.normalized())
		#rset_unreliable("puppet_pos", position)
		#rset_unreliable("puppet_vel", velocity)
	#else:
		# If we are not the ones controlling this player, 
		# sync to last known position and velocity
		#position = puppet_pos
		#velocity = puppet_vel

	#velocity = puppet_vel
	position += puppet_vel * delta
		
	if not is_network_master():
		# It may happen that many frames pass before the controlling player sends
		# their position again. If we don't update puppet_pos to position after moving,
		# we will keep jumping back until controlling player sends next position update.
		# Therefore, we update puppet_pos to minimize jitter problems
		puppet_pos = position
		
		
remote func _update_player_movement(position, velocity):
	puppet_pos = position
	puppet_vel = velocity
	
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
	
func hit(load_strength):
	$"..".health_points -= 1 * load_strength
	$"..".position.x += 1
	
