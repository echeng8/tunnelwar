extends Control

func _ready():
	rpc_id(1,"initialize_rpc_sender")
	
puppet func update_broadcast(message: String, target_player_id = -1) -> void :  
	visible = true 
	$RichTextLabel.bbcode_text = "[center]%s[/center]" % message
	
	if not target_player_id == -1:
		$Compass.point_to(gamestate.get_player(target_player_id))
	else: 
		$Compass.stop_pointing() 


puppet func end_broadcast() -> void: 
	visible = false 
	$RichTextLabel.text = ""
	$Compass.stop_pointing()
