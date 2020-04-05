extends Sprite

const SPEED = 300
var velocity = Vector2()

puppet var puppet_pos
puppet var puppet_vel = Vector2()

# Called when the node enters the scene tree for the first time.
func _ready():
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
		
		velocity = move_dir.normalized() * SPEED
		
		rset_unreliable("puppet_pos", position)
		rset_unreliable("puppet_vel", velocity)
	else:
		# If we are not the ones controlling this player, 
		# sync to last known position and velocity
		position = puppet_pos
		velocity = puppet_vel
	
	position += velocity * delta
	
	if not is_network_master():
		# It may happen that many frames pass before the controlling player sends
		# their position again. If we don't update puppet_pos to position after moving,
		# we will keep jumping back until controlling player sends next position update.
		# Therefore, we update puppet_pos to minimize jitter problems
		puppet_pos = position
