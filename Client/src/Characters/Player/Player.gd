extends Character


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(_delta):
#	#send server user input
#	if is_network_master():
#		#movement
#		var leftValue = -Input.get_action_strength("left")
#		var rightValue = Input.get_action_strength("right")
#		var upValue = -Input.get_action_strength("up")
#		var downValue = Input.get_action_strength("down")
#		var movementValuesMerged = Vector2(leftValue + rightValue, upValue + downValue)
#		rset_unreliable_id(1, 'input_direction', movementValuesMerged)
#
#		#Combat 
#		rset_unreliable_id(1, "input_aim_pos", get_global_mouse_position()) #todo check for cheating potential 
#		rset_unreliable_id(1, "input_pull_p", Input.is_action_pressed('pull')) 
