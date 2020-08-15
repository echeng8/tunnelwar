extends TileMap

#MAKE SURE this lines up with tilesheet
enum block {DIRT, GOLD, BEDROCK, RESET, EMPTY}
puppet var initialized = false  
var length = 100

# Called when the node enters the scene tree for the first time.
func _ready():
	rpc("initialize_rpc_sender")
#	while not initialized:
#		rpc("initialize_rpc_sender") 
#		yield(get_tree().create_timer(1), "timeout")
	#z index settings
	tile_set.tile_set_z_index(block.EMPTY,z_index * -1)

#blocks [[cell type 0], [cell type 1], ...] 
puppet func load_world(block_arrays : Array) -> void: 
	for tile_index in range(block_arrays.size()): 
		for i in range(block_arrays[tile_index].size()):
			set_cellv(block_arrays[tile_index][i], tile_index)

puppet func set_block(cell : Vector2, index : int):
	set_cellv(cell, index)
