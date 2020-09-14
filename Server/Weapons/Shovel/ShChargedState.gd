extends State

func physics_process(_delta):
	fsm_root.break_touched_block()
