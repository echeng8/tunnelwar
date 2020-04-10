extends Area2D

export(float) var SPEED = 1000
export(float) var DAMAGE = 20

var fire = false
var direction = 0
var velocity = 0

func start(_position, _direction):
	position = _position
	rotation = _direction.angle()
	velocity = _direction * SPEED

func _process(delta):
	if fire == true:
		position +=  velocity * delta
		
func _on_body_entered(body):
	#if body.is_a_parent_of(self):
	#	return
	if body.has_method('damage'):
		body.damage(DAMAGE)
	fire = false
	queue_free()

func _on_VisibilityNotifier2D_screen_exited():
	fire = false
	queue_free()
	
