extends Node
class_name Blocks 

#######GAME MECHANICS
var reset_blocks = 3 #number of reset blocks until game resets 
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

var reset_block = null #ref to the current resetblock

# adjacent list of open coordiantes; key - coordinate, val - open adjacents  
var open_adjlist = {} 

const chunk_length = 20

#########SIGNALS
signal on_block_edit_done  #emits when blocks destroyed or generated


######### private variables
var _chunks_generating = 0 

func _ready(): 
	#initialize 
	gamestate.blocks_node = self 
	
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
	yield(self, "on_block_edit_done")
	
	gamestate.set_game_phase(gamestate.game_phases.IN_PROGRESS)
	
	yield(get_tree().create_timer(10), "timeout") #time to load blocks 
	spawn_reset_block()

#prereq: all reset blocks destroyed
func reset():
	gamestate.set_game_phase(gamestate.game_phases.INTERIM)
	destroy_all_blocks(["Bedrock"])
	yield(self, "on_block_edit_done")
	gen_at_origin()  
	
	
####### RESET BLOCKS ########### 

#convert random block to reset
#pre-condition: blocks exist 
func spawn_reset_block():  
	gamestate.broadcast_node.broadcast("Somewhere, a Reset Block appears. %s more until RESET." % reset_blocks, rb_respawn_time, 1)
	
	var random_block = get_random_block("Dirt")
	var coord = random_block.coord
	random_block.destroy() 
	yield(get_tree(), "idle_frame") #let destruction occur (maybe better solution?)
	create_block("ResetBlock", coord) 
	
	block_dict[coord].connect("on_destroy", self, "on_reset_block_destroyed")
	reset_block = block_dict[coord]
	
func on_reset_block_destroyed(_coord): 
	reset_blocks -= 1 
	gamestate.broadcast_node.broadcast("The Reset Block is destroyed. %s remain." % reset_blocks, rb_respawn_time, 1)
	yield(get_tree().create_timer(rb_respawn_time), "timeout")

	if reset_blocks > 0 and block_dict.keys().size() > 0:
		spawn_reset_block()
	else: 
		gamestate.set_game_phase(gamestate.game_phases.INTERIM)
		reset() 

###### GOLD ######
func spawn_golds_at(pos : Vector2, gold_count : int): 
	var gold_to_spawn = gold_count
	var spawn_queue = [pos]
	while gold_to_spawn > 0 and spawn_queue.size() > 0: 
		var spawn_coord = spawn_queue.pop_front()
		
		if spawn_coord in block_dict: 
			continue 
		
		#create gold
		create_block("GoldOreBlock", spawn_coord)
		gold_to_spawn -= 1 
		
		#add additional empty spots 
		for dir in HelperFunctions.directions:
			if not spawn_coord + dir in block_dict:
				spawn_queue.append(spawn_coord + dir)
		
		yield(get_tree(), "idle_frame") 

####### UTILITY #########

#returns -1 if no blocks exist 
func get_random_block(type = ""):
	if type == "": 
		if get_child_count() > 0:
			return get_child(randi() % get_child_count())
		else:
			return null
	else: 
		var block_array = block_type[type]
		if block_array.size() > 0:
			return block_array[(randi() % block_type[type].size())]
		else:
			return null

#RETURNS if a block already exists at the location
func generate_chunk(origin_coord : Vector2):
	_chunks_generating += 1 
	
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
			yield(get_tree(),"idle_frame")
			
	_chunks_generating -= 1
	if _chunks_generating == 0:
		emit_signal("on_block_edit_done")

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
	
	emit_signal("on_block_edit_done")	
	
#deletes block from dictionaries, updates open adjlist
#connected to blocks ondestroy() 
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


#### helper func 
func is_generating() -> bool:
	return not _chunks_generating == 0
