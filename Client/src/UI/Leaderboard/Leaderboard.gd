extends Control
#
#
#var player_rankings = [] #[str] of player ids
#
#func _ready():
#	gamestate.world_node.connect("on_load_complete", self, "request_server_init")
#
#func request_server_init():
#	rpc_id(1, "initialize_rpc_sender")
#
#puppet func set_rankings(rankings):
#	if not gamestate.world_node.load_completed:
#		return 
#
#	player_rankings = rankings
#	update_display() 
#
#func update_display():
#	#todo queue until load complete? 
#	$RichTextLabel.text = "Richest Shoveling"
#	for i in range(0,player_rankings.size()) : 
#		var rank = i + 1
#		var username = gamestate.get_player(player_rankings[i]).username
#		var gold = gamestate.get_player(player_rankings[i]).gold
#
#		var entry = "%s. %s %s" % [rank, username, gold] 
#
#		$RichTextLabel.text	 += "\n" + entry 
#
#	if player_rankings.size() > 1:
#		var target = gamestate.get_player(player_rankings[0])
#		$Compass.point_to(target)
