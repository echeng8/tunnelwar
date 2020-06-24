extends Node

signal on_message_send(message)  

var firstMessage = true

remote func add_message(message):
	#If it's the first message EVER, then DON'T new line it.
	#NOTE: this could be done better possibly, but this is how I solved this.
	if firstMessage:
		$ChatText.bbcode_text += str(message)
		firstMessage = false
	else:
		$ChatText.bbcode_text += "\n" + str(message)
	
	#How to color text:
	#[color=green]youText[/color]
	#chatTextLabelRef.append_bbcode("%s" % message)

func _input(ev):
	var just_pressed = ev.is_pressed() and not ev.is_echo()
	if Input.is_key_pressed(KEY_ENTER) and just_pressed:
		if $UserInput.has_focus():
			_on_SendButton_pressed()
		else:
			$UserInput.grab_focus() 


func _on_SendButton_pressed() -> void:
	$UserInput.release_focus()
	if  $UserInput.text == "":
		return

	rpc_id(1, "_chat_box_received_message", $UserInput.text)
	emit_signal("on_message_send", $UserInput.text)
	$UserInput.text = ""



