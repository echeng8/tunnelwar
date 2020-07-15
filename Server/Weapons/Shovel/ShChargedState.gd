extends State

func physics_process(delta):
	print(fsm_root.get_buried_percent())
	if fsm_root.get_buried_percent() > 0.5: 
		fsm_root.break_touched_block()
