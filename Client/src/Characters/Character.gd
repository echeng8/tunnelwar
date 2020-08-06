extends KinematicBody2D
class_name Character

signal on_state_change(state_name) 

#MECHANICS 
export var speed = 600
export var base_hp = 40


#runtime vars 
var velocity = Vector2.ZERO 
var hp = 40
var _gold = 0 setget set_gold, get_gold 

#input
var input_direction =  Vector2.ZERO
var input_aim_pos = Vector2.ZERO
var input_pull_p = false 

signal struck_by(node) 
signal on_gold_change

#dependencies
onready var state_machine : StateMachineUpdate = $StateMachine


func setup(hp : int, gold : int, state : String):
	hp = hp 
	_gold = gold 
	state_machine.change_to(state)

#gameplay
func get_struck_by(source):
	emit_signal("struck_by", source)

func respawn() -> void:
	if $StateMachine.state.has_method("respawn"):
		$StateMachine.state.respawn() 
		 
func disconnect_die() -> void:
	$StateMachine.change_to("PDeadState") 
	broadcast_death() 
	queue_free() 



func broadcast_death(killer_id = -1):
	pass 
#	var msg = ""
#	if not killer_id == -1:
#		msg = "[p] has killed %s for %s gold." % [username, _gold]
#	else: 
#		msg = "%s suddenly dies, dropping %s gold." % [username, _gold]
	#gamestate.broadcast_node.broadcast(msg, 3, 0, killer_id)
#
##SETTERS AND GETTERS
func set_gold(amount):
	_gold = amount 
	emit_signal("on_gold_change")

func get_gold():
	return _gold 

func add_gold(amount = 1):
	set_gold(get_gold() + amount) 
