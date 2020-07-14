extends State

var velocity = Vector2(0,0)
var duration = 0
var durability = 0 
func enter():
	velocity = Vector2(1,0).rotated(fsm_root.rotation) * fsm_root.speed
	durability = fsm_root.detached_durability 
	duration = 0
	
func process(delta):
		duration += delta
		
		if(duration > fsm_root.detached_lifespan):
			fsm_root.rpc("destroy")

func physics_process(delta):
	if velocity.length() > 0:
		fsm_root.position +=  velocity * delta
		fsm_root.rpc_unreliable("_update_shovel_position", fsm_root.position)
		

func on_body_entered(body):
	if body.is_in_group("Players") and body.has_method("get_struck_by"): 
		body.get_struck_by(fsm_root)
		velocity = Vector2(0,0)
		exit("ShPickUpState")

func on_Shovel_area_entered(area): #hit an activated ShovelGun
	if area.get_node("StateMachine").state.name == "ShChargedState": #reflected
		velocity = -velocity 
		#todo deflect refactor
		fsm_root.rotation = velocity.angle()  
		fsm_root.rpc("set_rotation", fsm_root.rotation) 
