#note: state machine changed by parent shovelgun node
extends Area2D

class_name Shovel

############ MECHANICS 

#gameplay
export(float) var speed = 1200 
export(float) var damage = 10
const detached_lifespan = 10 #time it survives without a shovelgun 

#implementation  
var last_owner_id : int 

#knockback stats
const knockback_speed = 2000 #game units per second
const knockback_duration = 0.1 #in seconds

func _ready():
	last_owner_id = int(HelperFunctions.get_parent_player_node(self).name)
	
func break_touched_block() -> void:
	if is_in_block(): 
		gamestate.blocks_node.break_block(
			gamestate.blocks_node.get_overlapping_cell($CollisionShape2D.global_position), 
			last_owner_id 
		)

func is_in_block() -> bool:
	var v = $CollisionShape2D.global_position
	var btm = gamestate.blocks_node #block_tile_map
	return not btm.get_overlapping_block_type(v) == gamestate.blocks_node.block.EMPTY
	
remotesync func destroy():
	queue_free()

#####COLLISIONS
func _on_body_entered(body):
	if $StateMachine.state.has_method("on_body_entered"):
		$StateMachine.state.on_body_entered(body)  

func _on_Shovel_area_entered(area): #hit an activated ShovelGun
	if $StateMachine.state.has_method("on_Shovel_area_entered"):
		$StateMachine.state.on_Shovel_area_entered(area)  
