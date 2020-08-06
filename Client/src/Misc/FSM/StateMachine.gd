extends Node

class_name StateMachine

signal on_state_change

const DEBUG = false

var state: Object

var history = []

func _ready():
	# Set the initial state to the first child node
	state = get_child(0)
	_enter_state()
	
func change_to(new_state):
	history.append(state.name)
	state = get_node(new_state)
	_enter_state()
	
	emit_signal("on_state_change")

func back():
	if history.size() > 0:
		state = get_node(history.pop_back())
		_enter_state()

func _enter_state():
	if DEBUG:
		print("Entering state: ", state.name)
	# Give the new state a reference to this state machine script
	state.fsm = self
	state.fsm_root = get_parent()
	state.enter()
