extends StaticBody2D

class_name Block

var coord

signal on_destroy(coordinates)

func _ready(): 
	coord = gamestate.get_coord(position)
	name = str("x", coord.x, "y", coord.y) 
	
	for player in gamestate.get_players():
		spawn_on_client(player.name) 
	
func spawn_on_client(id):
	gamestate.world_node.rpc_id(int(id), "spawn", "Block", name, HelperFunctions.get_transform_dict(self))
	rpc("set_server_state", filename.get_file().get_basename()) 

remotesync func destroy(): 
	emit_signal("on_destroy", coord)
	queue_free() 

func get_struck_by(body):
	return 
