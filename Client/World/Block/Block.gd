extends Node2D

const textures = {
	"GoldOreBlock" : preload("res://World/Block/GoldBlock.png"),
	"Dirt" : preload("res://World/Block/Block.png")
}

puppet func destroy():
	queue_free() 
	
puppet func set_server_state(filename):
	$Sprite.texture = textures[filename]
	$Sprite.texture = textures[filename]
