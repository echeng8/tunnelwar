extends Node

var player_rankings = [] #ids as strings 

func _ready():
	gamestate.world_node.connect("on_player_load", self, "connect_player_signals")
	gamestate.world_node.connect("on_player_load", self, "update_rankings")
	get_tree().connect("network_peer_disconnected", self, "update_rankings")
	update_rankings() 

remote func initialize_rpc_sender():
	rpc_id(get_tree().get_rpc_sender_id(), "set_rankings", player_rankings) 

func update_rankings(_id = -1): #dummy arg to get signals to work
	player_rankings = [] 
	for child in gamestate.world_node.get_node("Players").get_children():
		player_rankings.append(child.name)
	player_rankings.sort_custom(self, "sort_descending_gold")
	
	rpc("set_rankings", player_rankings) 

func sort_descending_gold(p1_name : String, p2_name: String): 
	return gamestate.get_player(p1_name).get_gold() > gamestate.get_player(p2_name).get_gold()

func connect_player_signals(p_id):
	gamestate.get_player(p_id).connect("on_gold_change", self, "update_rankings") 

func get_winner() -> Player: 
	return gamestate.get_player(player_rankings[0])
