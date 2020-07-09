extends KinematicBody2D

class_name Player

remotesync var username 
#STATS 
var _gold = 0 setget set_gold, get_gold 

#MECHANICS 
export var speed = 600
export var base_hp = 40
export var health_points = 40

#implementation vars 
var velocity = Vector2.ZERO 

#puppet vars
puppet var input_direction =  Vector2.ZERO
puppet var input_aim_pos = Vector2.ZERO
puppet var input_pull_jp = false 

signal struck_by(node) 
signal on_gold_change

func _ready():
	assert($StateMachine.connect("on_state_change", self, "update_client_state") == OK) 
#networking 

remote func initialize_rpc_sender() -> void:
	var s_id = get_tree().get_rpc_sender_id()
	rpc_id(s_id, "set_health", health_points)
	rset_id(s_id,"gold", get_gold())
	update_client_state() 
	
func update_client_state() -> void:
	rpc("update_client_state", $StateMachine.state.name)

remotesync func set_player_position(pos):
	position = pos 

#gameplay
func get_struck_by(source):
	emit_signal("struck_by", source)

remote func respawn() -> void:
	if $StateMachine.state.has_method("respawn"):
		$StateMachine.state.respawn() 

#die then delete 
func disconnect_die() -> void:
	$StateMachine.change_to("PDeadState") 
	broadcast_death() 
	rpc("destroy")

remotesync func destroy() -> void:
	queue_free() 
	
func broadcast_death(killer_id = -1):
	var msg = ""
	if not killer_id == -1:
		msg = "[p] has killed %s for %s gold." % [username, _gold]
	else: 
		msg = "%s suddenly dies, dropping %s gold." % [username, _gold]
	gamestate.broadcast_node.broadcast(msg, 3, 0, killer_id)
	
#SETTERS AND GETTERS
func set_gold(amount):
	_gold = amount 
	rset("gold", _gold)
	emit_signal("on_gold_change")
	
func get_gold():
	return _gold 
	
func add_gold(amount = 1):
	set_gold(get_gold() + amount) 
