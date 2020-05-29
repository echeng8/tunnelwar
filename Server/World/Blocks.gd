extends Node

func regen_all_blocks():
	for block in get_children():
		if block.is_broken():
			block.set_broken(false)  
