extends Area2D

export(float) var SPEED = 1000
export(float) var DAMAGE = 20
export var STAB_DAMAGE = 10
export(Vector2) var init_position = Vector2(335, -23)
export(Vector2) var firing_scale = Vector2(.5, .5)
var reflect = false

signal _pick_up

var fire = false
var direction = 0
var velocity = 0
var player_id

func _ready():
	position = init_position
	show_behind_parent = true

func setup():
	player_id = get_parent().player_id
	name = name + player_id
	add_to_group("shovels")
	print("Shovel:" + player_id)

func _process(delta):
	if fire == true:
		if reflect:
			velocity = -velocity
			reflect = false 
		global_position +=  velocity * delta
		rpc_unreliable("_update_shovel_movement", player_id, global_position)
		
			

func reflecting():
	reflect = true
	

func start(_position, _direction):
	scale = firing_scale
	global_position = _position
	global_rotation = _direction.angle()
	velocity = _direction * SPEED
	fire = true
		
func _on_body_entered(body):
	#print(get_parent().name)
#	if fire == false and get_parent().name != "ShovelGun" + body.name:
#		$Reload.stop()
#		emit_signal('_pick_up', body.name)
#		rpc('_pick_up', player_id, body.name)
#		queue_free()
	if get_parent().name == "ShovelGun" + player_id:
		if body.name != player_id:
			if body.has_method('damage'):
				body.damage(STAB_DAMAGE)
	else:
		if body.has_method('damage'):
			body.damage(DAMAGE)
			fire = false
			rpc("destory_shovel", player_id)
	#fire = false
	#queue_free()
#
#func _on_VisibilityNotifier2D_screen_exited():ww

remotesync func destory_shovel(player_id):
	queue_free()

func _on_Reload_timeout():
	$Reload.stop()
	rpc("_on_Reload_timeout", player_id)
	rpc("destory_shovel", player_id)


func _on_Projectile_area_shape_entered(area_id, area, area_shape, self_shape):
	print("ShovelArea")
	if is_in_group("shovels"):
		if get_parent().name == "ShovelGun" + player_id and area.get_parent().name != "ShovelGun" + area.player_id:
			print(area)
			print(area.name)
			if area.has_method('reflecting'):
				area.reflecting()
