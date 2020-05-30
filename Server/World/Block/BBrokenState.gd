extends State

func enter():
	fsm_root.get_node("CollisionShape2D").set_deferred("disabled", true)
