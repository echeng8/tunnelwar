
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct ObjectMethodTable {
    pub class_constructor: sys::godot_class_constructor,
    pub _get: *mut sys::godot_method_bind,
    pub _get_property_list: *mut sys::godot_method_bind,
    pub _init: *mut sys::godot_method_bind,
    pub _notification: *mut sys::godot_method_bind,
    pub _set: *mut sys::godot_method_bind,
    pub _to_string: *mut sys::godot_method_bind,
    pub add_user_signal: *mut sys::godot_method_bind,
    pub call: *mut sys::godot_method_bind,
    pub call_deferred: *mut sys::godot_method_bind,
    pub callv: *mut sys::godot_method_bind,
    pub can_translate_messages: *mut sys::godot_method_bind,
    pub connect: *mut sys::godot_method_bind,
    pub disconnect: *mut sys::godot_method_bind,
    pub emit_signal: *mut sys::godot_method_bind,
    pub get: *mut sys::godot_method_bind,
    pub get_class: *mut sys::godot_method_bind,
    pub get_incoming_connections: *mut sys::godot_method_bind,
    pub get_indexed: *mut sys::godot_method_bind,
    pub get_instance_id: *mut sys::godot_method_bind,
    pub get_meta: *mut sys::godot_method_bind,
    pub get_meta_list: *mut sys::godot_method_bind,
    pub get_method_list: *mut sys::godot_method_bind,
    pub get_property_list: *mut sys::godot_method_bind,
    pub get_script: *mut sys::godot_method_bind,
    pub get_signal_connection_list: *mut sys::godot_method_bind,
    pub get_signal_list: *mut sys::godot_method_bind,
    pub has_meta: *mut sys::godot_method_bind,
    pub has_method: *mut sys::godot_method_bind,
    pub has_user_signal: *mut sys::godot_method_bind,
    pub is_blocking_signals: *mut sys::godot_method_bind,
    pub is_class: *mut sys::godot_method_bind,
    pub is_connected: *mut sys::godot_method_bind,
    pub is_queued_for_deletion: *mut sys::godot_method_bind,
    pub notification: *mut sys::godot_method_bind,
    pub property_list_changed_notify: *mut sys::godot_method_bind,
    pub remove_meta: *mut sys::godot_method_bind,
    pub set: *mut sys::godot_method_bind,
    pub set_block_signals: *mut sys::godot_method_bind,
    pub set_deferred: *mut sys::godot_method_bind,
    pub set_indexed: *mut sys::godot_method_bind,
    pub set_message_translation: *mut sys::godot_method_bind,
    pub set_meta: *mut sys::godot_method_bind,
    pub set_script: *mut sys::godot_method_bind,
    pub to_string: *mut sys::godot_method_bind,
    pub tr: *mut sys::godot_method_bind,

}

