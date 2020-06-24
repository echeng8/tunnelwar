extends Node

#######GAME MECHANICS
var reset_blocks = 2 #number of reset blocks until game resets 
var rb_respawn_time = 3.0  #seconds until reset_block respawns
var no_block_time = 6.0 #seconds there is no blocks during a reset 

export var gold_chance = 2 #chance a block is gold
#######


#REFERENCES
const Blocks = {
	#scenes
	"Dirt": preload("res://World/Blocks/Dirt/Dirt.tscn"), 
	"GoldOre" : preload("res://World/Blocks/GoldOreBlock/GoldOreBlock.tscn"),
	"ResetBlock" : preload("res://World/Blocks/ResetRock/ResetBlock.tscn"), 
	"Bedrock" : preload("res://World/Blocks/Bedrock/Bedrock.tscn")
}
var block_dict = {} # key = Vector2
var reset_block = null #ref to resetblock


func _ready(): 
	gen_at_origin() #todo move this to gamestate
	
const chunk_length = 20


## WORLD MANAGEMENT
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
	
	spawn_reset_block()
	
	create_walls(
		"Bedrock", 
		surrounding_chunks['tl'] + chunk_size/2 * -1 - Vector2(1,1),
		surrounding_chunks['br'] + chunk_size/2
	)

func reset():
	destroy_all_blocks()
	while block_dict.values().size() > 0:
		yield(get_tree(), "idle_frame")
	#yield(get_tree().create_timer(no_block_time), "timeout")
	gen_at_origin()  
	
	
####### RESET BLOCKS ########### 

#convert random block to reset
#pre-condition: blocks exist 
func spawn_reset_block():  
	var random_block = get_random_block()
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
func get_random_block(): #todo add filters
	return get_child(randi() % get_child_count())

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
	block_dict[instance.coord] = instance
	return instance 

	
func destroy_all_blocks():
	for block in block_dict.values(): 
		if(is_instance_valid(block)):
			block.destroy()
			if randi() % 10 < 2:
				yield(get_tree(),"idle_frame")
					
#deletes block from dictionary, connected to blocks ondestroy() 
func _erase_block(coordinate : Vector2): 
	block_dict.erase(coordinate)

#use coords
func create_walls(block_name : String, top_left : Vector2, bottom_right : Vector2): 
	for i in range(bottom_right.x - top_left.x + 1):
		create_block(block_name, top_left + Vector2(i,0))
		create_block(block_name, bottom_right + Vector2(-i,0))
	for i in range(bottom_right.y - top_left.y + 1):
		create_block(block_name, top_left + Vector2(0,i))
		create_block(block_name, bottom_right + Vector2(0,-i))
