extends Camera2D

func _ready():
	if is_instance_valid(gamestate.user_player):
		align_with_player()
	else:
		gamestate.connect("on_user_player_set", self, "align_with_player")
	
func align_with_player() -> void:
	get_parent().remove_child(self) 
	gamestate.user_player.add_child(self) 

func _process(delta):
	align() 
