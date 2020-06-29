extends Node

var current_message = "" 
var current_message_priority = 0  

func broadcast(message : String, duration : int, priority : int) -> void:  
	if priority >= current_message_priority: 
		current_message = message
		rpc("update_broadcast", message)
