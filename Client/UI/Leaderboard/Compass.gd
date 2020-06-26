extends Sprite

func _process(delta): 
	if get_parent().player_rankings.size() > 0:
		var target = gamestate.world_node.get_node("Players").get_node(get_parent().player_rankings[0])
		var my_id =  get_tree().get_network_unique_id()
		if not target.name == str(my_id): #if it's not you 
			visible = true 
			rotation = gamestate.get_player(my_id).position.angle_to_point(target.position) 
		else: 
			visible = false  
