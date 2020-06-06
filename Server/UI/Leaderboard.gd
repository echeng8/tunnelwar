extends Node

var player_rankings = [] 

func _ready():
	gamestate.world_node.connect("on_player_load", self, "connect_player_signals")
	gamestate.world_node.connect("on_player_unload", self, "update_rankings")
	
func update_ranking():
	player_rankings = [] 
	for child in gamestate.world_node.get_node("Players").get_children():
		player_rankings.append(child.name)
	player_rankings.sort_custom(self, "has_less_gold")
	
	rpc("set_rankings", player_rankings) 

func has_less_gold(p1_name : String, p2_name: String): 
	return gamestate.get_player(p1_name).get_gold() < gamestate.get_player(p2_name).get_gold()

func connect_player_signals(p_id):
	gamestate.get_player(p_id).connect("on_gold_change", self, "update_ranking") 
