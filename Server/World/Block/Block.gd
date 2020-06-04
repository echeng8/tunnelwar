extends StaticBody2D

func _ready():
	$StateMachine.connect("on_state_change", self, "init_on_clients")
	
remote func init_on_client(client_id):  #todo , combine with init_on_clients
	rpc_id(client_id, "set_server_state", $StateMachine.state.name) 

remote func init_on_clients(): 
	print("init called")
	rpc("set_server_state", $StateMachine.state.name) 
	
func get_struck_by(body):
	if $StateMachine.state.has_method("get_struck_by"):
		$StateMachine.state.get_struck_by(body)
