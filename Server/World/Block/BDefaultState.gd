extends State

func enter():
	fsm_root.get_node("CollisionShape2D").set_deferred("disabled", false)
	
func get_struck_by(body):
	exit("BBrokenState")