impl ObjectMethodTable {
    unsafe fn get_mut() -> &'static mut Self {
        static mut TABLE: ObjectMethodTable = ObjectMethodTable {
            class_constructor: None,
            _get: 0 as *mut sys::godot_method_bind,
            _get_property_list: 0 as *mut sys::godot_method_bind,
            _init: 0 as *mut sys::godot_method_bind,
            _notification: 0 as *mut sys::godot_method_bind,
            _set: 0 as *mut sys::godot_method_bind,
            _to_string: 0 as *mut sys::godot_method_bind,
            add_user_signal: 0 as *mut sys::godot_method_bind,
            call: 0 as *mut sys::godot_method_bind,
            call_deferred: 0 as *mut sys::godot_method_bind,
            callv: 0 as *mut sys::godot_method_bind,
            can_translate_messages: 0 as *mut sys::godot_method_bind,
            connect: 0 as *mut sys::godot_method_bind,
            disconnect: 0 as *mut sys::godot_method_bind,
            emit_signal: 0 as *mut sys::godot_method_bind,
            get: 0 as *mut sys::godot_method_bind,
            get_class: 0 as *mut sys::godot_method_bind,
            get_incoming_connections: 0 as *mut sys::godot_method_bind,
            get_indexed: 0 as *mut sys::godot_method_bind,
            get_instance_id: 0 as *mut sys::godot_method_bind,
            get_meta: 0 as *mut sys::godot_method_bind,
            get_meta_list: 0 as *mut sys::godot_method_bind,
            get_method_list: 0 as *mut sys::godot_method_bind,
            get_property_list: 0 as *mut sys::godot_method_bind,
            get_script: 0 as *mut sys::godot_method_bind,
            get_signal_connection_list: 0 as *mut sys::godot_method_bind,
            get_signal_list: 0 as *mut sys::godot_method_bind,
            has_meta: 0 as *mut sys::godot_method_bind,
            has_method: 0 as *mut sys::godot_method_bind,
            has_user_signal: 0 as *mut sys::godot_method_bind,
            is_blocking_signals: 0 as *mut sys::godot_method_bind,
            is_class: 0 as *mut sys::godot_method_bind,
            is_connected: 0 as *mut sys::godot_method_bind,
            is_queued_for_deletion: 0 as *mut sys::godot_method_bind,
            notification: 0 as *mut sys::godot_method_bind,
            property_list_changed_notify: 0 as *mut sys::godot_method_bind,
            remove_meta: 0 as *mut sys::godot_method_bind,
            set: 0 as *mut sys::godot_method_bind,
            set_block_signals: 0 as *mut sys::godot_method_bind,
            set_deferred: 0 as *mut sys::godot_method_bind,
            set_indexed: 0 as *mut sys::godot_method_bind,
            set_message_translation: 0 as *mut sys::godot_method_bind,
            set_meta: 0 as *mut sys::godot_method_bind,
            set_script: 0 as *mut sys::godot_method_bind,
            to_string: 0 as *mut sys::godot_method_bind,
            tr: 0 as *mut sys::godot_method_bind,

        };

