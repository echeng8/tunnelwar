##Movement-denying Crowd Control State (e.g. knockbacked)
extends State

export var duration = 0

var knockback_source
func enter():
	pass
	duration = 0

func physics_process(delta):
	duration += delta

	if duration < knockback_source.knockback_duration:
		var move_dir = Vector2(1, 0).rotated(knockback_source.global_rotation).normalized()
		var velocity = move_dir * (knockback_source.knockback_speed)
		fsm_root.move_and_slide(velocity, Vector2(0,0))	#TODO consollidate this with prev code
		fsm_root.rpc_unreliable("set_player_position", fsm_root.position)
	
	else:
		fsm.change_to("PDefaultState")
	

	return
