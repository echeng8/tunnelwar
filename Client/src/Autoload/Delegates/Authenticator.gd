# Delegate class that handles logging in and registering accounts. Holds the 
# authenticated session that ServerConnection uses to send messages or create a socket.
class_name Authenticator
extends Reference

var session: NakamaSession setget _no_set
var _client: NakamaClient setget _no_set
var _exception_handler: ExceptionHandler setget _no_set


func _init(client: NakamaClient, exception_handler: ExceptionHandler) -> void:
	_client = client
	_exception_handler = exception_handler


# Asynchronous coroutine. Authenticates a new session via email and password, and
# creates a new account when it did not previously exist, then initializes session.
# Returns OK or a nakama error code. Stores error messages in `ServerConnection.error_message`
func register_async() -> int:
	var id = uuid.v4() 
	var new_session: NakamaSession = yield(
		_client.authenticate_device_async(id), "completed"
	)

	var parsed_result := _exception_handler.parse_exception(new_session)
	if parsed_result == OK:
		session = new_session

	return parsed_result

func cleanup() -> void:
	session = null
	
func _no_set(_value) -> void:
	pass
