/// The base class of most Godot classes.
///
/// ## Official documentation
///
/// See the [documentation of this class](https://godot.readthedocs.io/en/latest/classes/class_object.html) in the Godot engine's official documentation.
///
/// ## Memory management
///
/// Non reference counted objects such as the ones of this type are usually owned by the engine.
///
/// `Object` is an unsafe pointer, and all of its methods are unsafe.
///
/// In the cases where Rust code owns an object of this type, for example if the object was just
/// created on the Rust side and not passed to the engine yet, ownership should be either given
/// to the engine or the object must be manually destroyed using `Object::free`.
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Object {
    #[doc(hidden)]
    pub this: *mut sys::godot_object,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObjectConnectFlags {
    ConnectDeferred = 1,
    ConnectPersist = 2,
    ConnectOneshot = 4,
    ConnectReferenceCounted = 8,
}
/// Constants
#[allow(non_upper_case_globals)]
impl Object {
    pub const CONNECT_ONESHOT: i64 = 4;
    pub const NOTIFICATION_POSTINITIALIZE: i64 = 0;
    pub const CONNECT_REFERENCE_COUNTED: i64 = 8;
    pub const CONNECT_DEFERRED: i64 = 1;
    pub const CONNECT_PERSIST: i64 = 2;
    pub const NOTIFICATION_PREDELETE: i64 = 1;
}
impl Object {

    /// Constructor.
    ///
    /// Because this type is not reference counted, the lifetime of the returned object
    /// is *not* automatically managed.
    /// Immediately after creation, the object is owned by the caller, and can be
    /// passed to the engine (in which case the engine will be responsible for
    /// destroying the object) or destroyed manually using `Object::free`.
    pub fn new() -> Self {
        unsafe {
            let gd_api = get_api();
            let ctor = ObjectMethodTable::get(gd_api).class_constructor.unwrap();
            let this = ctor();

            Object {
                this
            }
        }
    }

    /// Manually deallocate the object.
    #[inline]
    pub unsafe fn free(self) {
        (get_api().godot_object_destroy)(self.this);
    }
    #[inline]
    pub unsafe fn _get(&mut self, property: GodotString) -> Variant {
        Object__get(self.this, property)
    }

    #[inline]
    pub unsafe fn _get_property_list(&mut self) -> VariantArray {
        Object__get_property_list(self.this)
    }

    #[inline]
    pub unsafe fn _init(&mut self) -> () {
        Object__init(self.this)
    }

    #[inline]
    pub unsafe fn _notification(&mut self, what: i64) -> () {
        Object__notification(self.this, what)
    }

    #[inline]
    pub unsafe fn _set(&mut self, property: GodotString, value: Variant) -> bool {
        Object__set(self.this, property, value)
    }

    #[inline]
    pub unsafe fn _to_string(&mut self) -> GodotString {
        Object__to_string(self.this)
    }

    #[inline]
    pub unsafe fn add_user_signal(&mut self, signal: GodotString, arguments: VariantArray) -> () {
        Object_add_user_signal(self.this, signal, arguments)
    }

    #[inline]
    pub unsafe fn call(&mut self, method: GodotString, varargs: &[Variant]) -> Variant {
        Object_call(self.this, method, varargs)
    }

    #[inline]
    pub unsafe fn call_deferred(&mut self, method: GodotString, varargs: &[Variant]) -> Variant {
        Object_call_deferred(self.this, method, varargs)
    }

    #[inline]
    pub unsafe fn callv(&mut self, method: GodotString, arg_array: VariantArray) -> Variant {
        Object_callv(self.this, method, arg_array)
    }

    #[inline]
    pub unsafe fn can_translate_messages(&self) -> bool {
        Object_can_translate_messages(self.this)
    }

    #[inline]
    pub unsafe fn connect(&mut self, signal: GodotString, target: Option<Object>, method: GodotString, binds: VariantArray, flags: i64) -> GodotResult {
        Object_connect(self.this, signal, target, method, binds, flags)
    }

    #[inline]
    pub unsafe fn disconnect(&mut self, signal: GodotString, target: Option<Object>, method: GodotString) -> () {
        Object_disconnect(self.this, signal, target, method)
    }

    #[inline]
    pub unsafe fn emit_signal(&mut self, signal: GodotString, varargs: &[Variant]) -> Variant {
        Object_emit_signal(self.this, signal, varargs)
    }

    #[inline]
    pub unsafe fn get(&self, property: GodotString) -> Variant {
        Object_get(self.this, property)
    }

    #[inline]
    pub unsafe fn get_class(&self) -> GodotString {
        Object_get_class(self.this)
    }

    #[inline]
    pub unsafe fn get_incoming_connections(&self) -> VariantArray {
        Object_get_incoming_connections(self.this)
    }

    #[inline]
    pub unsafe fn get_indexed(&self, property: NodePath) -> Variant {
        Object_get_indexed(self.this, property)
    }

    #[inline]
    pub unsafe fn get_instance_id(&self) -> i64 {
        Object_get_instance_id(self.this)
    }

    #[inline]
    pub unsafe fn get_meta(&self, name: GodotString) -> Variant {
        Object_get_meta(self.this, name)
    }

    #[inline]
    pub unsafe fn get_meta_list(&self) -> StringArray {
        Object_get_meta_list(self.this)
    }

    #[inline]
    pub unsafe fn get_method_list(&self) -> VariantArray {
        Object_get_method_list(self.this)
    }

    #[inline]
    pub unsafe fn get_property_list(&self) -> VariantArray {
        Object_get_property_list(self.this)
    }

    #[inline]
    pub unsafe fn get_script(&self) -> Option<Reference> {
        Object_get_script(self.this)
    }

    #[inline]
    pub unsafe fn get_signal_connection_list(&self, signal: GodotString) -> VariantArray {
        Object_get_signal_connection_list(self.this, signal)
    }

    #[inline]
    pub unsafe fn get_signal_list(&self) -> VariantArray {
        Object_get_signal_list(self.this)
    }

    #[inline]
    pub unsafe fn has_meta(&self, name: GodotString) -> bool {
        Object_has_meta(self.this, name)
    }

    #[inline]
    pub unsafe fn has_method(&self, method: GodotString) -> bool {
        Object_has_method(self.this, method)
    }

    #[inline]
    pub unsafe fn has_user_signal(&self, signal: GodotString) -> bool {
        Object_has_user_signal(self.this, signal)
    }

    #[inline]
    pub unsafe fn is_blocking_signals(&self) -> bool {
        Object_is_blocking_signals(self.this)
    }

    #[inline]
    pub unsafe fn is_class(&self, class: GodotString) -> bool {
        Object_is_class(self.this, class)
    }

    #[inline]
    pub unsafe fn is_connected(&self, signal: GodotString, target: Option<Object>, method: GodotString) -> bool {
        Object_is_connected(self.this, signal, target, method)
    }

    #[inline]
    pub unsafe fn is_queued_for_deletion(&self) -> bool {
        Object_is_queued_for_deletion(self.this)
    }

    #[inline]
    pub unsafe fn notification(&mut self, what: i64, reversed: bool) -> () {
        Object_notification(self.this, what, reversed)
    }

    #[inline]
    pub unsafe fn property_list_changed_notify(&mut self) -> () {
        Object_property_list_changed_notify(self.this)
    }

    #[inline]
    pub unsafe fn remove_meta(&mut self, name: GodotString) -> () {
        Object_remove_meta(self.this, name)
    }

    #[inline]
    pub unsafe fn set(&mut self, property: GodotString, value: Variant) -> () {
        Object_set(self.this, property, value)
    }

    #[inline]
    pub unsafe fn set_block_signals(&mut self, enable: bool) -> () {
        Object_set_block_signals(self.this, enable)
    }

    #[inline]
    pub unsafe fn set_deferred(&mut self, property: GodotString, value: Variant) -> () {
        Object_set_deferred(self.this, property, value)
    }

    #[inline]
    pub unsafe fn set_indexed(&mut self, property: NodePath, value: Variant) -> () {
        Object_set_indexed(self.this, property, value)
    }

    #[inline]
    pub unsafe fn set_message_translation(&mut self, enable: bool) -> () {
        Object_set_message_translation(self.this, enable)
    }

    #[inline]
    pub unsafe fn set_meta(&mut self, name: GodotString, value: Variant) -> () {
        Object_set_meta(self.this, name, value)
    }

    #[inline]
    pub unsafe fn set_script(&mut self, script: Option<Reference>) -> () {
        Object_set_script(self.this, script)
    }

    #[inline]
    pub unsafe fn to_string(&mut self) -> GodotString {
        Object_to_string(self.this)
    }

    #[inline]
    pub unsafe fn tr(&self, message: GodotString) -> GodotString {
        Object_tr(self.this, message)
    }


    /// Generic dynamic cast.
    pub unsafe fn cast<T: GodotObject>(&self) -> Option<T> {
        object::godot_cast::<T>(self.this)
    }
}
/// Base class of all reference-counted types. Inherits `Object`.
///
/// ## Official documentation
///
/// See the [documentation of this class](https://godot.readthedocs.io/en/latest/classes/class_reference.html) in the Godot engine's official documentation.
///
/// ## Memory management
///
/// The lifetime of this object is automatically managed through reference counting.
///
/// ## Class hierarchy
///
/// Reference inherits methods from:
/// - [Object](struct.Object.html)
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Reference {
    #[doc(hidden)]
    pub this: *mut sys::godot_object,
}

impl Reference {

    // Constructor
    pub fn new() -> Self {
        unsafe {
            let gd_api = get_api();
            let ctor = ReferenceMethodTable::get(gd_api).class_constructor.unwrap();
            let obj = ctor();
            object::init_ref_count(obj);

            Reference {
                this: obj
            }
        }
    }


    /// Creates a new reference to the same reference-counted object.
    pub fn new_ref(&self) -> Self {
        unsafe {
            object::add_ref(self.this);

            Self {
                this: self.this,
            }
        }
    }

    #[inline]
    pub fn init_ref(&mut self) -> bool {
        unsafe { Reference_init_ref(self.this) }
    }

    /// Up-cast.
    #[inline]
    pub fn to_object(&self) -> Object {
        // Not reference-counted.
        Object { this: self.this }
    }

    /// Generic dynamic cast.
    pub fn cast<T: GodotObject>(&self) -> Option<T> {
        object::godot_cast::<T>(self.this)
    }
}
