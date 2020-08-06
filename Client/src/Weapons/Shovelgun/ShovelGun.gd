#extends Sprite
#
##const Shovel #= preload("res://Weapons/Shovel/Shovel.tscn")
#
##references
#var sg_player : Player 
#
#var server_state : String
#
#var time_in_state = 0 
#
#remote var pull_time : float
#remote var shoot_charge_time : float   
#
##debug variables
#var debug_pulled = false
#signal on_server_state_change
#func _ready():
#	var shovel = get_node("Shovel")
#	rpc_id(1, "initialize_rpc_sender")  
#
#	sg_player = HelperFunctions.get_parent_player_node(self) 
#
#remote func server_set_transform(rot, pos):
#	global_rotation = rot
#	global_position = pos 
#
####ANIMATION ##################
#remote func update_client_state(s_state : String):
#	server_state = s_state
#	time_in_state = 0 
#	emit_signal("on_server_state_change", server_state)
#	match server_state:
#		"SGDefaultState":
#			sg_player.get_node("PlayerSprites").switch_face("normal")
#		"SGPulledState":
#			sg_player.get_node("PlayerSprites").switch_face("scared")
#		"SGStabState":
#			pass
#		"SGVulnerableState": 
#			pass 
#
##todo refactor?  
#func _process(delta):
#	time_in_state += delta
#	match server_state:
#		"SGPulledState": 
#			if time_in_state > shoot_charge_time + pull_time:
#				sg_player.get_node("PlayerSprites").get_node("ExclaimMark").visible = true
#		_:
#			sg_player.get_node("PlayerSprites").get_node("ExclaimMark").visible = false 
#
###SHOOTING STUFF ###############
#remotesync func shoot():
#	$Shovel.visible = false  
#remotesync func reload():
#	$Shovel.visible = true
#
### DEBUG
#func check_for_command(message: String):
#	if(message == "/toggle_pulled"):
#		debug_pulled = not debug_pulled
