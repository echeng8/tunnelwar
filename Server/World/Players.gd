extends Node

func _ready():
	gamestate.connect("on_match_begin", self, "clear_player_golds")

func clear_player_golds() -> void: 
	for player in get_children():
		player.set_gold(0)
