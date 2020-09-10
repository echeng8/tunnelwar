extends Node

func _ready():
	assert(get_tree().connect("network_peer_disconnected", self, "remove_player") == OK)
	gamestate.connect("on_match_begin", self, "clear_player_golds")
	gamestate.connect("on_player_id_set", self ,"update_player_owner") 
	
func clear_player_golds() -> void: 
	for player in get_children():
		player.set_gold(0)


func remove_player(id : int):
	var p = get_node_or_null(str(id))
	if not p == null:
		p.disconnect_die() 

func update_player_owner(index : int, id : int) -> void:
	var player = get_children()[index]
	player.rpc("set_network_owner", id)
	player.respawn()
