extends Node
#class_name Blocks 
#
########GAME MECHANICS
#var reset_blocks = 3 #number of reset blocks until game resets 
#var rb_respawn_time = 3.0  #seconds until reset_block spawns 
#var no_block_time = 6.0 #seconds there is no blocks during a reset 
#
#export var gold_chance = 2 #chance a block is gold
########
#
##REFERENCES
#const Blocks = {
#	"ResetBlock" : preload("res://World/Blocks/ResetRock/ResetBlock.tscn"), 
#}
#
#var reset_block = null #ref to the current resetblock
#
#var block_dict = {}
##prereq: all reset blocks destroyed
#
######## RESET BLOCKS ########### 
#
##convert random block to reset
##pre-condition: blocks exist 
##func spawn_reset_block():  
##	gamestate.broadcast_node.broadcast("Somewhere, a Reset Block appears. %s more until RESET." % reset_blocks, rb_respawn_time, 1)
##
##	var random_block = get_random_block("Dirt")
##	var coord = random_block.coord
##	random_block.destroy() 
##	yield(get_tree(), "idle_frame") #let destruction occur (maybe better solution?)
##	create_block("ResetBlock", coord) 
##
##	instance.connect("on_destroy", self, "on_reset_block_destroyed")
##	reset_block = instance
#
##func on_reset_block_destroyed(_coord): 
##	reset_blocks -= 1 
##	gamestate.broadcast_node.broadcast("The Reset Block is destroyed. %s remain." % reset_blocks, rb_respawn_time, 1)
##	yield(get_tree().create_timer(rb_respawn_time), "timeout")
##
##	if reset_blocks > 0 and block_dict.keys().size() > 0:
##		spawn_reset_block()
##	else: 
##		gamestate.set_game_phase(gamestate.game_phases.INTERIM)
##		reset() 
#
####### GOLD ######
#func spawn_golds_at(pos : Vector2, gold_count : int): 
#	var gold_to_spawn = gold_count
#	var spawn_queue = [pos]
#	while gold_to_spawn > 0 and spawn_queue.size() > 0: 
#		var spawn_coord = spawn_queue.pop_front()
#
#		if spawn_coord in block_dict: 
#			continue 
#
#		#create gold
#		create_block("GoldOreBlock", spawn_coord)
#		gold_to_spawn -= 1 
#
#		#add additional empty spots 
#		for dir in HelperFunctions.directions:
#			if not spawn_coord + dir in block_dict:
#				spawn_queue.append(spawn_coord + dir)
#
#		yield(get_tree(), "idle_frame") 
#
######## UTILITY #########
#
#
##func create_block(block_name : String, coordinate: Vector2):
##	if coordinate in block_dict: 
##		block_dict[coordinate].destroy() 
##		yield(get_tree(), "idle_frame")
##
##	var instance = Blocks[block_name].instance()
##	instance.position = gamestate.get_pos(coordinate)
##	instance.connect("on_destroy", self, "_erase_block")
##	add_child(instance)
##
##	#adding references 
##	block_dict[instance.coord] = instance
##	block_type[instance.get_class()].append(instance)  
##
##	return instance 
#
##deletes block from dictionaries, updates open adjlist
##connected to blocks ondestroy() 
#func _erase_block(coordinate : Vector2):
#	var block_name = block_dict[coordinate].get_class()
#
#	block_type[block_name].erase(block_dict[coordinate])
#	block_dict.erase(coordinate)
#
##use coords
#func create_walls(block_name : String, top_left : Vector2, bottom_right : Vector2): 
#	for i in range(bottom_right.x - top_left.x + 1):
#		create_block(block_name, top_left + Vector2(i,0))
#		create_block(block_name, bottom_right + Vector2(-i,0))
#	for i in range(bottom_right.y - top_left.y + 1):
#		create_block(block_name, top_left + Vector2(0,i))
#		create_block(block_name, bottom_right + Vector2(0,-i))
#
#
##### helper func 
#func is_generating() -> bool:
#	return not _chunks_generating == 0
