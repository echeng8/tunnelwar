
unsafe impl GodotObject for Object {
    fn class_name() -> &'static str {
        "Object"
    }

    unsafe fn from_sys(obj: *mut sys::godot_object) -> Self {
        // Not reference-counted.
        Self { this: obj, }
    }

    unsafe fn from_return_position_sys(obj: *mut sys::godot_object) -> Self {
        Self { this: obj, }
    }

    unsafe fn to_sys(&self) -> *mut sys::godot_object {
        self.this
    }
}

impl ToVariant for Object {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Object {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Object {
    unsafe fn godot_free(self) { self.free() }
}

impl Instanciable for Object {
    fn construct() -> Self {
        Object::new()
    }
}

unsafe impl GodotObject for Reference {
    fn class_name() -> &'static str {
        "Reference"
    }

    unsafe fn from_sys(obj: *mut sys::godot_object) -> Self {
        object::add_ref(obj);
        Self { this: obj, }
    }

    unsafe fn from_return_position_sys(obj: *mut sys::godot_object) -> Self {
        Self { this: obj, }
    }

    unsafe fn to_sys(&self) -> *mut sys::godot_object {
        self.this
    }
}

impl ToVariant for Reference {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Reference {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Reference {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Reference {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Reference {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Reference {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Reference {
    fn construct() -> Self {
        Reference::new()
    }
}
