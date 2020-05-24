extends "res://Scripts_General/Base_Classes/FSM/State.gd"
var ShovelNode
func enter():
	ShovelNode = get_parent().get_parent()
	assert(ShovelNode is Shovel)
	
	for body in ShovelNode.get_overlapping_bodies():
		try_damage_player(body)
# Opttttional handler functions for game loop events
func process(delta):
	pass
	
func on_body_entered(body):
	try_damage_player(body)

func try_damage_player(body):
	if body.has_method("get_struck_by"):
		body.get_struck_by(ShovelNode)
