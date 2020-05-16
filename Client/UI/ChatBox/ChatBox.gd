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

func _input(ev):
	var just_pressed = ev.is_pressed() and not ev.is_echo()
	if Input.is_key_pressed(KEY_ENTER) and just_pressed:
		print('he')
		if $UserInput.has_focus():
			_on_SendButton_pressed()
		else:
			$UserInput.grab_focus() 
			
func _on_SendButton_pressed() -> void:
	$UserInput.release_focus()
	if  $UserInput.text == "":
		return
	var world = get_node("/root/World")
#	var uniqueNID = get_tree().get_network_unique_id()
	world.rpc_id(1, "_chat_box_received_message", $UserInput.text)

	$UserInput.text = ""



