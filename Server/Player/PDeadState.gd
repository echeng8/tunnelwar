extends State
#fsm_root = Player 


func enter():
	#drop gold
	gamestate.blocks_node.spawn_golds_at(gamestate.blocks_node.get_nearest_cell(fsm_root.position), fsm_root.get_gold())
	fsm_root.set_gold(0)
	
	#disable nodes
	fsm_root.visible = false 

	
func respawn() -> void:
	#reset health 
	fsm_root.health_points = fsm_root.base_hp 
	fsm_root.rpc("set_health", fsm_root.health_points)
	
	#reset position 
	var respawn_position = Vector2(0,0)
	var random_block = gamestate.blocks_node.get_random_block(gamestate.blocks_node.block.DIRT)
	if not random_block == null: #if no blocks exist 
		respawn_position = gamestate.blocks_node.get_pos(random_block)
	fsm_root.rpc("set_player_position", respawn_position)
	
	exit("PDefaultState") 
