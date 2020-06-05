extends State

#MECHANICS

func get_struck_by(body):
	HelperFunctions.get_parent_player_node(body).add_gold() 
	exit("BBrokenState")
