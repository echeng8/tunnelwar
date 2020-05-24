extends Node2D

var broke = false

remote func set_broke(is_broke):
	broke = is_broke
	if(broke):
		visible = false 