        &mut TABLE
    }

    pub unsafe fn unchecked_get() -> &'static Self {
        Self::get_mut()
    }

    pub fn get(gd_api: &GodotApi) -> &'static Self {
        unsafe {
            let table = Self::get_mut();
            static INIT: Once = Once::new();
            INIT.call_once(|| {
                ObjectMethodTable::init(table, gd_api);
            });

            table
        }
    }

    #[inline(never)]
    fn init(table: &mut Self, gd_api: &GodotApi) {
        unsafe {
            let class_name = b"Object\0".as_ptr() as *const c_char;
            table.class_constructor = (gd_api.godot_get_class_constructor)(class_name);
            table._get = (gd_api.godot_method_bind_get_method)(class_name, "_get\0".as_ptr() as *const c_char );
            table._get_property_list = (gd_api.godot_method_bind_get_method)(class_name, "_get_property_list\0".as_ptr() as *const c_char );
            table._init = (gd_api.godot_method_bind_get_method)(class_name, "_init\0".as_ptr() as *const c_char );
            table._notification = (gd_api.godot_method_bind_get_method)(class_name, "_notification\0".as_ptr() as *const c_char );
            table._set = (gd_api.godot_method_bind_get_method)(class_name, "_set\0".as_ptr() as *const c_char );
            table._to_string = (gd_api.godot_method_bind_get_method)(class_name, "_to_string\0".as_ptr() as *const c_char );
            table.add_user_signal = (gd_api.godot_method_bind_get_method)(class_name, "add_user_signal\0".as_ptr() as *const c_char );
            table.call = (gd_api.godot_method_bind_get_method)(class_name, "call\0".as_ptr() as *const c_char );
            table.call_deferred = (gd_api.godot_method_bind_get_method)(class_name, "call_deferred\0".as_ptr() as *const c_char );
            table.callv = (gd_api.godot_method_bind_get_method)(class_name, "callv\0".as_ptr() as *const c_char );
            table.can_translate_messages = (gd_api.godot_method_bind_get_method)(class_name, "can_translate_messages\0".as_ptr() as *const c_char );
            table.connect = (gd_api.godot_method_bind_get_method)(class_name, "connect\0".as_ptr() as *const c_char );
            table.disconnect = (gd_api.godot_method_bind_get_method)(class_name, "disconnect\0".as_ptr() as *const c_char );
            table.emit_signal = (gd_api.godot_method_bind_get_method)(class_name, "emit_signal\0".as_ptr() as *const c_char );
            table.get = (gd_api.godot_method_bind_get_method)(class_name, "get\0".as_ptr() as *const c_char );
            table.get_class = (gd_api.godot_method_bind_get_method)(class_name, "get_class\0".as_ptr() as *const c_char );
            table.get_incoming_connections = (gd_api.godot_method_bind_get_method)(class_name, "get_incoming_connections\0".as_ptr() as *const c_char );
            table.get_indexed = (gd_api.godot_method_bind_get_method)(class_name, "get_indexed\0".as_ptr() as *const c_char );
            table.get_instance_id = (gd_api.godot_method_bind_get_method)(class_name, "get_instance_id\0".as_ptr() as *const c_char );
            table.get_meta = (gd_api.godot_method_bind_get_method)(class_name, "get_meta\0".as_ptr() as *const c_char );
            table.get_meta_list = (gd_api.godot_method_bind_get_method)(class_name, "get_meta_list\0".as_ptr() as *const c_char );
            table.get_method_list = (gd_api.godot_method_bind_get_method)(class_name, "get_method_list\0".as_ptr() as *const c_char );
            table.get_property_list = (gd_api.godot_method_bind_get_method)(class_name, "get_property_list\0".as_ptr() as *const c_char );
            table.get_script = (gd_api.godot_method_bind_get_method)(class_name, "get_script\0".as_ptr() as *const c_char );
            table.get_signal_connection_list = (gd_api.godot_method_bind_get_method)(class_name, "get_signal_connection_list\0".as_ptr() as *const c_char );
            table.get_signal_list = (gd_api.godot_method_bind_get_method)(class_name, "get_signal_list\0".as_ptr() as *const c_char );
            table.has_meta = (gd_api.godot_method_bind_get_method)(class_name, "has_meta\0".as_ptr() as *const c_char );
            table.has_method = (gd_api.godot_method_bind_get_method)(class_name, "has_method\0".as_ptr() as *const c_char );
            table.has_user_signal = (gd_api.godot_method_bind_get_method)(class_name, "has_user_signal\0".as_ptr() as *const c_char );
            table.is_blocking_signals = (gd_api.godot_method_bind_get_method)(class_name, "is_blocking_signals\0".as_ptr() as *const c_char );
            table.is_class = (gd_api.godot_method_bind_get_method)(class_name, "is_class\0".as_ptr() as *const c_char );
            table.is_connected = (gd_api.godot_method_bind_get_method)(class_name, "is_connected\0".as_ptr() as *const c_char );
            table.is_queued_for_deletion = (gd_api.godot_method_bind_get_method)(class_name, "is_queued_for_deletion\0".as_ptr() as *const c_char );
            table.notification = (gd_api.godot_method_bind_get_method)(class_name, "notification\0".as_ptr() as *const c_char );
            table.property_list_changed_notify = (gd_api.godot_method_bind_get_method)(class_name, "property_list_changed_notify\0".as_ptr() as *const c_char );
            table.remove_meta = (gd_api.godot_method_bind_get_method)(class_name, "remove_meta\0".as_ptr() as *const c_char );
            table.set = (gd_api.godot_method_bind_get_method)(class_name, "set\0".as_ptr() as *const c_char );
            table.set_block_signals = (gd_api.godot_method_bind_get_method)(class_name, "set_block_signals\0".as_ptr() as *const c_char );
            table.set_deferred = (gd_api.godot_method_bind_get_method)(class_name, "set_deferred\0".as_ptr() as *const c_char );
            table.set_indexed = (gd_api.godot_method_bind_get_method)(class_name, "set_indexed\0".as_ptr() as *const c_char );
            table.set_message_translation = (gd_api.godot_method_bind_get_method)(class_name, "set_message_translation\0".as_ptr() as *const c_char );
            table.set_meta = (gd_api.godot_method_bind_get_method)(class_name, "set_meta\0".as_ptr() as *const c_char );
            table.set_script = (gd_api.godot_method_bind_get_method)(class_name, "set_script\0".as_ptr() as *const c_char );
            table.to_string = (gd_api.godot_method_bind_get_method)(class_name, "to_string\0".as_ptr() as *const c_char );
            table.tr = (gd_api.godot_method_bind_get_method)(class_name, "tr\0".as_ptr() as *const c_char );

        }
    }
}


