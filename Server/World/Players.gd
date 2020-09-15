extends Node

#emitted on gold change or network owner change
signal on_player_change

func _ready():
# warning-ignore:return_value_discarded
	gamestate.connect("on_player_remove", self, "clear_removed_player")
# warning-ignore:return_value_discarded
	gamestate.connect("on_match_begin", self, "clear_player_golds")
# warning-ignore:return_value_discarded
	gamestate.connect("on_player_id_set", self ,"update_player_owner") 

#listen to player signals
	for player in get_children():
		player.connect("on_gold_change", self, "on_player_gold_change")

func clear_removed_player(player_index : int) -> void: 
	get_player(player_index).clear_network_owner() 
	emit_signal("on_player_change") 
	
func clear_player_golds() -> void: 
	for player in get_children():
		player.set_gold(0)
	emit_signal("on_player_change")
	
func update_player_owner(index : int, id : int) -> void:
	var player = get_children()[index]
	player.set_network_owner(id) 
	player.respawn()
	emit_signal("on_player_change") 

func on_player_gold_change():
	emit_signal("on_player_change") 

#helper functions 
func get_player(index : int):
	return get_children()[index]
