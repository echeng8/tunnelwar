extends Sprite

export var load_strength = 0
var stabbing = [false, 0]

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _process(delta):
	match load_strength:
		0:
			position.x = 29
		5:
			position.x = 26
		10:
			position.x = 23
		15:
			position.x = 20

func fire():
	stabbing = [true, load_strength]
	load_strength = 0


func _on_Area2D_area_entered(area):
	if stabbing[0] == true and area.name == "PlayerArea":
		area.hit(stabbing[1])
