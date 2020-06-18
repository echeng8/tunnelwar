extends Node

const Blocks = {
	#scenes
	"Dirt": preload("res://World/Blocks/Dirt/Dirt.tscn"), 
	"GoldOre" : preload("res://World/Blocks/GoldOreBlock/GoldOreBlock.tscn"),
}

const chunk_length = 20


export var gold_ratio = 0.25

func _ready():
	generate_chunk(Vector2(0,0))

func generate_chunk(origin_coord : Vector2):
	var top_left = origin_coord - Vector2(chunk_length/2, chunk_length/2)
	
	for row in range(20):
		for col in range(20):
			var instance = Blocks["Dirt"].instance() 
			instance.position = gamestate.get_pos(top_left + Vector2(row, col))
			add_child(instance) 
	
func regen_all_blocks():
	for block in get_children():
		block.set_default() 

func set_gold_blocks():
	var gold_blocks = get_child_count() * 0.25
	for n in range(0,gold_blocks):
		var random_block = get_children()[randi() % get_child_count()]
		
		#ensnure its not already gold
		while (random_block.get_node("StateMachine").state.name == "BGoldState"):
			random_block = get_children()[randi() % get_child_count()]
		
		random_block.get_node("StateMachine").change_to("BGoldState")
