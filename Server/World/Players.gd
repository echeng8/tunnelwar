extends Node

func _ready():
	assert(gamestate.connect("on_player_remove", self, "clear_removed_player") == OK)
# warning-ignore:return_value_discarded
	gamestate.connect("on_match_begin", self, "clear_player_golds")
# warning-ignore:return_value_discarded
	gamestate.connect("on_player_id_set", self ,"update_player_owner") 
	
func clear_player_golds() -> void: 
	for player in get_children():
		player.set_gold(0)

func clear_removed_player(player_index : int) -> void: 
	get_player(player_index).clear_network_owner() 

func get_player(index : int):
	return get_children()[index]

func update_player_owner(index : int, id : int) -> void:
	var player = get_children()[index]
	player.set_network_owner(id) 
	player.respawn()
