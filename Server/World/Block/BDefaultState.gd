extends State

func get_struck_by(body):
	fsm_root.rpc("break_block") 
	exit("BBrokenState")
