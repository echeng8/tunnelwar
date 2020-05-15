extends "res://Scripts_General/Base_Classes/FSM/State.gd"
var player

func enter():
	player = get_parent().get_parent()
	assert(player is KinematicBody2D) 
	
	#todo death screen
	player.respawn(Vector2(gamestate.DEV_SPAWN_X, gamestate.DEV_SPAWN_Y), 40) ##placeholder
