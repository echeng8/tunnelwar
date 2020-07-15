extends State

var ShovelNode

func enter():
	ShovelNode = fsm_root
	assert(ShovelNode is Shovel)
	
	for body in ShovelNode.get_overlapping_bodies():
		try_damage_player(body)

func physics_process(delta):
	if fsm_root.get_buried_percent() == 1: 
		fsm_root.break_touched_block()

func on_body_entered(body):
	try_damage_player(body)

func try_damage_player(body):
	if body.has_method("get_struck_by"):
		body.get_struck_by(ShovelNode)
