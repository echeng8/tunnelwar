extends Block

var hitpoints = 5 #how many strikes it takes to break

func get_struck_by(body):
	hitpoints -=  1 
	if hitpoints <= 0: 
		destroy()
