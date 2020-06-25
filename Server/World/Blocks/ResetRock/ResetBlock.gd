extends Block
class_name ResetBlock

func get_class(): 
	return "ResetBlock" 
	
var hitpoints = 3 #how many strikes it takes to break

func get_struck_by(body):
	hitpoints -=  1 
	if hitpoints <= 0: 
		destroy()
