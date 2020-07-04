extends State

var ShovelNode

func enter():
	ShovelNode = fsm_root
	assert(ShovelNode is Shovel)
	
	for body in ShovelNode.get_overlapping_bodies():
		try_damage_player(body)

func physics_process(delta):
	var body = gamestate.blocks_node
	body.break_block(
		body.get_nearest_cell(fsm_root.global_position), 
		int(HelperFunctions.get_parent_player_node(fsm_root).name)
	)

func on_body_entered(body):
	try_damage_player(body)

func try_damage_player(body):
	if body.has_method("get_struck_by"):
		body.get_struck_by(ShovelNode)
