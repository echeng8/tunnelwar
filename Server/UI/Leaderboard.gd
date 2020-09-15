extends Node

remotesync var player_rankings = [] # [int] as player indexes  

func _ready():
# warning-ignore:return_value_discarded
	gamestate.world_node.get_node("Players").connect("on_player_change", self, "update_rankings")
	update_rankings() 

remote func initialize_rpc_sender():
	rset_id(get_tree().get_rpc_sender_id(), "player_rankings", player_rankings) 

func update_rankings(_id = -1): #dummy arg to get signals to work
	var rankings = [] 
	for child in gamestate.world_node.get_node("Players").get_children():
		rankings.append(child.get_index())
	player_rankings.sort_custom(self, "sort_descending_gold")
	
	rset("player_rankings", rankings) 

func sort_descending_gold(p1_index : int, p2_index: int): 
	return gamestate.get_player_by_index(p1_index).get_gold() > gamestate.get_player_by_index(p2_index).get_gold()

func get_winner() -> Player: 
	return gamestate.get_player(player_rankings[0])
