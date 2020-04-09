extends Node2D

var speed = 300

var velocity = Vector2()

#puppet var puppet_pos
#puppet var puppet_vel = Vector2()

# Called when the node enters the scene tree for the first time.
#func _ready():
#	var player_id = get_network_master()
#	$NameLabel.text = gamestate.players[player_id]
	
#	puppet_pos = position # Just making sure we initilize it

remote func _update_player_movement(position, direction):
	var velocity = direction * speed
	
	var id = get_tree().get_rpc_sender_id()
	var world = get_node("/root/World")
	
	# Spawn all current players on new client
	#if world.get_node("Players").get_children() != null:
	rpc_unreliable("_update_player_movement", position, velocity) # Send each player to new dude
	#puppet_pos = position
	#puppet_vel = velocity
# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	# Sync to last known position and velocity
#	position = puppet_pos
#	velocity = puppet_vel
#
#	position += velocity * delta
#
#	# It may happen that many frames pass before the controlling player sends
#	# their position again. If we don't update puppet_pos to position after moving,
#	# we will keep jumping back until controlling player sends next position update.
#	# Therefore, we update puppet_pos to minimize jitter problems
#	puppet_pos = position