#[doc(hidden)]
pub unsafe fn Object__get(obj_ptr: *mut sys::godot_object, property: GodotString) -> Variant {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api)._get;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        property.sys() as *const _ as *const _,

    ];

    let mut ret = sys::godot_variant::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(property);

    Variant::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object__get_property_list(obj_ptr: *mut sys::godot_object) -> VariantArray {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api)._get_property_list;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = sys::godot_array::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    VariantArray::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object__init(obj_ptr: *mut sys::godot_object) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api)._init;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
}


#[doc(hidden)]
pub unsafe fn Object__notification(obj_ptr: *mut sys::godot_object, what: i64) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api)._notification;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        (&what) as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(what);
}


#[doc(hidden)]
pub unsafe fn Object__set(obj_ptr: *mut sys::godot_object, property: GodotString, value: Variant) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api)._set;

    let mut argument_buffer : [*const libc::c_void; 2] = [
        property.sys() as *const _ as *const _,
        value.sys() as *const _ as *const _,

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(property);
    drop(value);

    ret
}


#[doc(hidden)]
pub unsafe fn Object__to_string(obj_ptr: *mut sys::godot_object) -> GodotString {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api)._to_string;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = sys::godot_string::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    GodotString::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_add_user_signal(obj_ptr: *mut sys::godot_object, signal: GodotString, arguments: VariantArray) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).add_user_signal;

    let mut argument_buffer : [*const libc::c_void; 2] = [
        signal.sys() as *const _ as *const _,
        arguments.sys() as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(signal);
    drop(arguments);
}


#[doc(hidden)]
pub unsafe fn Object_call(obj_ptr: *mut sys::godot_object, method: GodotString, varargs: &[Variant]) -> Variant {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).call;
    let mut argument_buffer: Vec<*const sys::godot_variant> = Vec::with_capacity(1 + varargs.len());
    let method: Variant = Variant::from_godot_string(&method);
    argument_buffer.push(method.sys()); 

    for arg in varargs {
        argument_buffer.push(arg.sys() as *const _);
    }
    let ret = Variant::from_sys((gd_api.godot_method_bind_call)(method_bind, obj_ptr, argument_buffer.as_mut_ptr(), argument_buffer.len() as _, ptr::null_mut()));
    drop(method);
    ret.into()
}


#[doc(hidden)]
pub unsafe fn Object_call_deferred(obj_ptr: *mut sys::godot_object, method: GodotString, varargs: &[Variant]) -> Variant {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).call_deferred;
    let mut argument_buffer: Vec<*const sys::godot_variant> = Vec::with_capacity(1 + varargs.len());
    let method: Variant = Variant::from_godot_string(&method);
    argument_buffer.push(method.sys()); 

    for arg in varargs {
        argument_buffer.push(arg.sys() as *const _);
    }
    let ret = Variant::from_sys((gd_api.godot_method_bind_call)(method_bind, obj_ptr, argument_buffer.as_mut_ptr(), argument_buffer.len() as _, ptr::null_mut()));
    drop(method);
    ret.into()
}


#[doc(hidden)]
pub unsafe fn Object_callv(obj_ptr: *mut sys::godot_object, method: GodotString, arg_array: VariantArray) -> Variant {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).callv;

    let mut argument_buffer : [*const libc::c_void; 2] = [
        method.sys() as *const _ as *const _,
        arg_array.sys() as *const _ as *const _,

    ];

    let mut ret = sys::godot_variant::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(method);
    drop(arg_array);

    Variant::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_can_translate_messages(obj_ptr: *mut sys::godot_object) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).can_translate_messages;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    ret
}


