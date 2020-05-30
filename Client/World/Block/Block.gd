extends Node2D

var broken = false 

signal on_break

puppet func break_block():
	set_broken(true)
	emit_signal("on_break")
	
	
remotesync func set_broken(is_broke):		
	broken = is_broke 
	visible = not broken

func _ready():
	rpc_id(1, "init_on_client", get_tree().get_network_unique_id()) 
