extends CanvasLayer

func _ready():
	gamestate.connect("on_match_begin", self, "broadcast_game_start")
	gamestate.connect("on_match_end", self, "broadcast_reset") 
	
func broadcast_reset(): 
	var winner = $Leaderboard.get_winner()
	$Broadcast.broadcast("[p] WINS THE MATCH. Resetting now.", 10, 1, int(winner.name))

func broadcast_game_start():
	$Broadcast.broadcast("BEGIN MATCH: Whoever has the MOST gold when the FINAL Reset Block is destroyed WINS.", 5, 1)
	print('te')
