extends State
#fsm_root = Player 


func enter():
	#drop gold
	gamestate.blocks_node.spawn_golds_at(gamestate.get_coord(fsm_root.position), fsm_root.get_gold())
	fsm_root.set_gold(0)
	
	#disable nodes
	fsm_root.visible = false 

	
func respawn() -> void:
	#reset health 
	fsm_root.health_points = fsm_root.base_hp 
	fsm_root.rpc("set_health", fsm_root.health_points)
	
	#reset position 
	var respawn_position = Vector2(0,0)
	var random_block = gamestate.world_node.get_node("Blocks").get_random_block("Dirt")
	if not random_block == null: #if no blocks exist 
		respawn_position = random_block.position
	fsm_root.rpc("set_player_position", respawn_position)
	
	exit("PDefaultState") 
