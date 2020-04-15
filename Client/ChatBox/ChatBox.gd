extends Node


var chatTextLabelRef

var firstMessage = true


func _ready():
	chatTextLabelRef = $"ChatText"


func _update_chat_message_box(message):
	#If it's the first message EVER, then DON'T new line it.
	#NOTE: this could be done better possibly, but this is how I solved this.
	if firstMessage:
		chatTextLabelRef.bbcode_text += str(message)
		firstMessage = false
	else:
		chatTextLabelRef.bbcode_text += "\n" + str(message)
	
	#How to color text:
	#[color=green]youText[/color]
	#chatTextLabelRef.append_bbcode("%s" % message)


func _on_SendButton_pressed() -> void:
	var world = get_node("/root/World")
	var uniqueNID = get_tree().get_network_unique_id()
	world.rpc_id(1, "_chat_box_received_message", $"UserInput".text)
	#reset text
	$UserInput.text = ""



