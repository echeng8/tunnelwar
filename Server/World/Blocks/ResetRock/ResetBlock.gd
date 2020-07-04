extends Block
class_name ResetBlock

var hitpoints = 10 #how many strikes it takes to break


func get_class(): 
	return "ResetBlock" 
	
func get_struck_by(body):
	var parent_player =  HelperFunctions.get_parent_player_node(body) 
	
	if not parent_player == null:
		if hitpoints <= 0: 
			destroy()
		else:
			gamestate.broadcast_node.broadcast("[p] strikes the Reset Block. %s hits remain." % hitpoints, 3, 1, int(parent_player.name))
			hitpoints -=  1 
