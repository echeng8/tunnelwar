extends Node

var block_dict = {} # key = Vector2

const Blocks = {
	#scenes
	"Dirt": preload("res://World/Blocks/Dirt/Dirt.tscn"), 
	"GoldOre" : preload("res://World/Blocks/GoldOreBlock/GoldOreBlock.tscn"),
}

func _ready(): 
	gen_at_origin()
	
const chunk_length = 20

#chance a block is gold
export var gold_chance = 2 

func destroy_all_blocks():
	for block in get_children(): 
		block.rpc("destroy")

##WORLD SPAWNING
func gen_at_origin():
	var origin = Vector2(0,0)
	generate_chunk(origin)
	
	var chunk_size = Vector2(chunk_length, chunk_length)  
	
	var surrounding_chunks = {}
	surrounding_chunks['br'] = origin + chunk_size
	surrounding_chunks['b'] = origin + chunk_size * Vector2(0,1)
	surrounding_chunks['bl'] = origin + chunk_size * Vector2(-1,1)
	surrounding_chunks['l'] = origin + chunk_size * Vector2(-1, 0)
	surrounding_chunks['r'] = origin + chunk_size * Vector2(1,0)
	surrounding_chunks['tl'] = origin + chunk_size * -1
	surrounding_chunks['t'] = origin + chunk_size * Vector2(0,-1)
	surrounding_chunks['tr'] = origin + chunk_size * Vector2(1,-1)
	
	for coord in surrounding_chunks.values():
		generate_chunk(coord)
		
#RETURNS if a block already exists at the location
func generate_chunk(origin_coord : Vector2):
	if origin_coord in block_dict:
		return 
		
	var top_left = origin_coord - Vector2(chunk_length/2, chunk_length/2)
	
	for row in range(chunk_length):
		for col in range(chunk_length):
			var instance = null
			if randi() % 100 <= gold_chance:
				instance = Blocks["GoldOre"].instance()
			else:
				instance = Blocks["Dirt"].instance() 
				
			instance.position = gamestate.get_pos(top_left + Vector2(row, col))
			instance.connect("on_destroy", self, "erase_block")
			add_child(instance)
			block_dict[instance.coord] = instance

func get_random_block():
	return get_child(randi() % get_child_count())

func erase_block(coordinate : Vector2): 
	block_dict.erase(coordinate)