#[doc(hidden)]
pub unsafe fn Object_connect(obj_ptr: *mut sys::godot_object, signal: GodotString, target: Option<Object>, method: GodotString, binds: VariantArray, flags: i64) -> GodotResult {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).connect;

    let mut argument_buffer : [*const libc::c_void; 5] = [
        signal.sys() as *const _ as *const _,
        if let Some(arg) = &target { arg.this as *const _ as *const _ } else { ptr::null() },
        method.sys() as *const _ as *const _,
        binds.sys() as *const _ as *const _,
        (&flags) as *const _ as *const _,

    ];

    let mut ret: sys::godot_error = sys::godot_error_GODOT_OK;
    let ret_ptr = (&mut ret) as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(signal);
    drop(target);
    drop(method);
    drop(binds);
    drop(flags);

    result_from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_disconnect(obj_ptr: *mut sys::godot_object, signal: GodotString, target: Option<Object>, method: GodotString) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).disconnect;

    let mut argument_buffer : [*const libc::c_void; 3] = [
        signal.sys() as *const _ as *const _,
        if let Some(arg) = &target { arg.this as *const _ as *const _ } else { ptr::null() },
        method.sys() as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(signal);
    drop(target);
    drop(method);
}


#[doc(hidden)]
pub unsafe fn Object_emit_signal(obj_ptr: *mut sys::godot_object, signal: GodotString, varargs: &[Variant]) -> Variant {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).emit_signal;
    let mut argument_buffer: Vec<*const sys::godot_variant> = Vec::with_capacity(1 + varargs.len());
    let signal: Variant = Variant::from_godot_string(&signal);
    argument_buffer.push(signal.sys()); 

    for arg in varargs {
        argument_buffer.push(arg.sys() as *const _);
    }
    let ret = Variant::from_sys((gd_api.godot_method_bind_call)(method_bind, obj_ptr, argument_buffer.as_mut_ptr(), argument_buffer.len() as _, ptr::null_mut()));
    drop(signal);
    ret.into()
}


#[doc(hidden)]
pub unsafe fn Object_get(obj_ptr: *mut sys::godot_object, property: GodotString) -> Variant {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        property.sys() as *const _ as *const _,

    ];

    let mut ret = sys::godot_variant::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(property);

    Variant::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_get_class(obj_ptr: *mut sys::godot_object) -> GodotString {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_class;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = sys::godot_string::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    GodotString::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_get_incoming_connections(obj_ptr: *mut sys::godot_object) -> VariantArray {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_incoming_connections;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = sys::godot_array::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    VariantArray::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_get_indexed(obj_ptr: *mut sys::godot_object, property: NodePath) -> Variant {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_indexed;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        property.sys() as *const _ as *const _,

    ];

    let mut ret = sys::godot_variant::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(property);

    Variant::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_get_instance_id(obj_ptr: *mut sys::godot_object) -> i64 {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_instance_id;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = 0i64;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    ret
}


#[doc(hidden)]
pub unsafe fn Object_get_meta(obj_ptr: *mut sys::godot_object, name: GodotString) -> Variant {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_meta;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        name.sys() as *const _ as *const _,

    ];

    let mut ret = sys::godot_variant::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(name);

    Variant::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_get_meta_list(obj_ptr: *mut sys::godot_object) -> StringArray {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_meta_list;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = sys::godot_pool_string_array::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    StringArray::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_get_method_list(obj_ptr: *mut sys::godot_object) -> VariantArray {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_method_list;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = sys::godot_array::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    VariantArray::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_get_property_list(obj_ptr: *mut sys::godot_object) -> VariantArray {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_property_list;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = sys::godot_array::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    VariantArray::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_get_script(obj_ptr: *mut sys::godot_object) -> Option<Reference> {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_script;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret: *mut sys::godot_object = ptr::null_mut();
    let ret_ptr = (&mut ret) as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    if ret.is_null() {
        None
    } else {
        Some(Reference::from_return_position_sys(ret))
    }
}


#[doc(hidden)]
pub unsafe fn Object_get_signal_connection_list(obj_ptr: *mut sys::godot_object, signal: GodotString) -> VariantArray {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_signal_connection_list;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        signal.sys() as *const _ as *const _,

    ];

    let mut ret = sys::godot_array::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(signal);

    VariantArray::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_get_signal_list(obj_ptr: *mut sys::godot_object) -> VariantArray {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).get_signal_list;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = sys::godot_array::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    VariantArray::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_has_meta(obj_ptr: *mut sys::godot_object, name: GodotString) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).has_meta;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        name.sys() as *const _ as *const _,

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(name);

    ret
}


#[doc(hidden)]
pub unsafe fn Object_has_method(obj_ptr: *mut sys::godot_object, method: GodotString) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).has_method;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        method.sys() as *const _ as *const _,

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(method);

    ret
}


#[doc(hidden)]
pub unsafe fn Object_has_user_signal(obj_ptr: *mut sys::godot_object, signal: GodotString) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).has_user_signal;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        signal.sys() as *const _ as *const _,

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(signal);

    ret
}


