extends Node2D

const textures = {
	"GoldOre" : preload("res://World/Block/GoldBlock.png"),
	"Dirt" : preload("res://World/Block/Block.png")
}

var server_state = "" 

remotesync func destroy():
	print('client dd')
	queue_free() 
	
puppet func set_server_state(state):
	server_state = state
	match state:
		"BDefaultState":
			visible = true 
			$Sprite.texture = textures["Block"]
		"BGoldState":
			visible = true 
			$Sprite.texture = textures["GoldBlock"]
		"BBrokenState": 
			visible = false 
