extends Node2D

const textures = {
	"GoldBlock" : preload("res://World/Block/GoldBlock.png"),
	"Block" : preload("res://World/Block/Block.png")
}

var server_state = "" 

func _ready():
	rpc_id(1, "init_on_client", get_tree().get_network_unique_id())
	
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