#[doc(hidden)]
pub unsafe fn Object_is_blocking_signals(obj_ptr: *mut sys::godot_object) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).is_blocking_signals;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    ret
}


#[doc(hidden)]
pub unsafe fn Object_is_class(obj_ptr: *mut sys::godot_object, class: GodotString) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).is_class;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        class.sys() as *const _ as *const _,

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(class);

    ret
}


#[doc(hidden)]
pub unsafe fn Object_is_connected(obj_ptr: *mut sys::godot_object, signal: GodotString, target: Option<Object>, method: GodotString) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).is_connected;

    let mut argument_buffer : [*const libc::c_void; 3] = [
        signal.sys() as *const _ as *const _,
        if let Some(arg) = &target { arg.this as *const _ as *const _ } else { ptr::null() },
        method.sys() as *const _ as *const _,

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(signal);
    drop(target);
    drop(method);

    ret
}


#[doc(hidden)]
pub unsafe fn Object_is_queued_for_deletion(obj_ptr: *mut sys::godot_object) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).is_queued_for_deletion;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    ret
}


#[doc(hidden)]
pub unsafe fn Object_notification(obj_ptr: *mut sys::godot_object, what: i64, reversed: bool) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).notification;

    let mut argument_buffer : [*const libc::c_void; 2] = [
        (&what) as *const _ as *const _,
        (&reversed) as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(what);
    drop(reversed);
}


#[doc(hidden)]
pub unsafe fn Object_property_list_changed_notify(obj_ptr: *mut sys::godot_object) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).property_list_changed_notify;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
}


#[doc(hidden)]
pub unsafe fn Object_remove_meta(obj_ptr: *mut sys::godot_object, name: GodotString) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).remove_meta;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        name.sys() as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(name);
}


#[doc(hidden)]
pub unsafe fn Object_set(obj_ptr: *mut sys::godot_object, property: GodotString, value: Variant) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).set;

    let mut argument_buffer : [*const libc::c_void; 2] = [
        property.sys() as *const _ as *const _,
        value.sys() as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(property);
    drop(value);
}


#[doc(hidden)]
pub unsafe fn Object_set_block_signals(obj_ptr: *mut sys::godot_object, enable: bool) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).set_block_signals;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        (&enable) as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(enable);
}


#[doc(hidden)]
pub unsafe fn Object_set_deferred(obj_ptr: *mut sys::godot_object, property: GodotString, value: Variant) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).set_deferred;

    let mut argument_buffer : [*const libc::c_void; 2] = [
        property.sys() as *const _ as *const _,
        value.sys() as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(property);
    drop(value);
}


#[doc(hidden)]
pub unsafe fn Object_set_indexed(obj_ptr: *mut sys::godot_object, property: NodePath, value: Variant) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).set_indexed;

    let mut argument_buffer : [*const libc::c_void; 2] = [
        property.sys() as *const _ as *const _,
        value.sys() as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(property);
    drop(value);
}


