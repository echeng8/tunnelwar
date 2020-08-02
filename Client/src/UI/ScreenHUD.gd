extends CanvasLayer

signal text_sent(text)
#todo integrate these
signal chat_edit_started
signal chat_edit_ended
signal logged_out


func _on_ChatBox_text_send(text):
	emit_signal("text_sent", text)
