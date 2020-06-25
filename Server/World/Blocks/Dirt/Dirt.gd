extends Block
class_name Dirt

func get_class(): 
	return "Dirt" 
	
	
func get_struck_by(body):
	destroy()

