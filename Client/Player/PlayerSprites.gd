extends Node2D
const faces = {
	"normal" : preload("res://Player/Faces/normal_face.png"),
	"scared" : preload("res://Player/Faces/scared_face.png") 
}

func switch_face(face_name: String) -> void: 
	$face.texture = faces[face_name] 
