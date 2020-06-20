extends Node

#######GAME MECHANICS
var reset_blocks = 3 #number of reset blocks until game resets 
#######

var block_dict = {} # key = Vector2

const Blocks = {
	#scenes
	"Dirt": preload("res://World/Blocks/Dirt/Dirt.tscn"), 
	"GoldOre" : preload("res://World/Blocks/GoldOreBlock/GoldOreBlock.tscn"),
	"ResetBlock" : preload("res://World/Blocks/ResetRock/ResetBlock.tscn")
}

func _ready(): 
	gen_at_origin()
	
const chunk_length = 20

#chance a block is gold
export var gold_chance = 2 


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

func destroy_all_blocks():
	for block in get_children(): 
		block.rpc("destroy")


#RETURNS if a block already exists at the location
func generate_chunk(origin_coord : Vector2):
	if origin_coord in block_dict:
		return 
		
	var top_left = origin_coord - Vector2(chunk_length/2, chunk_length/2)
	
	for row in range(chunk_length):
		for col in range(chunk_length):
			var block_name = ""
			if randi() % 100 <= gold_chance:
				block_name = "GoldOre"
			else:
				block_name = "Dirt"
			create_block(block_name, top_left + Vector2(row, col))

#convert random block to reset
#pre-condition: blocks exist 
func spawn_reset_block():  
	var random_block = get_random_block()
	var coord = random_block.coord
	random_block.destroy() 
	create_block("ResetBlock", coord) 
	
	block_dict[coord].connect("on_destroy", self, "on_reset_block_destroyed")

func on_reset_block_destroyed(): 
	if reset_blocks > 0 and block_dict.keys.length > 0:
		spawn_reset_block()
		reset_blocks -= 1 
	else: 
		destroy_all_blocks()
		#todo bring them back after delay
		
func get_random_block():
	return get_child(randi() % get_child_count())

func create_block(block_name : String, coordinate: Vector2):
	var instance = Blocks[block_name].instance()
	instance.position = gamestate.get_pos(coordinate)
	instance.connect("on_destroy", self, "_erase_block")
	add_child(instance)
	block_dict[instance.coord] = instance
	
#deletes block from dictionary, connected to blocks ondestroy() 
func _erase_block(coordinate : Vector2): 
	block_dict.erase(coordinate)
