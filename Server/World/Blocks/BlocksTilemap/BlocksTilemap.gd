extends TileMap

var length = 100

enum blocks {DIRT, GOLD}
const block_num = 2 

# Called when the node enters the scene tree for the first time.
func _ready():
	gamestate.blocks_node = self 
	
	generate_dirt(length)
	generate_gold(5)
	get_random_block()

master func initialize_rpc_sender() -> void:
	var block_array = []
	for i in range(block_num):
		block_array.append(get_used_cells_by_id(i))
	
	rpc_id(get_tree().get_rpc_sender_id(), "load_world", block_array)
	
func reset():
	gamestate.set_game_phase(gamestate.game_phases.INTERIM)
	#todo reset
	
func generate_dirt(square_length : int) -> void: 
	for r in range(square_length):
		for c in range(square_length):
			var coords = Vector2(r,c)
			set_cellv(coords, 0)

func generate_gold(num: int):
	for i in range(num):
		var coords = Vector2(randi() % length, randi() % length)
		set_cellv(coords, 1)

func set_block(cell : Vector2, index : int) -> void: 
	set_cellv(cell, index) 
	rpc("set_block", cell, index)
	
func break_block(cell : Vector2, player_id = -1):
	if get_cellv(cell) == 0:
		set_block(cell, -1)

#returns -1 if no blocks exist 
func get_random_block(_type = "") -> Vector2:
	return get_used_cells()[randi() % get_used_cells().size()]

func get_pos(cell_coord : Vector2):
	return map_to_world(cell_coord) * scale.x 

func get_nearest_cell(world_pos : Vector2) -> Vector2:
	return Vector2(stepify(world_pos.x - 100, 200), stepify(world_pos.y - 100, 200)) / 200
