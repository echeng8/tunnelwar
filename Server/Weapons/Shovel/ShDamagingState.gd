
extends "res://Scripts_General/Base_Classes/FSM/State.gd"

func enter():
	pass
# Opttttional handler functions for game loop events
func process(delta):
	pass
	
func on_body_entered(body):
	if body.is_in_group("Players") and body.has_method("get_struck_by"):
		body.get_struck_by(self)
