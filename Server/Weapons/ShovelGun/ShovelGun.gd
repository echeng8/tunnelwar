extends Sprite

const Shovel = preload("res://Weapons/Shovel/Shovel.tscn")

signal shoot

#gameplay values 
export var stab_charge_time = 0.5 #seconds you need to pull back in order to stab
export var vulnerability_time = 0.75 #seconds you are vulnerable after the stab TODO
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
var player_id

#Client Set Variables 
puppet var mousepos = Vector2(0,0) #todo cheater check
puppet var stab_btn_p = false # stab button just pressed
puppet var shoot_btn_p = false # shoot button pressed

onready var TweenNode = get_node("Tween")
var ShovelNode

func setup():
	player_id = get_parent().name
	name = name + player_id
	
	$Shovel.setup()
	ShovelNode = get_node("Shovel" + player_id)
	
	#shooting todo make state

	ShovelNode.connect("_pick_up", self, "_on_shovel_pick_up")

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

remotesync func _stabbing(player_id, currPos, newPos):
	TweenNode.interpolate_property(self, "position", currPos, newPos, stab_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT)
	TweenNode.interpolate_callback(self, stab_dur, "_disable_collision", ShovelNode, true) 
	TweenNode.start()
	
remotesync func _after_stabbing(player_id, currPos, newPos):
	TweenNode.interpolate_property(self, "position", currPos, newPos, reset_dur, Tween.TRANS_LINEAR, Tween.EASE_OUT) #todo fix pull-back duration not actually working
	TweenNode.start()



##HELPER FUNCTION #####################################################
func _disable_collision(obj, disable):
	var wr = weakref(obj)
	if (wr.get_ref()):
		obj.get_node("CollisionShape2D").set_deferred("disabled", disable)
		
##SHOOTING STUFF #########################################################
func shoot():
	if not isLoaded():
		return 
	
	$Reload.start()
	_disable_collision(ShovelNode, false)
	
	HelperFunctions.rpc("reparent", ShovelNode.get_path(), "/root/World/Projectiles", true)
	ShovelNode.fire()


func isLoaded(): 
	return has_node("Shovel" + player_id)
	
func _on_Reload_timeout():
	$Reload.stop()
	if not isLoaded():
		rpc("reload")
	
remotesync func reload():
	var shovel = Shovel.instance()
	call_deferred("add_child", shovel)
	shovel.call_deferred("setup")
	ShovelNode = shovel
	_disable_collision(ShovelNode, true) #quickfix to allow melee combat after shooting

