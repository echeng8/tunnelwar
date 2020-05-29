extends Node

func regen_all_blocks():
	for block in get_children():
		if block.broken == true:
			block.set_broken(false)  
