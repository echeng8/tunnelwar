extends Block

func get_struck_by(body):
	if not HelperFunctions.get_parent_player_node(body) == null: 
		HelperFunctions.get_parent_player_node(body).add_gold() 
	rpc("destroy")
