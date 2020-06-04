extends State

#MECHANICS

func get_struck_by(body):
	print(HelperFunctions.get_ancestor_pID(body))
	exit("BBrokenState")
