extends Sprite

var speed = 300
var velocity = Vector2()
export var health_points = 20

puppet var puppet_pos
puppet var puppet_vel = Vector2()

# Called when the node enters the scene tree for the first time.
func _ready():
	_update_health_bar()
	if is_network_master():
		$NameLabel.text = "You"
	else:
		var player_id = get_network_master()
		$NameLabel.text = gamestate.players[player_id]
		
		puppet_pos = position # Just making sure we initilize it


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
		
		velocity = move_dir.normalized() * speed
		
		rset_unreliable("puppet_pos", position)
		rset_unreliable("puppet_vel", velocity)
	else:
		# If we are not the ones controlling this player, 
		# sync to last known position and velocity
		position = puppet_pos
		velocity = puppet_vel
	
	position += velocity * delta
	
	print(health_points)
	
	if not is_network_master():
		# It may happen that many frames pass before the controlling player sends
		# their position again. If we don't update puppet_pos to position after moving,
		# we will keep jumping back until controlling player sends next position update.
		# Therefore, we update puppet_pos to minimize jitter problems
		puppet_pos = position
		
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
