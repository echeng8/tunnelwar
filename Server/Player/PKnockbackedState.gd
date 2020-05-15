##Movement-denying Crowd Control State (e.g. knockbacked)
extends "res://Scripts_General/Base_Classes/FSM/State.gd"

export var duration = 0
var player

var knockback_source
func enter():
	player = get_parent().get_parent()
	assert(player is KinematicBody2D) 

	duration = 0

func physics_process(delta):
	duration += delta

	if duration < knockback_source.knockback_duration:
		var move_dir = Vector2(1, 0).rotated(knockback_source.global_rotation).normalized()
		var velocity = move_dir * (knockback_source.knockback_speed)
		player.move_and_slide(velocity, Vector2(0,0))	#TODO consollidate this with prev code
		player.rpc_unreliable("set_player_position", player.position)
	
	else:
		fsm.change_to("PDefaultState")
	

	return
