extends CanvasLayer

signal text_sent(text)
#todo integrate these
signal chat_edit_started
signal chat_edit_ended
signal logged_out

onready var chat_box := $ChatBox

func _gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		chat_box.line_edit.release_focus()
	
func add_chat_reply(text: String, sender: String) -> void:
	chat_box.add_reply(text, sender)

func _on_ChatBox_edit_started() -> void:
	emit_signal("chat_edit_started")

func _on_ChatBox_edit_ended() -> void:
	emit_signal("chat_edit_ended")

func _on_ChatBox_text_sent(text):
	emit_signal("text_sent", text)
