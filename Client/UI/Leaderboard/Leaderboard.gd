extends Control

remotesync var player_rankings = []  setget player_rankings_set #[int] index of players

func _ready():
	rpc_id(1, "initialize_rpc_sender")
	
	gamestate.players_node.connect("on_player_load", self, "update_display") 

func player_rankings_set(ranking : Array) -> void:
	player_rankings = ranking 
	update_display()

func update_display():
	#todo queue until load complete? 
	$RichTextLabel.text = "Richest Shoveling"
	for i in range(0,player_rankings.size()) : 
		var rank = i + 1
		var username = gamestate.get_player_by_index(player_rankings[i]).username
		var gold = gamestate.get_player_by_index(player_rankings[i]).gold
		var entry = "%s. %s %s" % [rank, username, gold] 
		
		$RichTextLabel.text	 += "\n" + entry 

	if player_rankings.size() > 1:
		var target = gamestate.get_player_by_index(player_rankings[0])
		$Compass.point_to(target)
