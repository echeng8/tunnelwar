extends Control


var player_rankings = [] #[str] of player ids

func _ready():
	gamestate.world_node.connect("on_load_complete", self, "request_server_init")
	
func request_server_init():
	rpc_id(1, "initialize_rpc_sender")

puppet func set_rankings(rankings):
	if not gamestate.world_node.load_completed:
		return 
		
	player_rankings = rankings
	update_display() 
	
func update_display():
	#todo queue until load complete? 
	$RichTextLabel.text = "Richest Shoveling"
	for i in range(0,player_rankings.size()) : 
		var rank = i + 1
		var username = gamestate.get_player(player_rankings[i]).username
		var gold = gamestate.get_player(player_rankings[i]).gold
		
		var entry = "%s. %s %s" % [rank, username, gold] 
		
		$RichTextLabel.text	 += "\n" + entry 

	if player_rankings.size() > 1:
		var target = gamestate.get_player(player_rankings[0])
		var my_id =  get_tree().get_network_unique_id()
		if not target.name == str(my_id): #if it's not you 
			$Compass.point_to(gamestate.get_player(my_id), target)
		else:
			$Compass.stop_pointing() 