#[doc(hidden)]
pub unsafe fn Object_set_message_translation(obj_ptr: *mut sys::godot_object, enable: bool) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).set_message_translation;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        (&enable) as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(enable);
}


#[doc(hidden)]
pub unsafe fn Object_set_meta(obj_ptr: *mut sys::godot_object, name: GodotString, value: Variant) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).set_meta;

    let mut argument_buffer : [*const libc::c_void; 2] = [
        name.sys() as *const _ as *const _,
        value.sys() as *const _ as *const _,

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(name);
    drop(value);
}


#[doc(hidden)]
pub unsafe fn Object_set_script(obj_ptr: *mut sys::godot_object, script: Option<Reference>) -> () {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).set_script;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        if let Some(arg) = &script { arg.this as *const _ as *const _ } else { ptr::null() },

    ];

    let ret_ptr = ptr::null_mut();

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(script);
}


#[doc(hidden)]
pub unsafe fn Object_to_string(obj_ptr: *mut sys::godot_object) -> GodotString {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).to_string;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = sys::godot_string::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    GodotString::from_sys(ret)
}


#[doc(hidden)]
pub unsafe fn Object_tr(obj_ptr: *mut sys::godot_object, message: GodotString) -> GodotString {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ObjectMethodTable::get(gd_api).tr;

    let mut argument_buffer : [*const libc::c_void; 1] = [
        message.sys() as *const _ as *const _,

    ];

    let mut ret = sys::godot_string::default();
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);
    drop(message);

    GodotString::from_sys(ret)
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct ReferenceMethodTable {
    pub class_constructor: sys::godot_class_constructor,
    pub init_ref: *mut sys::godot_method_bind,
    pub reference: *mut sys::godot_method_bind,
    pub unreference: *mut sys::godot_method_bind,

}

impl ReferenceMethodTable {
    unsafe fn get_mut() -> &'static mut Self {
        static mut TABLE: ReferenceMethodTable = ReferenceMethodTable {
            class_constructor: None,
            init_ref: 0 as *mut sys::godot_method_bind,
            reference: 0 as *mut sys::godot_method_bind,
            unreference: 0 as *mut sys::godot_method_bind,

        };

        &mut TABLE
    }

    pub unsafe fn unchecked_get() -> &'static Self {
        Self::get_mut()
    }

    pub fn get(gd_api: &GodotApi) -> &'static Self {
        unsafe {
            let table = Self::get_mut();
            static INIT: Once = Once::new();
            INIT.call_once(|| {
                ReferenceMethodTable::init(table, gd_api);
            });

            table
        }
    }

    #[inline(never)]
    fn init(table: &mut Self, gd_api: &GodotApi) {
        unsafe {
            let class_name = b"Reference\0".as_ptr() as *const c_char;
            table.class_constructor = (gd_api.godot_get_class_constructor)(class_name);
            table.init_ref = (gd_api.godot_method_bind_get_method)(class_name, "init_ref\0".as_ptr() as *const c_char );
            table.reference = (gd_api.godot_method_bind_get_method)(class_name, "reference\0".as_ptr() as *const c_char );
            table.unreference = (gd_api.godot_method_bind_get_method)(class_name, "unreference\0".as_ptr() as *const c_char );

        }
    }
}


#[doc(hidden)]
pub unsafe fn Reference_init_ref(obj_ptr: *mut sys::godot_object) -> bool {
    let gd_api = get_api();

    let method_bind: *mut sys::godot_method_bind = ReferenceMethodTable::get(gd_api).init_ref;

    let mut argument_buffer : [*const libc::c_void; 0] = [

    ];

    let mut ret = false;
    let ret_ptr = &mut ret as *mut _;

    (gd_api.godot_method_bind_ptrcall)(method_bind, obj_ptr, argument_buffer.as_mut_ptr() as *mut _, ret_ptr as *mut _);

    ret
}
