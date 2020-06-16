extends StaticBody2D

class_name Block

func _ready(): 
	for player in gamestate.get_players():
		spawn_on_client(player.name) 
	
func spawn_on_client(id):
	gamestate.world_node.rpc_id(id, "spawn", "Block", name, HelperFunctions.get_transform_dict(self))
	rpc("set_server_state", filename) 

remotesync func destroy(): 
	queue_free()
	
	
func get_struck_by(body):
	return 
