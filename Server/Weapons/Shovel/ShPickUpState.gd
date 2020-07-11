extends State

var duration = 0

func enter():
	duration = 0
	
	
# optional handler functions for game loop events
func process(delta):
	duration += delta 
	if duration > fsm_root.detached_lifespan:
		fsm_root.rpc("destroy")

func on_body_entered(body):
	#player picks up shovelnode
	if body.is_in_group("Players") and not body.get_node("ShovelGun/Model").is_loaded() :
		body.get_node("ShovelGun/Model").rpc("reload")	 
		fsm_root.rpc("destroy")
