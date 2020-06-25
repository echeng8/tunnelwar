extends Node

#######GAME MECHANICS
var reset_blocks = 2 #number of reset blocks until game resets 
var rb_respawn_time = 3.0  #seconds until reset_block spawns 
var no_block_time = 6.0 #seconds there is no blocks during a reset 

export var gold_chance = 2 #chance a block is gold
#######


#REFERENCES
const Blocks = {
	#scenes
	"Dirt": preload("res://World/Blocks/Dirt/Dirt.tscn"), 
	"GoldOreBlock" : preload("res://World/Blocks/GoldOreBlock/GoldOreBlock.tscn"),
	"ResetBlock" : preload("res://World/Blocks/ResetRock/ResetBlock.tscn"), 
	"Bedrock" : preload("res://World/Blocks/Bedrock/Bedrock.tscn")
}

#Block References - each reference same objects but use different keys 
var block_dict = {} # key = Vector2
var block_type = {} # key = "BlockName" - refers to arrays of blocks

var reset_block = null #ref to resetblock

const chunk_length = 20
#########SIGNALS
signal on_generation_done  #todo emit this

func _ready(): 
	#initialize 
	for block_name in Blocks.keys():
		block_type[block_name] = []
	
	gen_at_origin(true) #todo move this to gamestate


## WORLD MANAGEMENT
func gen_at_origin(create_border = false):
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
	
	if create_border:
		create_walls(
			"Bedrock", 
			surrounding_chunks['tl'] + chunk_size/2 * -1 - Vector2(1,1),
			surrounding_chunks['br'] + chunk_size/2
		)
		
	#todo refactor this 
	yield(get_tree().create_timer(10), "timeout") #time to load blocks 
	spawn_reset_block()
	
func reset():
	destroy_all_blocks(["Bedrock"])
	while block_dict.values().size() > 0:
		yield(get_tree(), "idle_frame")
	#yield(get_tree().create_timer(no_block_time), "timeout")
	gen_at_origin()  
	
	
####### RESET BLOCKS ########### 

#convert random block to reset
#pre-condition: blocks exist 
func spawn_reset_block():  
	var random_block = get_random_block("Dirt")
	var coord = random_block.coord
	random_block.destroy() 
	yield(get_tree(), "idle_frame") #let destruction occur (maybe better solution?)
	create_block("ResetBlock", coord) 
	
	block_dict[coord].connect("on_destroy", self, "on_reset_block_destroyed")
	reset_block = block_dict[coord]
	
func on_reset_block_destroyed(_coord): 
	yield(get_tree().create_timer(rb_respawn_time), "timeout")
	
	if reset_blocks > 0 and block_dict.keys().size() > 0:
		spawn_reset_block()
		reset_blocks -= 1 
	else: 
		reset() 


####### UTILITY #########

#doesn't return bedrock blocks, which aren't part of block_dict 
#returns -1 if no blocks exist 
func get_random_block(type = ""):
	if type == "": 
		if get_child_count() > 0:
			return get_child(randi() % get_child_count())
		else:
			return -1 
	else: 
		var block_array = block_type[type]
		if block_array.size() > 0:
			return block_array[(randi() % block_type[type].size())]
		else:
			return -1 

#RETURNS if a block already exists at the location
func generate_chunk(origin_coord : Vector2):
	if origin_coord in block_dict:
		return 
		
	var top_left = origin_coord - Vector2(chunk_length/2, chunk_length/2)
	
	for row in range(chunk_length):
		for col in range(chunk_length):
			var block_name = ""
			if randi() % 100 <= gold_chance:
				block_name = "GoldOreBlock"
			else:
				block_name = "Dirt"
			create_block(block_name, top_left + Vector2(row, col))
			if randi() % 10 < 5:
				yield(get_tree(),"idle_frame")


func create_block(block_name : String, coordinate: Vector2):
	if coordinate in block_dict: 
		block_dict[coordinate].destroy() 
		yield(get_tree(), "idle_frame")
		
	var instance = Blocks[block_name].instance()
	instance.position = gamestate.get_pos(coordinate)
	instance.connect("on_destroy", self, "_erase_block")
	add_child(instance)
	
	#adding references 
	block_dict[instance.coord] = instance
	block_type[instance.get_class()].append(instance)  
	
	return instance 

	
func destroy_all_blocks(excluded_blocks = []):
	for block in block_dict.values(): 
		if(is_instance_valid(block) and not block.get_class() in excluded_blocks):
			block.destroy()
			if randi() % 10 < 2:
				yield(get_tree(),"idle_frame")
					
#deletes block from dictionaries, connected to blocks ondestroy() 
func _erase_block(coordinate : Vector2):
	var block_name = block_dict[coordinate].get_class()
	
	block_type[block_name].erase(block_dict[coordinate])
	block_dict.erase(coordinate)

#use coords
func create_walls(block_name : String, top_left : Vector2, bottom_right : Vector2): 
	for i in range(bottom_right.x - top_left.x + 1):
		create_block(block_name, top_left + Vector2(i,0))
		create_block(block_name, bottom_right + Vector2(-i,0))
	for i in range(bottom_right.y - top_left.y + 1):
		create_block(block_name, top_left + Vector2(0,i))
		create_block(block_name, bottom_right + Vector2(0,-i))
