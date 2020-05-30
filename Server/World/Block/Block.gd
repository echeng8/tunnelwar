extends StaticBody2D

remote func init_on_client(client_id): 
	rpc_id(client_id, "set_broken", is_broken()) 

func is_broken():
	return $StateMachine.state.name == "BBrokenState"
	
func get_struck_by(body):
	if $StateMachine.state.has_method("get_struck_by"):
		$StateMachine.state.get_struck_by(body)

