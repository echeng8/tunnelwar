extends Node

export var gold_ratio = 0.25

func _ready():
	set_gold_blocks()
	
func regen_all_blocks():
	for block in get_children():
		block.set_default() 
		
	set_gold_blocks() 
			
func set_gold_blocks():
	var gold_blocks = get_child_count() * 0.25
	for n in range(0,gold_blocks):
		var random_block = get_children()[randi() % get_child_count()]
		
		#ensnure its not already gold
		while (random_block.get_node("StateMachine").state.name == "BGoldState"):
			random_block = get_children()[randi() % get_child_count()]
		
		random_block.get_node("StateMachine").change_to("BGoldState")
