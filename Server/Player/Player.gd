extends KinematicBody2D

export var speed = 300
enum MoveDirection { UP, DOWN, LEFT, RIGHT, NONE }
#func _ready():
var player_direction =  MoveDirection.NONE # Just making sure we initilize it

func _process(delta):
	var move_dir = Vector2()
	match player_direction:
		MoveDirection.NONE:
			return
		MoveDirection.UP:
			move_dir.y -= 1
		MoveDirection.DOWN:
			move_dir.y += 1
		MoveDirection.LEFT:
			move_dir.x -= 1
		MoveDirection.RIGHT:
			move_dir.x += 1
			
	var velocity = move_dir * speed
	position += velocity * delta
	rpc_unreliable("_update_player_movement", name, position)

remote func _update_player_movement(direction):
	var id = get_tree().get_rpc_sender_id()
	if name == String(id):
		player_direction = direction
#	var velocity = direction * speed
#	var id = get_tree().get_rpc_sender_id()
#	var world = get_node("/root/World")
#	var players = world.get_node("Players").get_children()
#	for p in players:
#		if(p.name == String(id)):
#			p.position += velocity * delta
#			rpc_unreliable("_update_player_movement", id, p.position)





	#rpc_unreliable("_update_player_movement", id, player.position)
	#print(player.name)
	#player.position += velocity * delta
	#rpc_unreliable("_update_player_movement", id, player.position) 
#remote func _update_player_movement(direction, delta):
#	var velocity = direction * speed
#	var id = get_tree().get_rpc_sender_id()
#	var world = get_node("/root/World")
#	var player = world.get_node("Players").get_child(id)
#	print(player.name)
#	player.position += velocity * delta
#	rpc_unreliable("_update_player_movement", id, player.position) 
