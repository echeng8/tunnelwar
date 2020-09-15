extends Node

signal on_player_load

func _ready():
	gamestate.players_node = self  
	
	for child in get_children():
		child.connect("on_loaded", self, "emit_on_player_loaded")

func emit_on_player_loaded():
	emit_signal("on_player_load") 
