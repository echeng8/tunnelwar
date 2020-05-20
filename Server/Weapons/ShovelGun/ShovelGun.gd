extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

#gameplay values 
export var stab_charge_time = 0.5 #seconds you need to pull back in order to stab
export var vulnerability_time = 0.5 #seconds you are vulnerable after the stab TODO
export var slowed_move_rate = .5

#pull back animation 
export var pull_back_dist = -2500

#stabbing animation
export var stabbing_dist = 5000
export var init_position = Vector2(-10,17)

#animation duration (how long it takes for the anim to complete)
export var pull_dur = 1.5
export var stab_dur = 0.5
export var reset_dur = 1

#implementation global var
var normal_speed_rate = 1

#Client Set Variables 
puppet var mousepos = Vector2(0,0) #todo cheater check
puppet var stab_btn_p = false # stab button just pressed
puppet var shoot_btn_p = false # shoot button pressed

onready var TweenNode = get_node("Tween")

func _ready():
	#set animation durations on client
	rset("pull_dur", pull_dur)
	rset("stab_dur", stab_dur)
	rset("reset_dur", reset_dur)
	
var velocity = Vector2.ZERO
var newPos = Vector2.ZERO	

######ANIMATION FUNCTIONS to be called by states (todo put in states?)
		
remotesync func _pre_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", self.position, newPos, pull_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()

remotesync func _stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", currPos, newPos, stab_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.start()
	
remotesync func _after_stabbing(currPos, newPos):
	TweenNode.interpolate_property(self, "position", currPos, newPos, reset_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT) #todo fix pull-back duration not actually working
	TweenNode.start()


		
##SHOOTING STUFF #########################################################
func shoot():
	if not isLoaded():
		return 
	
	$Reload.start()
	var ShovelNode = get_node("Shovel")
	HelperFunctions.rpc("reparent", get_node("Shovel").get_path(), "/root/World/Items", true) #TODO server > client side projectile codeZ
	ShovelNode.get_node("StateMachine").change_to("ShShotState")



func isLoaded(): 
	return has_node("Shovel")
	
func _on_Reload_timeout():
	$Reload.stop()
	if not isLoaded():
		rpc("reload")
	
remotesync func reload():
	var shovel = Shovel.instance()
	call_deferred("add_child", shovel)

