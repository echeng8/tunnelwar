
unsafe impl GodotObject for GlobalConstants {
    fn class_name() -> &'static str {
        "GlobalConstants"
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

impl ToVariant for GlobalConstants {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GlobalConstants {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

unsafe impl GodotObject for ARVRAnchor {
    fn class_name() -> &'static str {
        "ARVRAnchor"
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

impl ToVariant for ARVRAnchor {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ARVRAnchor {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ARVRAnchor {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ARVRAnchor {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ARVRAnchor {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ARVRAnchor {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ARVRAnchor {
    fn construct() -> Self {
        ARVRAnchor::new()
    }
}

unsafe impl GodotObject for ARVRCamera {
    fn class_name() -> &'static str {
        "ARVRCamera"
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

impl ToVariant for ARVRCamera {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ARVRCamera {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ARVRCamera {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ARVRCamera {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ARVRCamera {
    type Target = Camera;

    fn deref(&self) -> &Camera {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ARVRCamera {
    fn deref_mut(&mut self) -> &mut Camera {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ARVRCamera {
    fn construct() -> Self {
        ARVRCamera::new()
    }
}

unsafe impl GodotObject for ARVRController {
    fn class_name() -> &'static str {
        "ARVRController"
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

impl ToVariant for ARVRController {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ARVRController {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ARVRController {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ARVRController {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ARVRController {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ARVRController {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ARVRController {
    fn construct() -> Self {
        ARVRController::new()
    }
}

unsafe impl GodotObject for ARVRInterface {
    fn class_name() -> &'static str {
        "ARVRInterface"
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

impl ToVariant for ARVRInterface {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ARVRInterface {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ARVRInterface {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ARVRInterface {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ARVRInterface {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ARVRInterface {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for ARVRInterfaceGDNative {
    fn class_name() -> &'static str {
        "ARVRInterfaceGDNative"
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

impl ToVariant for ARVRInterfaceGDNative {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ARVRInterfaceGDNative {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ARVRInterfaceGDNative {
    type Target = ARVRInterface;

    fn deref(&self) -> &ARVRInterface {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ARVRInterfaceGDNative {
    fn deref_mut(&mut self) -> &mut ARVRInterface {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ARVRInterfaceGDNative {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ARVRInterfaceGDNative {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ARVRInterfaceGDNative {
    fn construct() -> Self {
        ARVRInterfaceGDNative::new()
    }
}

unsafe impl GodotObject for ARVROrigin {
    fn class_name() -> &'static str {
        "ARVROrigin"
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

impl ToVariant for ARVROrigin {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ARVROrigin {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ARVROrigin {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ARVROrigin {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ARVROrigin {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ARVROrigin {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ARVROrigin {
    fn construct() -> Self {
        ARVROrigin::new()
    }
}

unsafe impl GodotObject for ARVRPositionalTracker {
    fn class_name() -> &'static str {
        "ARVRPositionalTracker"
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

impl ToVariant for ARVRPositionalTracker {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ARVRPositionalTracker {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ARVRPositionalTracker {
    unsafe fn godot_free(self) { self.free() }
}

impl std::ops::Deref for ARVRPositionalTracker {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ARVRPositionalTracker {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ARVRPositionalTracker {
    fn construct() -> Self {
        ARVRPositionalTracker::new()
    }
}

unsafe impl GodotObject for ARVRServer {
    fn class_name() -> &'static str {
        "ARVRServer"
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

impl ToVariant for ARVRServer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ARVRServer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ARVRServer {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ARVRServer {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for AStar {
    fn class_name() -> &'static str {
        "AStar"
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

impl ToVariant for AStar {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AStar {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AStar {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AStar {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AStar {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AStar {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AStar {
    fn construct() -> Self {
        AStar::new()
    }
}

unsafe impl GodotObject for AStar2D {
    fn class_name() -> &'static str {
        "AStar2D"
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

impl ToVariant for AStar2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AStar2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AStar2D {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AStar2D {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AStar2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AStar2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AStar2D {
    fn construct() -> Self {
        AStar2D::new()
    }
}

unsafe impl GodotObject for AcceptDialog {
    fn class_name() -> &'static str {
        "AcceptDialog"
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

impl ToVariant for AcceptDialog {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AcceptDialog {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for AcceptDialog {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for AcceptDialog {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for AcceptDialog {
    type Target = WindowDialog;

    fn deref(&self) -> &WindowDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AcceptDialog {
    fn deref_mut(&mut self) -> &mut WindowDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for AcceptDialog {
    fn construct() -> Self {
        AcceptDialog::new()
    }
}

unsafe impl GodotObject for AnimatedSprite {
    fn class_name() -> &'static str {
        "AnimatedSprite"
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

impl ToVariant for AnimatedSprite {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimatedSprite {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for AnimatedSprite {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for AnimatedSprite {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for AnimatedSprite {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimatedSprite {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for AnimatedSprite {
    fn construct() -> Self {
        AnimatedSprite::new()
    }
}

unsafe impl GodotObject for AnimatedSprite3D {
    fn class_name() -> &'static str {
        "AnimatedSprite3D"
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

impl ToVariant for AnimatedSprite3D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimatedSprite3D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for AnimatedSprite3D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for AnimatedSprite3D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for AnimatedSprite3D {
    type Target = SpriteBase3D;

    fn deref(&self) -> &SpriteBase3D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimatedSprite3D {
    fn deref_mut(&mut self) -> &mut SpriteBase3D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for AnimatedSprite3D {
    fn construct() -> Self {
        AnimatedSprite3D::new()
    }
}

unsafe impl GodotObject for AnimatedTexture {
    fn class_name() -> &'static str {
        "AnimatedTexture"
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

impl ToVariant for AnimatedTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimatedTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimatedTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimatedTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimatedTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimatedTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimatedTexture {
    fn construct() -> Self {
        AnimatedTexture::new()
    }
}

unsafe impl GodotObject for Animation {
    fn class_name() -> &'static str {
        "Animation"
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

impl ToVariant for Animation {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Animation {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Animation {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Animation {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Animation {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Animation {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Animation {
    fn construct() -> Self {
        Animation::new()
    }
}

unsafe impl GodotObject for AnimationNode {
    fn class_name() -> &'static str {
        "AnimationNode"
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

impl ToVariant for AnimationNode {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNode {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNode {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNode {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNode {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNode {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNode {
    fn construct() -> Self {
        AnimationNode::new()
    }
}

unsafe impl GodotObject for AnimationNodeAdd2 {
    fn class_name() -> &'static str {
        "AnimationNodeAdd2"
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

impl ToVariant for AnimationNodeAdd2 {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeAdd2 {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeAdd2 {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeAdd2 {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeAdd2 {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeAdd2 {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeAdd2 {
    fn construct() -> Self {
        AnimationNodeAdd2::new()
    }
}

unsafe impl GodotObject for AnimationNodeAdd3 {
    fn class_name() -> &'static str {
        "AnimationNodeAdd3"
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

impl ToVariant for AnimationNodeAdd3 {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeAdd3 {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeAdd3 {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeAdd3 {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeAdd3 {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeAdd3 {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeAdd3 {
    fn construct() -> Self {
        AnimationNodeAdd3::new()
    }
}

unsafe impl GodotObject for AnimationNodeAnimation {
    fn class_name() -> &'static str {
        "AnimationNodeAnimation"
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

impl ToVariant for AnimationNodeAnimation {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeAnimation {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeAnimation {
    type Target = AnimationRootNode;

    fn deref(&self) -> &AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeAnimation {
    fn deref_mut(&mut self) -> &mut AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeAnimation {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeAnimation {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeAnimation {
    fn construct() -> Self {
        AnimationNodeAnimation::new()
    }
}

unsafe impl GodotObject for AnimationNodeBlend2 {
    fn class_name() -> &'static str {
        "AnimationNodeBlend2"
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

impl ToVariant for AnimationNodeBlend2 {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeBlend2 {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeBlend2 {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeBlend2 {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeBlend2 {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeBlend2 {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeBlend2 {
    fn construct() -> Self {
        AnimationNodeBlend2::new()
    }
}

unsafe impl GodotObject for AnimationNodeBlend3 {
    fn class_name() -> &'static str {
        "AnimationNodeBlend3"
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

impl ToVariant for AnimationNodeBlend3 {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeBlend3 {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeBlend3 {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeBlend3 {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeBlend3 {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeBlend3 {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeBlend3 {
    fn construct() -> Self {
        AnimationNodeBlend3::new()
    }
}

unsafe impl GodotObject for AnimationNodeBlendSpace1D {
    fn class_name() -> &'static str {
        "AnimationNodeBlendSpace1D"
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

impl ToVariant for AnimationNodeBlendSpace1D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeBlendSpace1D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeBlendSpace1D {
    type Target = AnimationRootNode;

    fn deref(&self) -> &AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeBlendSpace1D {
    fn deref_mut(&mut self) -> &mut AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeBlendSpace1D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeBlendSpace1D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeBlendSpace1D {
    fn construct() -> Self {
        AnimationNodeBlendSpace1D::new()
    }
}

unsafe impl GodotObject for AnimationNodeBlendSpace2D {
    fn class_name() -> &'static str {
        "AnimationNodeBlendSpace2D"
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

impl ToVariant for AnimationNodeBlendSpace2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeBlendSpace2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeBlendSpace2D {
    type Target = AnimationRootNode;

    fn deref(&self) -> &AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeBlendSpace2D {
    fn deref_mut(&mut self) -> &mut AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeBlendSpace2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeBlendSpace2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeBlendSpace2D {
    fn construct() -> Self {
        AnimationNodeBlendSpace2D::new()
    }
}

unsafe impl GodotObject for AnimationNodeBlendTree {
    fn class_name() -> &'static str {
        "AnimationNodeBlendTree"
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

impl ToVariant for AnimationNodeBlendTree {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeBlendTree {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeBlendTree {
    type Target = AnimationRootNode;

    fn deref(&self) -> &AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeBlendTree {
    fn deref_mut(&mut self) -> &mut AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeBlendTree {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeBlendTree {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeBlendTree {
    fn construct() -> Self {
        AnimationNodeBlendTree::new()
    }
}

unsafe impl GodotObject for AnimationNodeOneShot {
    fn class_name() -> &'static str {
        "AnimationNodeOneShot"
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

impl ToVariant for AnimationNodeOneShot {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeOneShot {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeOneShot {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeOneShot {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeOneShot {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeOneShot {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeOneShot {
    fn construct() -> Self {
        AnimationNodeOneShot::new()
    }
}

unsafe impl GodotObject for AnimationNodeOutput {
    fn class_name() -> &'static str {
        "AnimationNodeOutput"
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

impl ToVariant for AnimationNodeOutput {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeOutput {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeOutput {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeOutput {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeOutput {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeOutput {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeOutput {
    fn construct() -> Self {
        AnimationNodeOutput::new()
    }
}

unsafe impl GodotObject for AnimationNodeStateMachine {
    fn class_name() -> &'static str {
        "AnimationNodeStateMachine"
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

impl ToVariant for AnimationNodeStateMachine {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeStateMachine {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeStateMachine {
    type Target = AnimationRootNode;

    fn deref(&self) -> &AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeStateMachine {
    fn deref_mut(&mut self) -> &mut AnimationRootNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeStateMachine {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeStateMachine {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeStateMachine {
    fn construct() -> Self {
        AnimationNodeStateMachine::new()
    }
}

unsafe impl GodotObject for AnimationNodeStateMachinePlayback {
    fn class_name() -> &'static str {
        "AnimationNodeStateMachinePlayback"
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

impl ToVariant for AnimationNodeStateMachinePlayback {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeStateMachinePlayback {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeStateMachinePlayback {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeStateMachinePlayback {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeStateMachinePlayback {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeStateMachinePlayback {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeStateMachinePlayback {
    fn construct() -> Self {
        AnimationNodeStateMachinePlayback::new()
    }
}

unsafe impl GodotObject for AnimationNodeStateMachineTransition {
    fn class_name() -> &'static str {
        "AnimationNodeStateMachineTransition"
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

impl ToVariant for AnimationNodeStateMachineTransition {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeStateMachineTransition {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeStateMachineTransition {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeStateMachineTransition {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeStateMachineTransition {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeStateMachineTransition {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeStateMachineTransition {
    fn construct() -> Self {
        AnimationNodeStateMachineTransition::new()
    }
}

unsafe impl GodotObject for AnimationNodeTimeScale {
    fn class_name() -> &'static str {
        "AnimationNodeTimeScale"
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

impl ToVariant for AnimationNodeTimeScale {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeTimeScale {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeTimeScale {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeTimeScale {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeTimeScale {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeTimeScale {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeTimeScale {
    fn construct() -> Self {
        AnimationNodeTimeScale::new()
    }
}

unsafe impl GodotObject for AnimationNodeTimeSeek {
    fn class_name() -> &'static str {
        "AnimationNodeTimeSeek"
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

impl ToVariant for AnimationNodeTimeSeek {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeTimeSeek {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeTimeSeek {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeTimeSeek {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeTimeSeek {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeTimeSeek {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeTimeSeek {
    fn construct() -> Self {
        AnimationNodeTimeSeek::new()
    }
}

unsafe impl GodotObject for AnimationNodeTransition {
    fn class_name() -> &'static str {
        "AnimationNodeTransition"
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

impl ToVariant for AnimationNodeTransition {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationNodeTransition {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationNodeTransition {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationNodeTransition {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationNodeTransition {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationNodeTransition {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationNodeTransition {
    fn construct() -> Self {
        AnimationNodeTransition::new()
    }
}

unsafe impl GodotObject for AnimationPlayer {
    fn class_name() -> &'static str {
        "AnimationPlayer"
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

impl ToVariant for AnimationPlayer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationPlayer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for AnimationPlayer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for AnimationPlayer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for AnimationPlayer {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationPlayer {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for AnimationPlayer {
    fn construct() -> Self {
        AnimationPlayer::new()
    }
}

unsafe impl GodotObject for AnimationRootNode {
    fn class_name() -> &'static str {
        "AnimationRootNode"
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

impl ToVariant for AnimationRootNode {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationRootNode {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationRootNode {
    type Target = AnimationNode;

    fn deref(&self) -> &AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationRootNode {
    fn deref_mut(&mut self) -> &mut AnimationNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationRootNode {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationRootNode {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AnimationRootNode {
    fn construct() -> Self {
        AnimationRootNode::new()
    }
}

unsafe impl GodotObject for AnimationTrackEditPlugin {
    fn class_name() -> &'static str {
        "AnimationTrackEditPlugin"
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

impl ToVariant for AnimationTrackEditPlugin {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationTrackEditPlugin {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AnimationTrackEditPlugin {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationTrackEditPlugin {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AnimationTrackEditPlugin {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AnimationTrackEditPlugin {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for AnimationTree {
    fn class_name() -> &'static str {
        "AnimationTree"
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

impl ToVariant for AnimationTree {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationTree {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for AnimationTree {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for AnimationTree {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for AnimationTree {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationTree {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for AnimationTree {
    fn construct() -> Self {
        AnimationTree::new()
    }
}

unsafe impl GodotObject for AnimationTreePlayer {
    fn class_name() -> &'static str {
        "AnimationTreePlayer"
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

impl ToVariant for AnimationTreePlayer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AnimationTreePlayer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for AnimationTreePlayer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for AnimationTreePlayer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for AnimationTreePlayer {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AnimationTreePlayer {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for AnimationTreePlayer {
    fn construct() -> Self {
        AnimationTreePlayer::new()
    }
}

unsafe impl GodotObject for Area {
    fn class_name() -> &'static str {
        "Area"
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

impl ToVariant for Area {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Area {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Area {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Area {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Area {
    type Target = CollisionObject;

    fn deref(&self) -> &CollisionObject {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Area {
    fn deref_mut(&mut self) -> &mut CollisionObject {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Area {
    fn construct() -> Self {
        Area::new()
    }
}

unsafe impl GodotObject for Area2D {
    fn class_name() -> &'static str {
        "Area2D"
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

impl ToVariant for Area2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Area2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Area2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Area2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Area2D {
    type Target = CollisionObject2D;

    fn deref(&self) -> &CollisionObject2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Area2D {
    fn deref_mut(&mut self) -> &mut CollisionObject2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Area2D {
    fn construct() -> Self {
        Area2D::new()
    }
}

unsafe impl GodotObject for ArrayMesh {
    fn class_name() -> &'static str {
        "ArrayMesh"
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

impl ToVariant for ArrayMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ArrayMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ArrayMesh {
    type Target = Mesh;

    fn deref(&self) -> &Mesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ArrayMesh {
    fn deref_mut(&mut self) -> &mut Mesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ArrayMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ArrayMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ArrayMesh {
    fn construct() -> Self {
        ArrayMesh::new()
    }
}

unsafe impl GodotObject for AtlasTexture {
    fn class_name() -> &'static str {
        "AtlasTexture"
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

impl ToVariant for AtlasTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AtlasTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AtlasTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AtlasTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AtlasTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AtlasTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AtlasTexture {
    fn construct() -> Self {
        AtlasTexture::new()
    }
}

unsafe impl GodotObject for AudioBusLayout {
    fn class_name() -> &'static str {
        "AudioBusLayout"
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

impl ToVariant for AudioBusLayout {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioBusLayout {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioBusLayout {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioBusLayout {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioBusLayout {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioBusLayout {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioBusLayout {
    fn construct() -> Self {
        AudioBusLayout::new()
    }
}

unsafe impl GodotObject for AudioEffect {
    fn class_name() -> &'static str {
        "AudioEffect"
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

impl ToVariant for AudioEffect {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffect {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffect {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffect {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffect {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffect {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for AudioEffectAmplify {
    fn class_name() -> &'static str {
        "AudioEffectAmplify"
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

impl ToVariant for AudioEffectAmplify {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectAmplify {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectAmplify {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectAmplify {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectAmplify {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectAmplify {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectAmplify {
    fn construct() -> Self {
        AudioEffectAmplify::new()
    }
}

unsafe impl GodotObject for AudioEffectBandLimitFilter {
    fn class_name() -> &'static str {
        "AudioEffectBandLimitFilter"
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

impl ToVariant for AudioEffectBandLimitFilter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectBandLimitFilter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectBandLimitFilter {
    type Target = AudioEffectFilter;

    fn deref(&self) -> &AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectBandLimitFilter {
    fn deref_mut(&mut self) -> &mut AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectBandLimitFilter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectBandLimitFilter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectBandLimitFilter {
    fn construct() -> Self {
        AudioEffectBandLimitFilter::new()
    }
}

unsafe impl GodotObject for AudioEffectBandPassFilter {
    fn class_name() -> &'static str {
        "AudioEffectBandPassFilter"
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

impl ToVariant for AudioEffectBandPassFilter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectBandPassFilter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectBandPassFilter {
    type Target = AudioEffectFilter;

    fn deref(&self) -> &AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectBandPassFilter {
    fn deref_mut(&mut self) -> &mut AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectBandPassFilter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectBandPassFilter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectBandPassFilter {
    fn construct() -> Self {
        AudioEffectBandPassFilter::new()
    }
}

unsafe impl GodotObject for AudioEffectChorus {
    fn class_name() -> &'static str {
        "AudioEffectChorus"
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

impl ToVariant for AudioEffectChorus {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectChorus {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectChorus {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectChorus {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectChorus {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectChorus {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectChorus {
    fn construct() -> Self {
        AudioEffectChorus::new()
    }
}

unsafe impl GodotObject for AudioEffectCompressor {
    fn class_name() -> &'static str {
        "AudioEffectCompressor"
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

impl ToVariant for AudioEffectCompressor {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectCompressor {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectCompressor {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectCompressor {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectCompressor {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectCompressor {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectCompressor {
    fn construct() -> Self {
        AudioEffectCompressor::new()
    }
}

unsafe impl GodotObject for AudioEffectDelay {
    fn class_name() -> &'static str {
        "AudioEffectDelay"
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

impl ToVariant for AudioEffectDelay {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectDelay {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectDelay {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectDelay {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectDelay {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectDelay {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectDelay {
    fn construct() -> Self {
        AudioEffectDelay::new()
    }
}

unsafe impl GodotObject for AudioEffectDistortion {
    fn class_name() -> &'static str {
        "AudioEffectDistortion"
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

impl ToVariant for AudioEffectDistortion {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectDistortion {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectDistortion {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectDistortion {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectDistortion {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectDistortion {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectDistortion {
    fn construct() -> Self {
        AudioEffectDistortion::new()
    }
}

unsafe impl GodotObject for AudioEffectEQ {
    fn class_name() -> &'static str {
        "AudioEffectEQ"
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

impl ToVariant for AudioEffectEQ {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectEQ {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectEQ {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectEQ {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectEQ {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectEQ {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectEQ {
    fn construct() -> Self {
        AudioEffectEQ::new()
    }
}

unsafe impl GodotObject for AudioEffectEQ10 {
    fn class_name() -> &'static str {
        "AudioEffectEQ10"
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

impl ToVariant for AudioEffectEQ10 {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectEQ10 {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectEQ10 {
    type Target = AudioEffectEQ;

    fn deref(&self) -> &AudioEffectEQ {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectEQ10 {
    fn deref_mut(&mut self) -> &mut AudioEffectEQ {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectEQ10 {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectEQ10 {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectEQ10 {
    fn construct() -> Self {
        AudioEffectEQ10::new()
    }
}

unsafe impl GodotObject for AudioEffectEQ21 {
    fn class_name() -> &'static str {
        "AudioEffectEQ21"
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

impl ToVariant for AudioEffectEQ21 {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectEQ21 {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectEQ21 {
    type Target = AudioEffectEQ;

    fn deref(&self) -> &AudioEffectEQ {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectEQ21 {
    fn deref_mut(&mut self) -> &mut AudioEffectEQ {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectEQ21 {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectEQ21 {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectEQ21 {
    fn construct() -> Self {
        AudioEffectEQ21::new()
    }
}

unsafe impl GodotObject for AudioEffectEQ6 {
    fn class_name() -> &'static str {
        "AudioEffectEQ6"
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

impl ToVariant for AudioEffectEQ6 {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectEQ6 {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectEQ6 {
    type Target = AudioEffectEQ;

    fn deref(&self) -> &AudioEffectEQ {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectEQ6 {
    fn deref_mut(&mut self) -> &mut AudioEffectEQ {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectEQ6 {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectEQ6 {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectEQ6 {
    fn construct() -> Self {
        AudioEffectEQ6::new()
    }
}

unsafe impl GodotObject for AudioEffectFilter {
    fn class_name() -> &'static str {
        "AudioEffectFilter"
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

impl ToVariant for AudioEffectFilter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectFilter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectFilter {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectFilter {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectFilter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectFilter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectFilter {
    fn construct() -> Self {
        AudioEffectFilter::new()
    }
}

unsafe impl GodotObject for AudioEffectHighPassFilter {
    fn class_name() -> &'static str {
        "AudioEffectHighPassFilter"
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

impl ToVariant for AudioEffectHighPassFilter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectHighPassFilter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectHighPassFilter {
    type Target = AudioEffectFilter;

    fn deref(&self) -> &AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectHighPassFilter {
    fn deref_mut(&mut self) -> &mut AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectHighPassFilter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectHighPassFilter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectHighPassFilter {
    fn construct() -> Self {
        AudioEffectHighPassFilter::new()
    }
}

unsafe impl GodotObject for AudioEffectHighShelfFilter {
    fn class_name() -> &'static str {
        "AudioEffectHighShelfFilter"
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

impl ToVariant for AudioEffectHighShelfFilter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectHighShelfFilter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectHighShelfFilter {
    type Target = AudioEffectFilter;

    fn deref(&self) -> &AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectHighShelfFilter {
    fn deref_mut(&mut self) -> &mut AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectHighShelfFilter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectHighShelfFilter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectHighShelfFilter {
    fn construct() -> Self {
        AudioEffectHighShelfFilter::new()
    }
}

unsafe impl GodotObject for AudioEffectInstance {
    fn class_name() -> &'static str {
        "AudioEffectInstance"
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

impl ToVariant for AudioEffectInstance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectInstance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectInstance {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectInstance {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectInstance {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectInstance {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for AudioEffectLimiter {
    fn class_name() -> &'static str {
        "AudioEffectLimiter"
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

impl ToVariant for AudioEffectLimiter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectLimiter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectLimiter {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectLimiter {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectLimiter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectLimiter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectLimiter {
    fn construct() -> Self {
        AudioEffectLimiter::new()
    }
}

unsafe impl GodotObject for AudioEffectLowPassFilter {
    fn class_name() -> &'static str {
        "AudioEffectLowPassFilter"
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

impl ToVariant for AudioEffectLowPassFilter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectLowPassFilter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectLowPassFilter {
    type Target = AudioEffectFilter;

    fn deref(&self) -> &AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectLowPassFilter {
    fn deref_mut(&mut self) -> &mut AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectLowPassFilter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectLowPassFilter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectLowPassFilter {
    fn construct() -> Self {
        AudioEffectLowPassFilter::new()
    }
}

unsafe impl GodotObject for AudioEffectLowShelfFilter {
    fn class_name() -> &'static str {
        "AudioEffectLowShelfFilter"
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

impl ToVariant for AudioEffectLowShelfFilter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectLowShelfFilter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectLowShelfFilter {
    type Target = AudioEffectFilter;

    fn deref(&self) -> &AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectLowShelfFilter {
    fn deref_mut(&mut self) -> &mut AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectLowShelfFilter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectLowShelfFilter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectLowShelfFilter {
    fn construct() -> Self {
        AudioEffectLowShelfFilter::new()
    }
}

unsafe impl GodotObject for AudioEffectNotchFilter {
    fn class_name() -> &'static str {
        "AudioEffectNotchFilter"
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

impl ToVariant for AudioEffectNotchFilter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectNotchFilter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectNotchFilter {
    type Target = AudioEffectFilter;

    fn deref(&self) -> &AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectNotchFilter {
    fn deref_mut(&mut self) -> &mut AudioEffectFilter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectNotchFilter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectNotchFilter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectNotchFilter {
    fn construct() -> Self {
        AudioEffectNotchFilter::new()
    }
}

unsafe impl GodotObject for AudioEffectPanner {
    fn class_name() -> &'static str {
        "AudioEffectPanner"
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

impl ToVariant for AudioEffectPanner {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectPanner {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectPanner {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectPanner {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectPanner {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectPanner {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectPanner {
    fn construct() -> Self {
        AudioEffectPanner::new()
    }
}

unsafe impl GodotObject for AudioEffectPhaser {
    fn class_name() -> &'static str {
        "AudioEffectPhaser"
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

impl ToVariant for AudioEffectPhaser {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectPhaser {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectPhaser {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectPhaser {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectPhaser {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectPhaser {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectPhaser {
    fn construct() -> Self {
        AudioEffectPhaser::new()
    }
}

unsafe impl GodotObject for AudioEffectPitchShift {
    fn class_name() -> &'static str {
        "AudioEffectPitchShift"
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

impl ToVariant for AudioEffectPitchShift {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectPitchShift {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectPitchShift {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectPitchShift {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectPitchShift {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectPitchShift {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectPitchShift {
    fn construct() -> Self {
        AudioEffectPitchShift::new()
    }
}

unsafe impl GodotObject for AudioEffectRecord {
    fn class_name() -> &'static str {
        "AudioEffectRecord"
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

impl ToVariant for AudioEffectRecord {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectRecord {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectRecord {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectRecord {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectRecord {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectRecord {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectRecord {
    fn construct() -> Self {
        AudioEffectRecord::new()
    }
}

unsafe impl GodotObject for AudioEffectReverb {
    fn class_name() -> &'static str {
        "AudioEffectReverb"
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

impl ToVariant for AudioEffectReverb {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectReverb {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectReverb {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectReverb {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectReverb {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectReverb {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectReverb {
    fn construct() -> Self {
        AudioEffectReverb::new()
    }
}

unsafe impl GodotObject for AudioEffectSpectrumAnalyzer {
    fn class_name() -> &'static str {
        "AudioEffectSpectrumAnalyzer"
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

impl ToVariant for AudioEffectSpectrumAnalyzer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectSpectrumAnalyzer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectSpectrumAnalyzer {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectSpectrumAnalyzer {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectSpectrumAnalyzer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectSpectrumAnalyzer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectSpectrumAnalyzer {
    fn construct() -> Self {
        AudioEffectSpectrumAnalyzer::new()
    }
}

unsafe impl GodotObject for AudioEffectSpectrumAnalyzerInstance {
    fn class_name() -> &'static str {
        "AudioEffectSpectrumAnalyzerInstance"
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

impl ToVariant for AudioEffectSpectrumAnalyzerInstance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectSpectrumAnalyzerInstance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectSpectrumAnalyzerInstance {
    type Target = AudioEffectInstance;

    fn deref(&self) -> &AudioEffectInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectSpectrumAnalyzerInstance {
    fn deref_mut(&mut self) -> &mut AudioEffectInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectSpectrumAnalyzerInstance {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectSpectrumAnalyzerInstance {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for AudioEffectStereoEnhance {
    fn class_name() -> &'static str {
        "AudioEffectStereoEnhance"
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

impl ToVariant for AudioEffectStereoEnhance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioEffectStereoEnhance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioEffectStereoEnhance {
    type Target = AudioEffect;

    fn deref(&self) -> &AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioEffectStereoEnhance {
    fn deref_mut(&mut self) -> &mut AudioEffect {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioEffectStereoEnhance {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioEffectStereoEnhance {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioEffectStereoEnhance {
    fn construct() -> Self {
        AudioEffectStereoEnhance::new()
    }
}

unsafe impl GodotObject for AudioServer {
    fn class_name() -> &'static str {
        "AudioServer"
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

impl ToVariant for AudioServer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioServer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioServer {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioServer {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for AudioStream {
    fn class_name() -> &'static str {
        "AudioStream"
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

impl ToVariant for AudioStream {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStream {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioStream {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStream {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioStream {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioStream {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for AudioStreamGenerator {
    fn class_name() -> &'static str {
        "AudioStreamGenerator"
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

impl ToVariant for AudioStreamGenerator {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamGenerator {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioStreamGenerator {
    type Target = AudioStream;

    fn deref(&self) -> &AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamGenerator {
    fn deref_mut(&mut self) -> &mut AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioStreamGenerator {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioStreamGenerator {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioStreamGenerator {
    fn construct() -> Self {
        AudioStreamGenerator::new()
    }
}

unsafe impl GodotObject for AudioStreamGeneratorPlayback {
    fn class_name() -> &'static str {
        "AudioStreamGeneratorPlayback"
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

impl ToVariant for AudioStreamGeneratorPlayback {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamGeneratorPlayback {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioStreamGeneratorPlayback {
    type Target = AudioStreamPlaybackResampled;

    fn deref(&self) -> &AudioStreamPlaybackResampled {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamGeneratorPlayback {
    fn deref_mut(&mut self) -> &mut AudioStreamPlaybackResampled {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioStreamGeneratorPlayback {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioStreamGeneratorPlayback {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for AudioStreamMicrophone {
    fn class_name() -> &'static str {
        "AudioStreamMicrophone"
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

impl ToVariant for AudioStreamMicrophone {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamMicrophone {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioStreamMicrophone {
    type Target = AudioStream;

    fn deref(&self) -> &AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamMicrophone {
    fn deref_mut(&mut self) -> &mut AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioStreamMicrophone {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioStreamMicrophone {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioStreamMicrophone {
    fn construct() -> Self {
        AudioStreamMicrophone::new()
    }
}

unsafe impl GodotObject for AudioStreamOGGVorbis {
    fn class_name() -> &'static str {
        "AudioStreamOGGVorbis"
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

impl ToVariant for AudioStreamOGGVorbis {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamOGGVorbis {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioStreamOGGVorbis {
    type Target = AudioStream;

    fn deref(&self) -> &AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamOGGVorbis {
    fn deref_mut(&mut self) -> &mut AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioStreamOGGVorbis {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioStreamOGGVorbis {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioStreamOGGVorbis {
    fn construct() -> Self {
        AudioStreamOGGVorbis::new()
    }
}

unsafe impl GodotObject for AudioStreamPlayback {
    fn class_name() -> &'static str {
        "AudioStreamPlayback"
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

impl ToVariant for AudioStreamPlayback {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamPlayback {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioStreamPlayback {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamPlayback {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioStreamPlayback {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioStreamPlayback {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for AudioStreamPlaybackResampled {
    fn class_name() -> &'static str {
        "AudioStreamPlaybackResampled"
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

impl ToVariant for AudioStreamPlaybackResampled {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamPlaybackResampled {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioStreamPlaybackResampled {
    type Target = AudioStreamPlayback;

    fn deref(&self) -> &AudioStreamPlayback {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamPlaybackResampled {
    fn deref_mut(&mut self) -> &mut AudioStreamPlayback {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioStreamPlaybackResampled {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioStreamPlaybackResampled {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for AudioStreamPlayer {
    fn class_name() -> &'static str {
        "AudioStreamPlayer"
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

impl ToVariant for AudioStreamPlayer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamPlayer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for AudioStreamPlayer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for AudioStreamPlayer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for AudioStreamPlayer {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamPlayer {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for AudioStreamPlayer {
    fn construct() -> Self {
        AudioStreamPlayer::new()
    }
}

unsafe impl GodotObject for AudioStreamPlayer2D {
    fn class_name() -> &'static str {
        "AudioStreamPlayer2D"
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

impl ToVariant for AudioStreamPlayer2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamPlayer2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for AudioStreamPlayer2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for AudioStreamPlayer2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for AudioStreamPlayer2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamPlayer2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for AudioStreamPlayer2D {
    fn construct() -> Self {
        AudioStreamPlayer2D::new()
    }
}

unsafe impl GodotObject for AudioStreamPlayer3D {
    fn class_name() -> &'static str {
        "AudioStreamPlayer3D"
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

impl ToVariant for AudioStreamPlayer3D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamPlayer3D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for AudioStreamPlayer3D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for AudioStreamPlayer3D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for AudioStreamPlayer3D {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamPlayer3D {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for AudioStreamPlayer3D {
    fn construct() -> Self {
        AudioStreamPlayer3D::new()
    }
}

unsafe impl GodotObject for AudioStreamRandomPitch {
    fn class_name() -> &'static str {
        "AudioStreamRandomPitch"
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

impl ToVariant for AudioStreamRandomPitch {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamRandomPitch {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioStreamRandomPitch {
    type Target = AudioStream;

    fn deref(&self) -> &AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamRandomPitch {
    fn deref_mut(&mut self) -> &mut AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioStreamRandomPitch {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioStreamRandomPitch {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioStreamRandomPitch {
    fn construct() -> Self {
        AudioStreamRandomPitch::new()
    }
}

unsafe impl GodotObject for AudioStreamSample {
    fn class_name() -> &'static str {
        "AudioStreamSample"
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

impl ToVariant for AudioStreamSample {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for AudioStreamSample {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for AudioStreamSample {
    type Target = AudioStream;

    fn deref(&self) -> &AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for AudioStreamSample {
    fn deref_mut(&mut self) -> &mut AudioStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for AudioStreamSample {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for AudioStreamSample {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for AudioStreamSample {
    fn construct() -> Self {
        AudioStreamSample::new()
    }
}

unsafe impl GodotObject for BackBufferCopy {
    fn class_name() -> &'static str {
        "BackBufferCopy"
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

impl ToVariant for BackBufferCopy {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BackBufferCopy {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for BackBufferCopy {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for BackBufferCopy {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for BackBufferCopy {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BackBufferCopy {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for BackBufferCopy {
    fn construct() -> Self {
        BackBufferCopy::new()
    }
}

unsafe impl GodotObject for BakedLightmap {
    fn class_name() -> &'static str {
        "BakedLightmap"
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

impl ToVariant for BakedLightmap {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BakedLightmap {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for BakedLightmap {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for BakedLightmap {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for BakedLightmap {
    type Target = VisualInstance;

    fn deref(&self) -> &VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BakedLightmap {
    fn deref_mut(&mut self) -> &mut VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for BakedLightmap {
    fn construct() -> Self {
        BakedLightmap::new()
    }
}

unsafe impl GodotObject for BakedLightmapData {
    fn class_name() -> &'static str {
        "BakedLightmapData"
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

impl ToVariant for BakedLightmapData {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BakedLightmapData {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for BakedLightmapData {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BakedLightmapData {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for BakedLightmapData {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for BakedLightmapData {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for BakedLightmapData {
    fn construct() -> Self {
        BakedLightmapData::new()
    }
}

unsafe impl GodotObject for BaseButton {
    fn class_name() -> &'static str {
        "BaseButton"
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

impl ToVariant for BaseButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BaseButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for BaseButton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for BaseButton {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BaseButton {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for BitMap {
    fn class_name() -> &'static str {
        "BitMap"
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

impl ToVariant for BitMap {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BitMap {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for BitMap {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BitMap {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for BitMap {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for BitMap {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for BitMap {
    fn construct() -> Self {
        BitMap::new()
    }
}

unsafe impl GodotObject for BitmapFont {
    fn class_name() -> &'static str {
        "BitmapFont"
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

impl ToVariant for BitmapFont {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BitmapFont {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for BitmapFont {
    type Target = Font;

    fn deref(&self) -> &Font {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BitmapFont {
    fn deref_mut(&mut self) -> &mut Font {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for BitmapFont {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for BitmapFont {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for BitmapFont {
    fn construct() -> Self {
        BitmapFont::new()
    }
}

unsafe impl GodotObject for Bone2D {
    fn class_name() -> &'static str {
        "Bone2D"
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

impl ToVariant for Bone2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Bone2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Bone2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Bone2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Bone2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Bone2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Bone2D {
    fn construct() -> Self {
        Bone2D::new()
    }
}

unsafe impl GodotObject for BoneAttachment {
    fn class_name() -> &'static str {
        "BoneAttachment"
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

impl ToVariant for BoneAttachment {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BoneAttachment {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for BoneAttachment {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for BoneAttachment {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for BoneAttachment {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BoneAttachment {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for BoneAttachment {
    fn construct() -> Self {
        BoneAttachment::new()
    }
}

unsafe impl GodotObject for BoxContainer {
    fn class_name() -> &'static str {
        "BoxContainer"
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

impl ToVariant for BoxContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BoxContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for BoxContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for BoxContainer {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BoxContainer {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for BoxShape {
    fn class_name() -> &'static str {
        "BoxShape"
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

impl ToVariant for BoxShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BoxShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for BoxShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BoxShape {
    fn deref_mut(&mut self) -> &mut Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for BoxShape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for BoxShape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for BoxShape {
    fn construct() -> Self {
        BoxShape::new()
    }
}

unsafe impl GodotObject for BulletPhysicsDirectBodyState {
    fn class_name() -> &'static str {
        "BulletPhysicsDirectBodyState"
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

impl ToVariant for BulletPhysicsDirectBodyState {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BulletPhysicsDirectBodyState {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for BulletPhysicsDirectBodyState {
    type Target = PhysicsDirectBodyState;

    fn deref(&self) -> &PhysicsDirectBodyState {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BulletPhysicsDirectBodyState {
    fn deref_mut(&mut self) -> &mut PhysicsDirectBodyState {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for BulletPhysicsServer {
    fn class_name() -> &'static str {
        "BulletPhysicsServer"
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

impl ToVariant for BulletPhysicsServer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for BulletPhysicsServer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for BulletPhysicsServer {
    type Target = PhysicsServer;

    fn deref(&self) -> &PhysicsServer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for BulletPhysicsServer {
    fn deref_mut(&mut self) -> &mut PhysicsServer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Button {
    fn class_name() -> &'static str {
        "Button"
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

impl ToVariant for Button {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Button {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Button {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Button {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Button {
    type Target = BaseButton;

    fn deref(&self) -> &BaseButton {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Button {
    fn deref_mut(&mut self) -> &mut BaseButton {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Button {
    fn construct() -> Self {
        Button::new()
    }
}

unsafe impl GodotObject for ButtonGroup {
    fn class_name() -> &'static str {
        "ButtonGroup"
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

impl ToVariant for ButtonGroup {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ButtonGroup {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ButtonGroup {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ButtonGroup {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ButtonGroup {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ButtonGroup {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ButtonGroup {
    fn construct() -> Self {
        ButtonGroup::new()
    }
}

unsafe impl GodotObject for CPUParticles {
    fn class_name() -> &'static str {
        "CPUParticles"
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

impl ToVariant for CPUParticles {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CPUParticles {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CPUParticles {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CPUParticles {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CPUParticles {
    type Target = GeometryInstance;

    fn deref(&self) -> &GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CPUParticles {
    fn deref_mut(&mut self) -> &mut GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CPUParticles {
    fn construct() -> Self {
        CPUParticles::new()
    }
}

unsafe impl GodotObject for CPUParticles2D {
    fn class_name() -> &'static str {
        "CPUParticles2D"
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

impl ToVariant for CPUParticles2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CPUParticles2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CPUParticles2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CPUParticles2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CPUParticles2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CPUParticles2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CPUParticles2D {
    fn construct() -> Self {
        CPUParticles2D::new()
    }
}

unsafe impl GodotObject for CSGBox {
    fn class_name() -> &'static str {
        "CSGBox"
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

impl ToVariant for CSGBox {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CSGBox {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CSGBox {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CSGBox {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CSGBox {
    type Target = CSGPrimitive;

    fn deref(&self) -> &CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CSGBox {
    fn deref_mut(&mut self) -> &mut CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CSGBox {
    fn construct() -> Self {
        CSGBox::new()
    }
}

unsafe impl GodotObject for CSGCombiner {
    fn class_name() -> &'static str {
        "CSGCombiner"
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

impl ToVariant for CSGCombiner {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CSGCombiner {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CSGCombiner {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CSGCombiner {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CSGCombiner {
    type Target = CSGShape;

    fn deref(&self) -> &CSGShape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CSGCombiner {
    fn deref_mut(&mut self) -> &mut CSGShape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CSGCombiner {
    fn construct() -> Self {
        CSGCombiner::new()
    }
}

unsafe impl GodotObject for CSGCylinder {
    fn class_name() -> &'static str {
        "CSGCylinder"
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

impl ToVariant for CSGCylinder {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CSGCylinder {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CSGCylinder {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CSGCylinder {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CSGCylinder {
    type Target = CSGPrimitive;

    fn deref(&self) -> &CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CSGCylinder {
    fn deref_mut(&mut self) -> &mut CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CSGCylinder {
    fn construct() -> Self {
        CSGCylinder::new()
    }
}

unsafe impl GodotObject for CSGMesh {
    fn class_name() -> &'static str {
        "CSGMesh"
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

impl ToVariant for CSGMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CSGMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CSGMesh {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CSGMesh {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CSGMesh {
    type Target = CSGPrimitive;

    fn deref(&self) -> &CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CSGMesh {
    fn deref_mut(&mut self) -> &mut CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CSGMesh {
    fn construct() -> Self {
        CSGMesh::new()
    }
}

unsafe impl GodotObject for CSGPolygon {
    fn class_name() -> &'static str {
        "CSGPolygon"
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

impl ToVariant for CSGPolygon {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CSGPolygon {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CSGPolygon {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CSGPolygon {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CSGPolygon {
    type Target = CSGPrimitive;

    fn deref(&self) -> &CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CSGPolygon {
    fn deref_mut(&mut self) -> &mut CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CSGPolygon {
    fn construct() -> Self {
        CSGPolygon::new()
    }
}

unsafe impl GodotObject for CSGPrimitive {
    fn class_name() -> &'static str {
        "CSGPrimitive"
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

impl ToVariant for CSGPrimitive {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CSGPrimitive {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for CSGPrimitive {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CSGPrimitive {
    type Target = CSGShape;

    fn deref(&self) -> &CSGShape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CSGPrimitive {
    fn deref_mut(&mut self) -> &mut CSGShape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for CSGShape {
    fn class_name() -> &'static str {
        "CSGShape"
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

impl ToVariant for CSGShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CSGShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for CSGShape {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CSGShape {
    type Target = GeometryInstance;

    fn deref(&self) -> &GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CSGShape {
    fn deref_mut(&mut self) -> &mut GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for CSGSphere {
    fn class_name() -> &'static str {
        "CSGSphere"
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

impl ToVariant for CSGSphere {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CSGSphere {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CSGSphere {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CSGSphere {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CSGSphere {
    type Target = CSGPrimitive;

    fn deref(&self) -> &CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CSGSphere {
    fn deref_mut(&mut self) -> &mut CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CSGSphere {
    fn construct() -> Self {
        CSGSphere::new()
    }
}

unsafe impl GodotObject for CSGTorus {
    fn class_name() -> &'static str {
        "CSGTorus"
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

impl ToVariant for CSGTorus {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CSGTorus {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CSGTorus {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CSGTorus {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CSGTorus {
    type Target = CSGPrimitive;

    fn deref(&self) -> &CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CSGTorus {
    fn deref_mut(&mut self) -> &mut CSGPrimitive {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CSGTorus {
    fn construct() -> Self {
        CSGTorus::new()
    }
}

unsafe impl GodotObject for Camera {
    fn class_name() -> &'static str {
        "Camera"
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

impl ToVariant for Camera {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Camera {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Camera {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Camera {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Camera {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Camera {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Camera {
    fn construct() -> Self {
        Camera::new()
    }
}

unsafe impl GodotObject for Camera2D {
    fn class_name() -> &'static str {
        "Camera2D"
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

impl ToVariant for Camera2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Camera2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Camera2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Camera2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Camera2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Camera2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Camera2D {
    fn construct() -> Self {
        Camera2D::new()
    }
}

unsafe impl GodotObject for CameraFeed {
    fn class_name() -> &'static str {
        "CameraFeed"
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

impl ToVariant for CameraFeed {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CameraFeed {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CameraFeed {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CameraFeed {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CameraFeed {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CameraFeed {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CameraFeed {
    fn construct() -> Self {
        CameraFeed::new()
    }
}

unsafe impl GodotObject for CameraServer {
    fn class_name() -> &'static str {
        "CameraServer"
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

impl ToVariant for CameraServer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CameraServer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CameraServer {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CameraServer {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for CameraTexture {
    fn class_name() -> &'static str {
        "CameraTexture"
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

impl ToVariant for CameraTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CameraTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CameraTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CameraTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CameraTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CameraTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CameraTexture {
    fn construct() -> Self {
        CameraTexture::new()
    }
}

unsafe impl GodotObject for CanvasItem {
    fn class_name() -> &'static str {
        "CanvasItem"
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

impl ToVariant for CanvasItem {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CanvasItem {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for CanvasItem {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CanvasItem {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CanvasItem {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for CanvasItemMaterial {
    fn class_name() -> &'static str {
        "CanvasItemMaterial"
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

impl ToVariant for CanvasItemMaterial {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CanvasItemMaterial {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CanvasItemMaterial {
    type Target = Material;

    fn deref(&self) -> &Material {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CanvasItemMaterial {
    fn deref_mut(&mut self) -> &mut Material {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CanvasItemMaterial {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CanvasItemMaterial {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CanvasItemMaterial {
    fn construct() -> Self {
        CanvasItemMaterial::new()
    }
}

unsafe impl GodotObject for CanvasLayer {
    fn class_name() -> &'static str {
        "CanvasLayer"
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

impl ToVariant for CanvasLayer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CanvasLayer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CanvasLayer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CanvasLayer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CanvasLayer {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CanvasLayer {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CanvasLayer {
    fn construct() -> Self {
        CanvasLayer::new()
    }
}

unsafe impl GodotObject for CanvasModulate {
    fn class_name() -> &'static str {
        "CanvasModulate"
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

impl ToVariant for CanvasModulate {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CanvasModulate {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CanvasModulate {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CanvasModulate {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CanvasModulate {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CanvasModulate {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CanvasModulate {
    fn construct() -> Self {
        CanvasModulate::new()
    }
}

unsafe impl GodotObject for CapsuleMesh {
    fn class_name() -> &'static str {
        "CapsuleMesh"
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

impl ToVariant for CapsuleMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CapsuleMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CapsuleMesh {
    type Target = PrimitiveMesh;

    fn deref(&self) -> &PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CapsuleMesh {
    fn deref_mut(&mut self) -> &mut PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CapsuleMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CapsuleMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CapsuleMesh {
    fn construct() -> Self {
        CapsuleMesh::new()
    }
}

unsafe impl GodotObject for CapsuleShape {
    fn class_name() -> &'static str {
        "CapsuleShape"
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

impl ToVariant for CapsuleShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CapsuleShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CapsuleShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CapsuleShape {
    fn deref_mut(&mut self) -> &mut Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CapsuleShape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CapsuleShape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CapsuleShape {
    fn construct() -> Self {
        CapsuleShape::new()
    }
}

unsafe impl GodotObject for CapsuleShape2D {
    fn class_name() -> &'static str {
        "CapsuleShape2D"
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

impl ToVariant for CapsuleShape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CapsuleShape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CapsuleShape2D {
    type Target = Shape2D;

    fn deref(&self) -> &Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CapsuleShape2D {
    fn deref_mut(&mut self) -> &mut Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CapsuleShape2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CapsuleShape2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CapsuleShape2D {
    fn construct() -> Self {
        CapsuleShape2D::new()
    }
}

unsafe impl GodotObject for CenterContainer {
    fn class_name() -> &'static str {
        "CenterContainer"
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

impl ToVariant for CenterContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CenterContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CenterContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CenterContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CenterContainer {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CenterContainer {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CenterContainer {
    fn construct() -> Self {
        CenterContainer::new()
    }
}

unsafe impl GodotObject for CharFXTransform {
    fn class_name() -> &'static str {
        "CharFXTransform"
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

impl ToVariant for CharFXTransform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CharFXTransform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CharFXTransform {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CharFXTransform {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CharFXTransform {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CharFXTransform {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CharFXTransform {
    fn construct() -> Self {
        CharFXTransform::new()
    }
}

unsafe impl GodotObject for CheckBox {
    fn class_name() -> &'static str {
        "CheckBox"
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

impl ToVariant for CheckBox {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CheckBox {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CheckBox {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CheckBox {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CheckBox {
    type Target = Button;

    fn deref(&self) -> &Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CheckBox {
    fn deref_mut(&mut self) -> &mut Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CheckBox {
    fn construct() -> Self {
        CheckBox::new()
    }
}

unsafe impl GodotObject for CheckButton {
    fn class_name() -> &'static str {
        "CheckButton"
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

impl ToVariant for CheckButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CheckButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CheckButton {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CheckButton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CheckButton {
    type Target = Button;

    fn deref(&self) -> &Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CheckButton {
    fn deref_mut(&mut self) -> &mut Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CheckButton {
    fn construct() -> Self {
        CheckButton::new()
    }
}

unsafe impl GodotObject for CircleShape2D {
    fn class_name() -> &'static str {
        "CircleShape2D"
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

impl ToVariant for CircleShape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CircleShape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CircleShape2D {
    type Target = Shape2D;

    fn deref(&self) -> &Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CircleShape2D {
    fn deref_mut(&mut self) -> &mut Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CircleShape2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CircleShape2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CircleShape2D {
    fn construct() -> Self {
        CircleShape2D::new()
    }
}

unsafe impl GodotObject for ClippedCamera {
    fn class_name() -> &'static str {
        "ClippedCamera"
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

impl ToVariant for ClippedCamera {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ClippedCamera {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ClippedCamera {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ClippedCamera {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ClippedCamera {
    type Target = Camera;

    fn deref(&self) -> &Camera {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ClippedCamera {
    fn deref_mut(&mut self) -> &mut Camera {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ClippedCamera {
    fn construct() -> Self {
        ClippedCamera::new()
    }
}

unsafe impl GodotObject for CollisionObject {
    fn class_name() -> &'static str {
        "CollisionObject"
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

impl ToVariant for CollisionObject {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CollisionObject {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for CollisionObject {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CollisionObject {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CollisionObject {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for CollisionObject2D {
    fn class_name() -> &'static str {
        "CollisionObject2D"
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

impl ToVariant for CollisionObject2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CollisionObject2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for CollisionObject2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CollisionObject2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CollisionObject2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for CollisionPolygon {
    fn class_name() -> &'static str {
        "CollisionPolygon"
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

impl ToVariant for CollisionPolygon {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CollisionPolygon {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CollisionPolygon {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CollisionPolygon {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CollisionPolygon {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CollisionPolygon {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CollisionPolygon {
    fn construct() -> Self {
        CollisionPolygon::new()
    }
}

unsafe impl GodotObject for CollisionPolygon2D {
    fn class_name() -> &'static str {
        "CollisionPolygon2D"
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

impl ToVariant for CollisionPolygon2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CollisionPolygon2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CollisionPolygon2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CollisionPolygon2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CollisionPolygon2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CollisionPolygon2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CollisionPolygon2D {
    fn construct() -> Self {
        CollisionPolygon2D::new()
    }
}

unsafe impl GodotObject for CollisionShape {
    fn class_name() -> &'static str {
        "CollisionShape"
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

impl ToVariant for CollisionShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CollisionShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CollisionShape {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CollisionShape {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CollisionShape {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CollisionShape {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CollisionShape {
    fn construct() -> Self {
        CollisionShape::new()
    }
}

unsafe impl GodotObject for CollisionShape2D {
    fn class_name() -> &'static str {
        "CollisionShape2D"
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

impl ToVariant for CollisionShape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CollisionShape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for CollisionShape2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for CollisionShape2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for CollisionShape2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CollisionShape2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for CollisionShape2D {
    fn construct() -> Self {
        CollisionShape2D::new()
    }
}

unsafe impl GodotObject for ColorPicker {
    fn class_name() -> &'static str {
        "ColorPicker"
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

impl ToVariant for ColorPicker {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ColorPicker {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ColorPicker {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ColorPicker {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ColorPicker {
    type Target = BoxContainer;

    fn deref(&self) -> &BoxContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ColorPicker {
    fn deref_mut(&mut self) -> &mut BoxContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ColorPicker {
    fn construct() -> Self {
        ColorPicker::new()
    }
}

unsafe impl GodotObject for ColorPickerButton {
    fn class_name() -> &'static str {
        "ColorPickerButton"
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

impl ToVariant for ColorPickerButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ColorPickerButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ColorPickerButton {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ColorPickerButton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ColorPickerButton {
    type Target = Button;

    fn deref(&self) -> &Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ColorPickerButton {
    fn deref_mut(&mut self) -> &mut Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ColorPickerButton {
    fn construct() -> Self {
        ColorPickerButton::new()
    }
}

unsafe impl GodotObject for ColorRect {
    fn class_name() -> &'static str {
        "ColorRect"
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

impl ToVariant for ColorRect {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ColorRect {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ColorRect {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ColorRect {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ColorRect {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ColorRect {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ColorRect {
    fn construct() -> Self {
        ColorRect::new()
    }
}

unsafe impl GodotObject for ConcavePolygonShape {
    fn class_name() -> &'static str {
        "ConcavePolygonShape"
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

impl ToVariant for ConcavePolygonShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ConcavePolygonShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ConcavePolygonShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ConcavePolygonShape {
    fn deref_mut(&mut self) -> &mut Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ConcavePolygonShape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ConcavePolygonShape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ConcavePolygonShape {
    fn construct() -> Self {
        ConcavePolygonShape::new()
    }
}

unsafe impl GodotObject for ConcavePolygonShape2D {
    fn class_name() -> &'static str {
        "ConcavePolygonShape2D"
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

impl ToVariant for ConcavePolygonShape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ConcavePolygonShape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ConcavePolygonShape2D {
    type Target = Shape2D;

    fn deref(&self) -> &Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ConcavePolygonShape2D {
    fn deref_mut(&mut self) -> &mut Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ConcavePolygonShape2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ConcavePolygonShape2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ConcavePolygonShape2D {
    fn construct() -> Self {
        ConcavePolygonShape2D::new()
    }
}

unsafe impl GodotObject for ConeTwistJoint {
    fn class_name() -> &'static str {
        "ConeTwistJoint"
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

impl ToVariant for ConeTwistJoint {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ConeTwistJoint {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ConeTwistJoint {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ConeTwistJoint {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ConeTwistJoint {
    type Target = Joint;

    fn deref(&self) -> &Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ConeTwistJoint {
    fn deref_mut(&mut self) -> &mut Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ConeTwistJoint {
    fn construct() -> Self {
        ConeTwistJoint::new()
    }
}

unsafe impl GodotObject for ConfigFile {
    fn class_name() -> &'static str {
        "ConfigFile"
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

impl ToVariant for ConfigFile {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ConfigFile {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ConfigFile {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ConfigFile {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ConfigFile {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ConfigFile {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ConfigFile {
    fn construct() -> Self {
        ConfigFile::new()
    }
}

unsafe impl GodotObject for ConfirmationDialog {
    fn class_name() -> &'static str {
        "ConfirmationDialog"
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

impl ToVariant for ConfirmationDialog {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ConfirmationDialog {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ConfirmationDialog {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ConfirmationDialog {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ConfirmationDialog {
    type Target = AcceptDialog;

    fn deref(&self) -> &AcceptDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ConfirmationDialog {
    fn deref_mut(&mut self) -> &mut AcceptDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ConfirmationDialog {
    fn construct() -> Self {
        ConfirmationDialog::new()
    }
}

unsafe impl GodotObject for Container {
    fn class_name() -> &'static str {
        "Container"
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

impl ToVariant for Container {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Container {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Container {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Container {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Container {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Container {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Container {
    fn construct() -> Self {
        Container::new()
    }
}

unsafe impl GodotObject for Control {
    fn class_name() -> &'static str {
        "Control"
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

impl ToVariant for Control {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Control {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Control {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Control {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Control {
    type Target = CanvasItem;

    fn deref(&self) -> &CanvasItem {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Control {
    fn deref_mut(&mut self) -> &mut CanvasItem {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Control {
    fn construct() -> Self {
        Control::new()
    }
}

unsafe impl GodotObject for ConvexPolygonShape {
    fn class_name() -> &'static str {
        "ConvexPolygonShape"
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

impl ToVariant for ConvexPolygonShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ConvexPolygonShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ConvexPolygonShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ConvexPolygonShape {
    fn deref_mut(&mut self) -> &mut Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ConvexPolygonShape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ConvexPolygonShape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ConvexPolygonShape {
    fn construct() -> Self {
        ConvexPolygonShape::new()
    }
}

unsafe impl GodotObject for ConvexPolygonShape2D {
    fn class_name() -> &'static str {
        "ConvexPolygonShape2D"
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

impl ToVariant for ConvexPolygonShape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ConvexPolygonShape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ConvexPolygonShape2D {
    type Target = Shape2D;

    fn deref(&self) -> &Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ConvexPolygonShape2D {
    fn deref_mut(&mut self) -> &mut Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ConvexPolygonShape2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ConvexPolygonShape2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ConvexPolygonShape2D {
    fn construct() -> Self {
        ConvexPolygonShape2D::new()
    }
}

unsafe impl GodotObject for Crypto {
    fn class_name() -> &'static str {
        "Crypto"
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

impl ToVariant for Crypto {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Crypto {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Crypto {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Crypto {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Crypto {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Crypto {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Crypto {
    fn construct() -> Self {
        Crypto::new()
    }
}

unsafe impl GodotObject for CryptoKey {
    fn class_name() -> &'static str {
        "CryptoKey"
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

impl ToVariant for CryptoKey {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CryptoKey {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CryptoKey {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CryptoKey {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CryptoKey {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CryptoKey {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CryptoKey {
    fn construct() -> Self {
        CryptoKey::new()
    }
}

unsafe impl GodotObject for CubeMap {
    fn class_name() -> &'static str {
        "CubeMap"
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

impl ToVariant for CubeMap {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CubeMap {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CubeMap {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CubeMap {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CubeMap {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CubeMap {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CubeMap {
    fn construct() -> Self {
        CubeMap::new()
    }
}

unsafe impl GodotObject for CubeMesh {
    fn class_name() -> &'static str {
        "CubeMesh"
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

impl ToVariant for CubeMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CubeMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CubeMesh {
    type Target = PrimitiveMesh;

    fn deref(&self) -> &PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CubeMesh {
    fn deref_mut(&mut self) -> &mut PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CubeMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CubeMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CubeMesh {
    fn construct() -> Self {
        CubeMesh::new()
    }
}

unsafe impl GodotObject for Curve {
    fn class_name() -> &'static str {
        "Curve"
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

impl ToVariant for Curve {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Curve {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Curve {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Curve {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Curve {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Curve {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Curve {
    fn construct() -> Self {
        Curve::new()
    }
}

unsafe impl GodotObject for Curve2D {
    fn class_name() -> &'static str {
        "Curve2D"
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

impl ToVariant for Curve2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Curve2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Curve2D {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Curve2D {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Curve2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Curve2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Curve2D {
    fn construct() -> Self {
        Curve2D::new()
    }
}

unsafe impl GodotObject for Curve3D {
    fn class_name() -> &'static str {
        "Curve3D"
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

impl ToVariant for Curve3D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Curve3D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Curve3D {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Curve3D {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Curve3D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Curve3D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Curve3D {
    fn construct() -> Self {
        Curve3D::new()
    }
}

unsafe impl GodotObject for CurveTexture {
    fn class_name() -> &'static str {
        "CurveTexture"
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

impl ToVariant for CurveTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CurveTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CurveTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CurveTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CurveTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CurveTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CurveTexture {
    fn construct() -> Self {
        CurveTexture::new()
    }
}

unsafe impl GodotObject for CylinderMesh {
    fn class_name() -> &'static str {
        "CylinderMesh"
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

impl ToVariant for CylinderMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CylinderMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CylinderMesh {
    type Target = PrimitiveMesh;

    fn deref(&self) -> &PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CylinderMesh {
    fn deref_mut(&mut self) -> &mut PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CylinderMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CylinderMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CylinderMesh {
    fn construct() -> Self {
        CylinderMesh::new()
    }
}

unsafe impl GodotObject for CylinderShape {
    fn class_name() -> &'static str {
        "CylinderShape"
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

impl ToVariant for CylinderShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for CylinderShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for CylinderShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for CylinderShape {
    fn deref_mut(&mut self) -> &mut Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for CylinderShape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for CylinderShape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for CylinderShape {
    fn construct() -> Self {
        CylinderShape::new()
    }
}

unsafe impl GodotObject for DampedSpringJoint2D {
    fn class_name() -> &'static str {
        "DampedSpringJoint2D"
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

impl ToVariant for DampedSpringJoint2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for DampedSpringJoint2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for DampedSpringJoint2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for DampedSpringJoint2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for DampedSpringJoint2D {
    type Target = Joint2D;

    fn deref(&self) -> &Joint2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for DampedSpringJoint2D {
    fn deref_mut(&mut self) -> &mut Joint2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for DampedSpringJoint2D {
    fn construct() -> Self {
        DampedSpringJoint2D::new()
    }
}

unsafe impl GodotObject for DirectionalLight {
    fn class_name() -> &'static str {
        "DirectionalLight"
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

impl ToVariant for DirectionalLight {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for DirectionalLight {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for DirectionalLight {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for DirectionalLight {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for DirectionalLight {
    type Target = Light;

    fn deref(&self) -> &Light {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for DirectionalLight {
    fn deref_mut(&mut self) -> &mut Light {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for DirectionalLight {
    fn construct() -> Self {
        DirectionalLight::new()
    }
}

unsafe impl GodotObject for DynamicFont {
    fn class_name() -> &'static str {
        "DynamicFont"
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

impl ToVariant for DynamicFont {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for DynamicFont {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for DynamicFont {
    type Target = Font;

    fn deref(&self) -> &Font {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for DynamicFont {
    fn deref_mut(&mut self) -> &mut Font {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for DynamicFont {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for DynamicFont {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for DynamicFont {
    fn construct() -> Self {
        DynamicFont::new()
    }
}

unsafe impl GodotObject for DynamicFontData {
    fn class_name() -> &'static str {
        "DynamicFontData"
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

impl ToVariant for DynamicFontData {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for DynamicFontData {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for DynamicFontData {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for DynamicFontData {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for DynamicFontData {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for DynamicFontData {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for DynamicFontData {
    fn construct() -> Self {
        DynamicFontData::new()
    }
}

unsafe impl GodotObject for EditorExportPlugin {
    fn class_name() -> &'static str {
        "EditorExportPlugin"
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

impl ToVariant for EditorExportPlugin {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorExportPlugin {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorExportPlugin {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorExportPlugin {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorExportPlugin {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorExportPlugin {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorFeatureProfile {
    fn class_name() -> &'static str {
        "EditorFeatureProfile"
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

impl ToVariant for EditorFeatureProfile {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorFeatureProfile {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorFeatureProfile {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorFeatureProfile {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorFeatureProfile {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorFeatureProfile {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorFileDialog {
    fn class_name() -> &'static str {
        "EditorFileDialog"
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

impl ToVariant for EditorFileDialog {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorFileDialog {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for EditorFileDialog {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for EditorFileDialog {
    type Target = ConfirmationDialog;

    fn deref(&self) -> &ConfirmationDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorFileDialog {
    fn deref_mut(&mut self) -> &mut ConfirmationDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorFileSystem {
    fn class_name() -> &'static str {
        "EditorFileSystem"
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

impl ToVariant for EditorFileSystem {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorFileSystem {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for EditorFileSystem {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for EditorFileSystem {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorFileSystem {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorFileSystemDirectory {
    fn class_name() -> &'static str {
        "EditorFileSystemDirectory"
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

impl ToVariant for EditorFileSystemDirectory {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorFileSystemDirectory {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorFileSystemDirectory {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorFileSystemDirectory {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorImportPlugin {
    fn class_name() -> &'static str {
        "EditorImportPlugin"
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

impl ToVariant for EditorImportPlugin {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorImportPlugin {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorImportPlugin {
    type Target = ResourceImporter;

    fn deref(&self) -> &ResourceImporter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorImportPlugin {
    fn deref_mut(&mut self) -> &mut ResourceImporter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorImportPlugin {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorImportPlugin {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorInspector {
    fn class_name() -> &'static str {
        "EditorInspector"
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

impl ToVariant for EditorInspector {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorInspector {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for EditorInspector {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for EditorInspector {
    type Target = ScrollContainer;

    fn deref(&self) -> &ScrollContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorInspector {
    fn deref_mut(&mut self) -> &mut ScrollContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorInspectorPlugin {
    fn class_name() -> &'static str {
        "EditorInspectorPlugin"
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

impl ToVariant for EditorInspectorPlugin {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorInspectorPlugin {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorInspectorPlugin {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorInspectorPlugin {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorInspectorPlugin {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorInspectorPlugin {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorInterface {
    fn class_name() -> &'static str {
        "EditorInterface"
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

impl ToVariant for EditorInterface {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorInterface {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for EditorInterface {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for EditorInterface {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorInterface {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorNavigationMeshGenerator {
    fn class_name() -> &'static str {
        "EditorNavigationMeshGenerator"
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

impl ToVariant for EditorNavigationMeshGenerator {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorNavigationMeshGenerator {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorNavigationMeshGenerator {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorNavigationMeshGenerator {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorPlugin {
    fn class_name() -> &'static str {
        "EditorPlugin"
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

impl ToVariant for EditorPlugin {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorPlugin {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for EditorPlugin {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for EditorPlugin {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorPlugin {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorProperty {
    fn class_name() -> &'static str {
        "EditorProperty"
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

impl ToVariant for EditorProperty {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorProperty {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for EditorProperty {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for EditorProperty {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorProperty {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorResourceConversionPlugin {
    fn class_name() -> &'static str {
        "EditorResourceConversionPlugin"
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

impl ToVariant for EditorResourceConversionPlugin {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorResourceConversionPlugin {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorResourceConversionPlugin {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorResourceConversionPlugin {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorResourceConversionPlugin {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorResourceConversionPlugin {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorResourcePreview {
    fn class_name() -> &'static str {
        "EditorResourcePreview"
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

impl ToVariant for EditorResourcePreview {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorResourcePreview {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for EditorResourcePreview {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for EditorResourcePreview {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorResourcePreview {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorResourcePreviewGenerator {
    fn class_name() -> &'static str {
        "EditorResourcePreviewGenerator"
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

impl ToVariant for EditorResourcePreviewGenerator {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorResourcePreviewGenerator {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorResourcePreviewGenerator {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorResourcePreviewGenerator {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorResourcePreviewGenerator {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorResourcePreviewGenerator {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorSceneImporter {
    fn class_name() -> &'static str {
        "EditorSceneImporter"
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

impl ToVariant for EditorSceneImporter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorSceneImporter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorSceneImporter {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorSceneImporter {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorSceneImporter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorSceneImporter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorSceneImporterAssimp {
    fn class_name() -> &'static str {
        "EditorSceneImporterAssimp"
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

impl ToVariant for EditorSceneImporterAssimp {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorSceneImporterAssimp {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorSceneImporterAssimp {
    type Target = EditorSceneImporter;

    fn deref(&self) -> &EditorSceneImporter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorSceneImporterAssimp {
    fn deref_mut(&mut self) -> &mut EditorSceneImporter {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorSceneImporterAssimp {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorSceneImporterAssimp {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorScenePostImport {
    fn class_name() -> &'static str {
        "EditorScenePostImport"
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

impl ToVariant for EditorScenePostImport {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorScenePostImport {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorScenePostImport {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorScenePostImport {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorScenePostImport {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorScenePostImport {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorScript {
    fn class_name() -> &'static str {
        "EditorScript"
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

impl ToVariant for EditorScript {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorScript {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorScript {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorScript {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorScript {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorScript {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorSelection {
    fn class_name() -> &'static str {
        "EditorSelection"
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

impl ToVariant for EditorSelection {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorSelection {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorSelection {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorSelection {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorSettings {
    fn class_name() -> &'static str {
        "EditorSettings"
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

impl ToVariant for EditorSettings {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorSettings {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorSettings {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorSettings {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorSettings {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorSettings {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorSpatialGizmo {
    fn class_name() -> &'static str {
        "EditorSpatialGizmo"
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

impl ToVariant for EditorSpatialGizmo {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorSpatialGizmo {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorSpatialGizmo {
    type Target = SpatialGizmo;

    fn deref(&self) -> &SpatialGizmo {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorSpatialGizmo {
    fn deref_mut(&mut self) -> &mut SpatialGizmo {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorSpatialGizmo {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorSpatialGizmo {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorSpatialGizmoPlugin {
    fn class_name() -> &'static str {
        "EditorSpatialGizmoPlugin"
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

impl ToVariant for EditorSpatialGizmoPlugin {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorSpatialGizmoPlugin {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorSpatialGizmoPlugin {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorSpatialGizmoPlugin {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EditorSpatialGizmoPlugin {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EditorSpatialGizmoPlugin {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for EditorSpinSlider {
    fn class_name() -> &'static str {
        "EditorSpinSlider"
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

impl ToVariant for EditorSpinSlider {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorSpinSlider {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for EditorSpinSlider {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for EditorSpinSlider {
    type Target = Range;

    fn deref(&self) -> &Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorSpinSlider {
    fn deref_mut(&mut self) -> &mut Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EditorVCSInterface {
    fn class_name() -> &'static str {
        "EditorVCSInterface"
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

impl ToVariant for EditorVCSInterface {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EditorVCSInterface {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EditorVCSInterface {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EditorVCSInterface {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for EncodedObjectAsID {
    fn class_name() -> &'static str {
        "EncodedObjectAsID"
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

impl ToVariant for EncodedObjectAsID {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for EncodedObjectAsID {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for EncodedObjectAsID {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for EncodedObjectAsID {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for EncodedObjectAsID {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for EncodedObjectAsID {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for EncodedObjectAsID {
    fn construct() -> Self {
        EncodedObjectAsID::new()
    }
}

unsafe impl GodotObject for Environment {
    fn class_name() -> &'static str {
        "Environment"
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

impl ToVariant for Environment {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Environment {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Environment {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Environment {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Environment {
    fn construct() -> Self {
        Environment::new()
    }
}

unsafe impl GodotObject for Expression {
    fn class_name() -> &'static str {
        "Expression"
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

impl ToVariant for Expression {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Expression {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Expression {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Expression {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Expression {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Expression {
    fn construct() -> Self {
        Expression::new()
    }
}

unsafe impl GodotObject for FileDialog {
    fn class_name() -> &'static str {
        "FileDialog"
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

impl ToVariant for FileDialog {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for FileDialog {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for FileDialog {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for FileDialog {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for FileDialog {
    type Target = ConfirmationDialog;

    fn deref(&self) -> &ConfirmationDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for FileDialog {
    fn deref_mut(&mut self) -> &mut ConfirmationDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for FileDialog {
    fn construct() -> Self {
        FileDialog::new()
    }
}

unsafe impl GodotObject for Font {
    fn class_name() -> &'static str {
        "Font"
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

impl ToVariant for Font {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Font {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Font {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Font {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Font {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for FuncRef {
    fn class_name() -> &'static str {
        "FuncRef"
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

impl ToVariant for FuncRef {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for FuncRef {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for FuncRef {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for FuncRef {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for FuncRef {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for FuncRef {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for FuncRef {
    fn construct() -> Self {
        FuncRef::new()
    }
}

unsafe impl GodotObject for GDNative {
    fn class_name() -> &'static str {
        "GDNative"
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

impl ToVariant for GDNative {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GDNative {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for GDNative {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GDNative {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for GDNative {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for GDNative {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for GDNative {
    fn construct() -> Self {
        GDNative::new()
    }
}

unsafe impl GodotObject for GDNativeLibrary {
    fn class_name() -> &'static str {
        "GDNativeLibrary"
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

impl ToVariant for GDNativeLibrary {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GDNativeLibrary {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for GDNativeLibrary {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GDNativeLibrary {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for GDNativeLibrary {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for GDNativeLibrary {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for GDNativeLibrary {
    fn construct() -> Self {
        GDNativeLibrary::new()
    }
}

unsafe impl GodotObject for GDScript {
    fn class_name() -> &'static str {
        "GDScript"
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

impl ToVariant for GDScript {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GDScript {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for GDScript {
    type Target = Script;

    fn deref(&self) -> &Script {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GDScript {
    fn deref_mut(&mut self) -> &mut Script {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for GDScript {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for GDScript {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for GDScript {
    fn construct() -> Self {
        GDScript::new()
    }
}

unsafe impl GodotObject for GDScriptFunctionState {
    fn class_name() -> &'static str {
        "GDScriptFunctionState"
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

impl ToVariant for GDScriptFunctionState {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GDScriptFunctionState {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for GDScriptFunctionState {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GDScriptFunctionState {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for GDScriptFunctionState {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for GDScriptFunctionState {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for GIProbe {
    fn class_name() -> &'static str {
        "GIProbe"
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

impl ToVariant for GIProbe {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GIProbe {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for GIProbe {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for GIProbe {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for GIProbe {
    type Target = VisualInstance;

    fn deref(&self) -> &VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GIProbe {
    fn deref_mut(&mut self) -> &mut VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for GIProbe {
    fn construct() -> Self {
        GIProbe::new()
    }
}

unsafe impl GodotObject for GIProbeData {
    fn class_name() -> &'static str {
        "GIProbeData"
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

impl ToVariant for GIProbeData {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GIProbeData {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for GIProbeData {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GIProbeData {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for GIProbeData {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for GIProbeData {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for GIProbeData {
    fn construct() -> Self {
        GIProbeData::new()
    }
}

unsafe impl GodotObject for Generic6DOFJoint {
    fn class_name() -> &'static str {
        "Generic6DOFJoint"
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

impl ToVariant for Generic6DOFJoint {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Generic6DOFJoint {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Generic6DOFJoint {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Generic6DOFJoint {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Generic6DOFJoint {
    type Target = Joint;

    fn deref(&self) -> &Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Generic6DOFJoint {
    fn deref_mut(&mut self) -> &mut Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Generic6DOFJoint {
    fn construct() -> Self {
        Generic6DOFJoint::new()
    }
}

unsafe impl GodotObject for GeometryInstance {
    fn class_name() -> &'static str {
        "GeometryInstance"
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

impl ToVariant for GeometryInstance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GeometryInstance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for GeometryInstance {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for GeometryInstance {
    type Target = VisualInstance;

    fn deref(&self) -> &VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GeometryInstance {
    fn deref_mut(&mut self) -> &mut VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Gradient {
    fn class_name() -> &'static str {
        "Gradient"
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

impl ToVariant for Gradient {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Gradient {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Gradient {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Gradient {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Gradient {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Gradient {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Gradient {
    fn construct() -> Self {
        Gradient::new()
    }
}

unsafe impl GodotObject for GradientTexture {
    fn class_name() -> &'static str {
        "GradientTexture"
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

impl ToVariant for GradientTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GradientTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for GradientTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GradientTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for GradientTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for GradientTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for GradientTexture {
    fn construct() -> Self {
        GradientTexture::new()
    }
}

unsafe impl GodotObject for GraphEdit {
    fn class_name() -> &'static str {
        "GraphEdit"
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

impl ToVariant for GraphEdit {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GraphEdit {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for GraphEdit {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for GraphEdit {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for GraphEdit {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GraphEdit {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for GraphEdit {
    fn construct() -> Self {
        GraphEdit::new()
    }
}

unsafe impl GodotObject for GraphNode {
    fn class_name() -> &'static str {
        "GraphNode"
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

impl ToVariant for GraphNode {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GraphNode {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for GraphNode {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for GraphNode {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for GraphNode {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GraphNode {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for GraphNode {
    fn construct() -> Self {
        GraphNode::new()
    }
}

unsafe impl GodotObject for GridContainer {
    fn class_name() -> &'static str {
        "GridContainer"
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

impl ToVariant for GridContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GridContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for GridContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for GridContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for GridContainer {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GridContainer {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for GridContainer {
    fn construct() -> Self {
        GridContainer::new()
    }
}

unsafe impl GodotObject for GridMap {
    fn class_name() -> &'static str {
        "GridMap"
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

impl ToVariant for GridMap {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GridMap {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for GridMap {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for GridMap {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for GridMap {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GridMap {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for GridMap {
    fn construct() -> Self {
        GridMap::new()
    }
}

unsafe impl GodotObject for GrooveJoint2D {
    fn class_name() -> &'static str {
        "GrooveJoint2D"
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

impl ToVariant for GrooveJoint2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for GrooveJoint2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for GrooveJoint2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for GrooveJoint2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for GrooveJoint2D {
    type Target = Joint2D;

    fn deref(&self) -> &Joint2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for GrooveJoint2D {
    fn deref_mut(&mut self) -> &mut Joint2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for GrooveJoint2D {
    fn construct() -> Self {
        GrooveJoint2D::new()
    }
}

unsafe impl GodotObject for HBoxContainer {
    fn class_name() -> &'static str {
        "HBoxContainer"
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

impl ToVariant for HBoxContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HBoxContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for HBoxContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for HBoxContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for HBoxContainer {
    type Target = BoxContainer;

    fn deref(&self) -> &BoxContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HBoxContainer {
    fn deref_mut(&mut self) -> &mut BoxContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for HBoxContainer {
    fn construct() -> Self {
        HBoxContainer::new()
    }
}

unsafe impl GodotObject for HScrollBar {
    fn class_name() -> &'static str {
        "HScrollBar"
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

impl ToVariant for HScrollBar {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HScrollBar {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for HScrollBar {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for HScrollBar {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for HScrollBar {
    type Target = ScrollBar;

    fn deref(&self) -> &ScrollBar {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HScrollBar {
    fn deref_mut(&mut self) -> &mut ScrollBar {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for HScrollBar {
    fn construct() -> Self {
        HScrollBar::new()
    }
}

unsafe impl GodotObject for HSeparator {
    fn class_name() -> &'static str {
        "HSeparator"
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

impl ToVariant for HSeparator {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HSeparator {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for HSeparator {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for HSeparator {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for HSeparator {
    type Target = Separator;

    fn deref(&self) -> &Separator {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HSeparator {
    fn deref_mut(&mut self) -> &mut Separator {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for HSeparator {
    fn construct() -> Self {
        HSeparator::new()
    }
}

unsafe impl GodotObject for HSlider {
    fn class_name() -> &'static str {
        "HSlider"
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

impl ToVariant for HSlider {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HSlider {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for HSlider {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for HSlider {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for HSlider {
    type Target = Slider;

    fn deref(&self) -> &Slider {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HSlider {
    fn deref_mut(&mut self) -> &mut Slider {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for HSlider {
    fn construct() -> Self {
        HSlider::new()
    }
}

unsafe impl GodotObject for HSplitContainer {
    fn class_name() -> &'static str {
        "HSplitContainer"
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

impl ToVariant for HSplitContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HSplitContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for HSplitContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for HSplitContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for HSplitContainer {
    type Target = SplitContainer;

    fn deref(&self) -> &SplitContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HSplitContainer {
    fn deref_mut(&mut self) -> &mut SplitContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for HSplitContainer {
    fn construct() -> Self {
        HSplitContainer::new()
    }
}

unsafe impl GodotObject for HTTPClient {
    fn class_name() -> &'static str {
        "HTTPClient"
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

impl ToVariant for HTTPClient {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HTTPClient {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for HTTPClient {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HTTPClient {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for HTTPClient {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for HTTPClient {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for HTTPClient {
    fn construct() -> Self {
        HTTPClient::new()
    }
}

unsafe impl GodotObject for HTTPRequest {
    fn class_name() -> &'static str {
        "HTTPRequest"
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

impl ToVariant for HTTPRequest {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HTTPRequest {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for HTTPRequest {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for HTTPRequest {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for HTTPRequest {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HTTPRequest {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for HTTPRequest {
    fn construct() -> Self {
        HTTPRequest::new()
    }
}

unsafe impl GodotObject for HashingContext {
    fn class_name() -> &'static str {
        "HashingContext"
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

impl ToVariant for HashingContext {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HashingContext {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for HashingContext {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HashingContext {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for HashingContext {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for HashingContext {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for HashingContext {
    fn construct() -> Self {
        HashingContext::new()
    }
}

unsafe impl GodotObject for HeightMapShape {
    fn class_name() -> &'static str {
        "HeightMapShape"
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

impl ToVariant for HeightMapShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HeightMapShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for HeightMapShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HeightMapShape {
    fn deref_mut(&mut self) -> &mut Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for HeightMapShape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for HeightMapShape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for HeightMapShape {
    fn construct() -> Self {
        HeightMapShape::new()
    }
}

unsafe impl GodotObject for HingeJoint {
    fn class_name() -> &'static str {
        "HingeJoint"
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

impl ToVariant for HingeJoint {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for HingeJoint {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for HingeJoint {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for HingeJoint {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for HingeJoint {
    type Target = Joint;

    fn deref(&self) -> &Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for HingeJoint {
    fn deref_mut(&mut self) -> &mut Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for HingeJoint {
    fn construct() -> Self {
        HingeJoint::new()
    }
}

unsafe impl GodotObject for IP {
    fn class_name() -> &'static str {
        "IP"
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

impl ToVariant for IP {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for IP {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for IP {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for IP {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for IP_Unix {
    fn class_name() -> &'static str {
        "IP_Unix"
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

impl ToVariant for IP_Unix {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for IP_Unix {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for IP_Unix {
    type Target = IP;

    fn deref(&self) -> &IP {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for IP_Unix {
    fn deref_mut(&mut self) -> &mut IP {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Image {
    fn class_name() -> &'static str {
        "Image"
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

impl ToVariant for Image {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Image {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Image {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Image {
    fn construct() -> Self {
        Image::new()
    }
}

unsafe impl GodotObject for ImageTexture {
    fn class_name() -> &'static str {
        "ImageTexture"
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

impl ToVariant for ImageTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ImageTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ImageTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ImageTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ImageTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ImageTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ImageTexture {
    fn construct() -> Self {
        ImageTexture::new()
    }
}

unsafe impl GodotObject for ImmediateGeometry {
    fn class_name() -> &'static str {
        "ImmediateGeometry"
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

impl ToVariant for ImmediateGeometry {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ImmediateGeometry {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ImmediateGeometry {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ImmediateGeometry {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ImmediateGeometry {
    type Target = GeometryInstance;

    fn deref(&self) -> &GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ImmediateGeometry {
    fn deref_mut(&mut self) -> &mut GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ImmediateGeometry {
    fn construct() -> Self {
        ImmediateGeometry::new()
    }
}

unsafe impl GodotObject for Input {
    fn class_name() -> &'static str {
        "Input"
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

impl ToVariant for Input {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Input {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Input {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for InputDefault {
    fn class_name() -> &'static str {
        "InputDefault"
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

impl ToVariant for InputDefault {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputDefault {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputDefault {
    type Target = Input;

    fn deref(&self) -> &Input {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputDefault {
    fn deref_mut(&mut self) -> &mut Input {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for InputEvent {
    fn class_name() -> &'static str {
        "InputEvent"
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

impl ToVariant for InputEvent {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEvent {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEvent {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEvent {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEvent {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEvent {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for InputEventAction {
    fn class_name() -> &'static str {
        "InputEventAction"
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

impl ToVariant for InputEventAction {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventAction {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventAction {
    type Target = InputEvent;

    fn deref(&self) -> &InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventAction {
    fn deref_mut(&mut self) -> &mut InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventAction {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventAction {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventAction {
    fn construct() -> Self {
        InputEventAction::new()
    }
}

unsafe impl GodotObject for InputEventGesture {
    fn class_name() -> &'static str {
        "InputEventGesture"
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

impl ToVariant for InputEventGesture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventGesture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventGesture {
    type Target = InputEventWithModifiers;

    fn deref(&self) -> &InputEventWithModifiers {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventGesture {
    fn deref_mut(&mut self) -> &mut InputEventWithModifiers {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventGesture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventGesture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for InputEventJoypadButton {
    fn class_name() -> &'static str {
        "InputEventJoypadButton"
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

impl ToVariant for InputEventJoypadButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventJoypadButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventJoypadButton {
    type Target = InputEvent;

    fn deref(&self) -> &InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventJoypadButton {
    fn deref_mut(&mut self) -> &mut InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventJoypadButton {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventJoypadButton {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventJoypadButton {
    fn construct() -> Self {
        InputEventJoypadButton::new()
    }
}

unsafe impl GodotObject for InputEventJoypadMotion {
    fn class_name() -> &'static str {
        "InputEventJoypadMotion"
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

impl ToVariant for InputEventJoypadMotion {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventJoypadMotion {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventJoypadMotion {
    type Target = InputEvent;

    fn deref(&self) -> &InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventJoypadMotion {
    fn deref_mut(&mut self) -> &mut InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventJoypadMotion {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventJoypadMotion {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventJoypadMotion {
    fn construct() -> Self {
        InputEventJoypadMotion::new()
    }
}

unsafe impl GodotObject for InputEventKey {
    fn class_name() -> &'static str {
        "InputEventKey"
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

impl ToVariant for InputEventKey {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventKey {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventKey {
    type Target = InputEventWithModifiers;

    fn deref(&self) -> &InputEventWithModifiers {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventKey {
    fn deref_mut(&mut self) -> &mut InputEventWithModifiers {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventKey {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventKey {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventKey {
    fn construct() -> Self {
        InputEventKey::new()
    }
}

unsafe impl GodotObject for InputEventMIDI {
    fn class_name() -> &'static str {
        "InputEventMIDI"
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

impl ToVariant for InputEventMIDI {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventMIDI {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventMIDI {
    type Target = InputEvent;

    fn deref(&self) -> &InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventMIDI {
    fn deref_mut(&mut self) -> &mut InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventMIDI {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventMIDI {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventMIDI {
    fn construct() -> Self {
        InputEventMIDI::new()
    }
}

unsafe impl GodotObject for InputEventMagnifyGesture {
    fn class_name() -> &'static str {
        "InputEventMagnifyGesture"
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

impl ToVariant for InputEventMagnifyGesture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventMagnifyGesture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventMagnifyGesture {
    type Target = InputEventGesture;

    fn deref(&self) -> &InputEventGesture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventMagnifyGesture {
    fn deref_mut(&mut self) -> &mut InputEventGesture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventMagnifyGesture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventMagnifyGesture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventMagnifyGesture {
    fn construct() -> Self {
        InputEventMagnifyGesture::new()
    }
}

unsafe impl GodotObject for InputEventMouse {
    fn class_name() -> &'static str {
        "InputEventMouse"
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

impl ToVariant for InputEventMouse {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventMouse {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventMouse {
    type Target = InputEventWithModifiers;

    fn deref(&self) -> &InputEventWithModifiers {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventMouse {
    fn deref_mut(&mut self) -> &mut InputEventWithModifiers {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventMouse {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventMouse {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for InputEventMouseButton {
    fn class_name() -> &'static str {
        "InputEventMouseButton"
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

impl ToVariant for InputEventMouseButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventMouseButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventMouseButton {
    type Target = InputEventMouse;

    fn deref(&self) -> &InputEventMouse {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventMouseButton {
    fn deref_mut(&mut self) -> &mut InputEventMouse {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventMouseButton {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventMouseButton {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventMouseButton {
    fn construct() -> Self {
        InputEventMouseButton::new()
    }
}

unsafe impl GodotObject for InputEventMouseMotion {
    fn class_name() -> &'static str {
        "InputEventMouseMotion"
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

impl ToVariant for InputEventMouseMotion {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventMouseMotion {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventMouseMotion {
    type Target = InputEventMouse;

    fn deref(&self) -> &InputEventMouse {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventMouseMotion {
    fn deref_mut(&mut self) -> &mut InputEventMouse {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventMouseMotion {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventMouseMotion {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventMouseMotion {
    fn construct() -> Self {
        InputEventMouseMotion::new()
    }
}

unsafe impl GodotObject for InputEventPanGesture {
    fn class_name() -> &'static str {
        "InputEventPanGesture"
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

impl ToVariant for InputEventPanGesture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventPanGesture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventPanGesture {
    type Target = InputEventGesture;

    fn deref(&self) -> &InputEventGesture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventPanGesture {
    fn deref_mut(&mut self) -> &mut InputEventGesture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventPanGesture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventPanGesture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventPanGesture {
    fn construct() -> Self {
        InputEventPanGesture::new()
    }
}

unsafe impl GodotObject for InputEventScreenDrag {
    fn class_name() -> &'static str {
        "InputEventScreenDrag"
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

impl ToVariant for InputEventScreenDrag {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventScreenDrag {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventScreenDrag {
    type Target = InputEvent;

    fn deref(&self) -> &InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventScreenDrag {
    fn deref_mut(&mut self) -> &mut InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventScreenDrag {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventScreenDrag {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventScreenDrag {
    fn construct() -> Self {
        InputEventScreenDrag::new()
    }
}

unsafe impl GodotObject for InputEventScreenTouch {
    fn class_name() -> &'static str {
        "InputEventScreenTouch"
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

impl ToVariant for InputEventScreenTouch {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventScreenTouch {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventScreenTouch {
    type Target = InputEvent;

    fn deref(&self) -> &InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventScreenTouch {
    fn deref_mut(&mut self) -> &mut InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventScreenTouch {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventScreenTouch {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for InputEventScreenTouch {
    fn construct() -> Self {
        InputEventScreenTouch::new()
    }
}

unsafe impl GodotObject for InputEventWithModifiers {
    fn class_name() -> &'static str {
        "InputEventWithModifiers"
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

impl ToVariant for InputEventWithModifiers {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputEventWithModifiers {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputEventWithModifiers {
    type Target = InputEvent;

    fn deref(&self) -> &InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputEventWithModifiers {
    fn deref_mut(&mut self) -> &mut InputEvent {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for InputEventWithModifiers {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for InputEventWithModifiers {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for InputMap {
    fn class_name() -> &'static str {
        "InputMap"
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

impl ToVariant for InputMap {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InputMap {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for InputMap {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InputMap {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for InstancePlaceholder {
    fn class_name() -> &'static str {
        "InstancePlaceholder"
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

impl ToVariant for InstancePlaceholder {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InstancePlaceholder {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for InstancePlaceholder {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for InstancePlaceholder {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InstancePlaceholder {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for InterpolatedCamera {
    fn class_name() -> &'static str {
        "InterpolatedCamera"
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

impl ToVariant for InterpolatedCamera {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for InterpolatedCamera {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for InterpolatedCamera {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for InterpolatedCamera {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for InterpolatedCamera {
    type Target = Camera;

    fn deref(&self) -> &Camera {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for InterpolatedCamera {
    fn deref_mut(&mut self) -> &mut Camera {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for InterpolatedCamera {
    fn construct() -> Self {
        InterpolatedCamera::new()
    }
}

unsafe impl GodotObject for ItemList {
    fn class_name() -> &'static str {
        "ItemList"
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

impl ToVariant for ItemList {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ItemList {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ItemList {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ItemList {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ItemList {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ItemList {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ItemList {
    fn construct() -> Self {
        ItemList::new()
    }
}

unsafe impl GodotObject for JSONParseResult {
    fn class_name() -> &'static str {
        "JSONParseResult"
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

impl ToVariant for JSONParseResult {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for JSONParseResult {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for JSONParseResult {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for JSONParseResult {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for JSONParseResult {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for JSONParseResult {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for JSONParseResult {
    fn construct() -> Self {
        JSONParseResult::new()
    }
}

unsafe impl GodotObject for JSONRPC {
    fn class_name() -> &'static str {
        "JSONRPC"
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

impl ToVariant for JSONRPC {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for JSONRPC {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for JSONRPC {
    unsafe fn godot_free(self) { self.free() }
}

impl std::ops::Deref for JSONRPC {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for JSONRPC {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for JSONRPC {
    fn construct() -> Self {
        JSONRPC::new()
    }
}

unsafe impl GodotObject for JavaClass {
    fn class_name() -> &'static str {
        "JavaClass"
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

impl ToVariant for JavaClass {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for JavaClass {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for JavaClass {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for JavaClass {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for JavaClass {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for JavaClass {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for JavaClass {
    fn construct() -> Self {
        JavaClass::new()
    }
}

unsafe impl GodotObject for JavaClassWrapper {
    fn class_name() -> &'static str {
        "JavaClassWrapper"
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

impl ToVariant for JavaClassWrapper {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for JavaClassWrapper {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for JavaClassWrapper {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for JavaClassWrapper {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for JavaScript {
    fn class_name() -> &'static str {
        "JavaScript"
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

impl ToVariant for JavaScript {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for JavaScript {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for JavaScript {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for JavaScript {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Joint {
    fn class_name() -> &'static str {
        "Joint"
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

impl ToVariant for Joint {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Joint {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for Joint {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Joint {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Joint {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Joint2D {
    fn class_name() -> &'static str {
        "Joint2D"
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

impl ToVariant for Joint2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Joint2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for Joint2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Joint2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Joint2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for KinematicBody {
    fn class_name() -> &'static str {
        "KinematicBody"
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

impl ToVariant for KinematicBody {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for KinematicBody {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for KinematicBody {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for KinematicBody {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for KinematicBody {
    type Target = PhysicsBody;

    fn deref(&self) -> &PhysicsBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for KinematicBody {
    fn deref_mut(&mut self) -> &mut PhysicsBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for KinematicBody {
    fn construct() -> Self {
        KinematicBody::new()
    }
}

unsafe impl GodotObject for KinematicBody2D {
    fn class_name() -> &'static str {
        "KinematicBody2D"
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

impl ToVariant for KinematicBody2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for KinematicBody2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for KinematicBody2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for KinematicBody2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for KinematicBody2D {
    type Target = PhysicsBody2D;

    fn deref(&self) -> &PhysicsBody2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for KinematicBody2D {
    fn deref_mut(&mut self) -> &mut PhysicsBody2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for KinematicBody2D {
    fn construct() -> Self {
        KinematicBody2D::new()
    }
}

unsafe impl GodotObject for KinematicCollision {
    fn class_name() -> &'static str {
        "KinematicCollision"
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

impl ToVariant for KinematicCollision {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for KinematicCollision {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for KinematicCollision {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for KinematicCollision {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for KinematicCollision {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for KinematicCollision {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for KinematicCollision {
    fn construct() -> Self {
        KinematicCollision::new()
    }
}

unsafe impl GodotObject for KinematicCollision2D {
    fn class_name() -> &'static str {
        "KinematicCollision2D"
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

impl ToVariant for KinematicCollision2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for KinematicCollision2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for KinematicCollision2D {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for KinematicCollision2D {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for KinematicCollision2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for KinematicCollision2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for KinematicCollision2D {
    fn construct() -> Self {
        KinematicCollision2D::new()
    }
}

unsafe impl GodotObject for Label {
    fn class_name() -> &'static str {
        "Label"
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

impl ToVariant for Label {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Label {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Label {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Label {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Label {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Label {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Label {
    fn construct() -> Self {
        Label::new()
    }
}

unsafe impl GodotObject for LargeTexture {
    fn class_name() -> &'static str {
        "LargeTexture"
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

impl ToVariant for LargeTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for LargeTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for LargeTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for LargeTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for LargeTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for LargeTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for LargeTexture {
    fn construct() -> Self {
        LargeTexture::new()
    }
}

unsafe impl GodotObject for Light {
    fn class_name() -> &'static str {
        "Light"
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

impl ToVariant for Light {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Light {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for Light {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Light {
    type Target = VisualInstance;

    fn deref(&self) -> &VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Light {
    fn deref_mut(&mut self) -> &mut VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Light2D {
    fn class_name() -> &'static str {
        "Light2D"
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

impl ToVariant for Light2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Light2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Light2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Light2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Light2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Light2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Light2D {
    fn construct() -> Self {
        Light2D::new()
    }
}

unsafe impl GodotObject for LightOccluder2D {
    fn class_name() -> &'static str {
        "LightOccluder2D"
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

impl ToVariant for LightOccluder2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for LightOccluder2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for LightOccluder2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for LightOccluder2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for LightOccluder2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for LightOccluder2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for LightOccluder2D {
    fn construct() -> Self {
        LightOccluder2D::new()
    }
}

unsafe impl GodotObject for Line2D {
    fn class_name() -> &'static str {
        "Line2D"
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

impl ToVariant for Line2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Line2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Line2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Line2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Line2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Line2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Line2D {
    fn construct() -> Self {
        Line2D::new()
    }
}

unsafe impl GodotObject for LineEdit {
    fn class_name() -> &'static str {
        "LineEdit"
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

impl ToVariant for LineEdit {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for LineEdit {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for LineEdit {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for LineEdit {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for LineEdit {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for LineEdit {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for LineEdit {
    fn construct() -> Self {
        LineEdit::new()
    }
}

unsafe impl GodotObject for LineShape2D {
    fn class_name() -> &'static str {
        "LineShape2D"
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

impl ToVariant for LineShape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for LineShape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for LineShape2D {
    type Target = Shape2D;

    fn deref(&self) -> &Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for LineShape2D {
    fn deref_mut(&mut self) -> &mut Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for LineShape2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for LineShape2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for LineShape2D {
    fn construct() -> Self {
        LineShape2D::new()
    }
}

unsafe impl GodotObject for LinkButton {
    fn class_name() -> &'static str {
        "LinkButton"
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

impl ToVariant for LinkButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for LinkButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for LinkButton {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for LinkButton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for LinkButton {
    type Target = BaseButton;

    fn deref(&self) -> &BaseButton {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for LinkButton {
    fn deref_mut(&mut self) -> &mut BaseButton {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for LinkButton {
    fn construct() -> Self {
        LinkButton::new()
    }
}

unsafe impl GodotObject for Listener {
    fn class_name() -> &'static str {
        "Listener"
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

impl ToVariant for Listener {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Listener {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Listener {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Listener {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Listener {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Listener {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Listener {
    fn construct() -> Self {
        Listener::new()
    }
}

unsafe impl GodotObject for MainLoop {
    fn class_name() -> &'static str {
        "MainLoop"
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

impl ToVariant for MainLoop {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MainLoop {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for MainLoop {
    unsafe fn godot_free(self) { self.free() }
}

impl std::ops::Deref for MainLoop {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MainLoop {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for MainLoop {
    fn construct() -> Self {
        MainLoop::new()
    }
}

unsafe impl GodotObject for MarginContainer {
    fn class_name() -> &'static str {
        "MarginContainer"
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

impl ToVariant for MarginContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MarginContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for MarginContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for MarginContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for MarginContainer {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MarginContainer {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for MarginContainer {
    fn construct() -> Self {
        MarginContainer::new()
    }
}

unsafe impl GodotObject for Material {
    fn class_name() -> &'static str {
        "Material"
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

impl ToVariant for Material {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Material {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Material {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Material {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Material {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for MenuButton {
    fn class_name() -> &'static str {
        "MenuButton"
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

impl ToVariant for MenuButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MenuButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for MenuButton {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for MenuButton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for MenuButton {
    type Target = Button;

    fn deref(&self) -> &Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MenuButton {
    fn deref_mut(&mut self) -> &mut Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for MenuButton {
    fn construct() -> Self {
        MenuButton::new()
    }
}

unsafe impl GodotObject for Mesh {
    fn class_name() -> &'static str {
        "Mesh"
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

impl ToVariant for Mesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Mesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Mesh {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Mesh {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for MeshDataTool {
    fn class_name() -> &'static str {
        "MeshDataTool"
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

impl ToVariant for MeshDataTool {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MeshDataTool {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for MeshDataTool {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MeshDataTool {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for MeshDataTool {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for MeshDataTool {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for MeshDataTool {
    fn construct() -> Self {
        MeshDataTool::new()
    }
}

unsafe impl GodotObject for MeshInstance {
    fn class_name() -> &'static str {
        "MeshInstance"
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

impl ToVariant for MeshInstance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MeshInstance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for MeshInstance {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for MeshInstance {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for MeshInstance {
    type Target = GeometryInstance;

    fn deref(&self) -> &GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MeshInstance {
    fn deref_mut(&mut self) -> &mut GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for MeshInstance {
    fn construct() -> Self {
        MeshInstance::new()
    }
}

unsafe impl GodotObject for MeshInstance2D {
    fn class_name() -> &'static str {
        "MeshInstance2D"
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

impl ToVariant for MeshInstance2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MeshInstance2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for MeshInstance2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for MeshInstance2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for MeshInstance2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MeshInstance2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for MeshInstance2D {
    fn construct() -> Self {
        MeshInstance2D::new()
    }
}

unsafe impl GodotObject for MeshLibrary {
    fn class_name() -> &'static str {
        "MeshLibrary"
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

impl ToVariant for MeshLibrary {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MeshLibrary {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for MeshLibrary {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MeshLibrary {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for MeshLibrary {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for MeshLibrary {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for MeshLibrary {
    fn construct() -> Self {
        MeshLibrary::new()
    }
}

unsafe impl GodotObject for MeshTexture {
    fn class_name() -> &'static str {
        "MeshTexture"
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

impl ToVariant for MeshTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MeshTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for MeshTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MeshTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for MeshTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for MeshTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for MeshTexture {
    fn construct() -> Self {
        MeshTexture::new()
    }
}

unsafe impl GodotObject for MobileVRInterface {
    fn class_name() -> &'static str {
        "MobileVRInterface"
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

impl ToVariant for MobileVRInterface {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MobileVRInterface {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for MobileVRInterface {
    type Target = ARVRInterface;

    fn deref(&self) -> &ARVRInterface {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MobileVRInterface {
    fn deref_mut(&mut self) -> &mut ARVRInterface {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for MobileVRInterface {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for MobileVRInterface {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for MobileVRInterface {
    fn construct() -> Self {
        MobileVRInterface::new()
    }
}

unsafe impl GodotObject for MultiMesh {
    fn class_name() -> &'static str {
        "MultiMesh"
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

impl ToVariant for MultiMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MultiMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for MultiMesh {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MultiMesh {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for MultiMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for MultiMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for MultiMesh {
    fn construct() -> Self {
        MultiMesh::new()
    }
}

unsafe impl GodotObject for MultiMeshInstance {
    fn class_name() -> &'static str {
        "MultiMeshInstance"
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

impl ToVariant for MultiMeshInstance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MultiMeshInstance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for MultiMeshInstance {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for MultiMeshInstance {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for MultiMeshInstance {
    type Target = GeometryInstance;

    fn deref(&self) -> &GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MultiMeshInstance {
    fn deref_mut(&mut self) -> &mut GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for MultiMeshInstance {
    fn construct() -> Self {
        MultiMeshInstance::new()
    }
}

unsafe impl GodotObject for MultiMeshInstance2D {
    fn class_name() -> &'static str {
        "MultiMeshInstance2D"
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

impl ToVariant for MultiMeshInstance2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MultiMeshInstance2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for MultiMeshInstance2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for MultiMeshInstance2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for MultiMeshInstance2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MultiMeshInstance2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for MultiMeshInstance2D {
    fn construct() -> Self {
        MultiMeshInstance2D::new()
    }
}

unsafe impl GodotObject for MultiplayerAPI {
    fn class_name() -> &'static str {
        "MultiplayerAPI"
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

impl ToVariant for MultiplayerAPI {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MultiplayerAPI {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for MultiplayerAPI {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MultiplayerAPI {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for MultiplayerAPI {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for MultiplayerAPI {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for MultiplayerAPI {
    fn construct() -> Self {
        MultiplayerAPI::new()
    }
}

unsafe impl GodotObject for MultiplayerPeerGDNative {
    fn class_name() -> &'static str {
        "MultiplayerPeerGDNative"
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

impl ToVariant for MultiplayerPeerGDNative {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for MultiplayerPeerGDNative {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for MultiplayerPeerGDNative {
    type Target = NetworkedMultiplayerPeer;

    fn deref(&self) -> &NetworkedMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for MultiplayerPeerGDNative {
    fn deref_mut(&mut self) -> &mut NetworkedMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for MultiplayerPeerGDNative {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for MultiplayerPeerGDNative {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for MultiplayerPeerGDNative {
    fn construct() -> Self {
        MultiplayerPeerGDNative::new()
    }
}

unsafe impl GodotObject for NativeScript {
    fn class_name() -> &'static str {
        "NativeScript"
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

impl ToVariant for NativeScript {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for NativeScript {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for NativeScript {
    type Target = Script;

    fn deref(&self) -> &Script {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for NativeScript {
    fn deref_mut(&mut self) -> &mut Script {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for NativeScript {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for NativeScript {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for NativeScript {
    fn construct() -> Self {
        NativeScript::new()
    }
}

unsafe impl GodotObject for Navigation {
    fn class_name() -> &'static str {
        "Navigation"
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

impl ToVariant for Navigation {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Navigation {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Navigation {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Navigation {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Navigation {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Navigation {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Navigation {
    fn construct() -> Self {
        Navigation::new()
    }
}

unsafe impl GodotObject for Navigation2D {
    fn class_name() -> &'static str {
        "Navigation2D"
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

impl ToVariant for Navigation2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Navigation2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Navigation2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Navigation2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Navigation2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Navigation2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Navigation2D {
    fn construct() -> Self {
        Navigation2D::new()
    }
}

unsafe impl GodotObject for NavigationMesh {
    fn class_name() -> &'static str {
        "NavigationMesh"
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

impl ToVariant for NavigationMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for NavigationMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for NavigationMesh {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for NavigationMesh {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for NavigationMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for NavigationMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for NavigationMesh {
    fn construct() -> Self {
        NavigationMesh::new()
    }
}

unsafe impl GodotObject for NavigationMeshInstance {
    fn class_name() -> &'static str {
        "NavigationMeshInstance"
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

impl ToVariant for NavigationMeshInstance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for NavigationMeshInstance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for NavigationMeshInstance {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for NavigationMeshInstance {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for NavigationMeshInstance {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for NavigationMeshInstance {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for NavigationMeshInstance {
    fn construct() -> Self {
        NavigationMeshInstance::new()
    }
}

unsafe impl GodotObject for NavigationPolygon {
    fn class_name() -> &'static str {
        "NavigationPolygon"
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

impl ToVariant for NavigationPolygon {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for NavigationPolygon {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for NavigationPolygon {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for NavigationPolygon {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for NavigationPolygon {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for NavigationPolygon {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for NavigationPolygon {
    fn construct() -> Self {
        NavigationPolygon::new()
    }
}

unsafe impl GodotObject for NavigationPolygonInstance {
    fn class_name() -> &'static str {
        "NavigationPolygonInstance"
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

impl ToVariant for NavigationPolygonInstance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for NavigationPolygonInstance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for NavigationPolygonInstance {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for NavigationPolygonInstance {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for NavigationPolygonInstance {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for NavigationPolygonInstance {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for NavigationPolygonInstance {
    fn construct() -> Self {
        NavigationPolygonInstance::new()
    }
}

unsafe impl GodotObject for NetworkedMultiplayerENet {
    fn class_name() -> &'static str {
        "NetworkedMultiplayerENet"
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

impl ToVariant for NetworkedMultiplayerENet {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for NetworkedMultiplayerENet {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for NetworkedMultiplayerENet {
    type Target = NetworkedMultiplayerPeer;

    fn deref(&self) -> &NetworkedMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for NetworkedMultiplayerENet {
    fn deref_mut(&mut self) -> &mut NetworkedMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for NetworkedMultiplayerENet {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for NetworkedMultiplayerENet {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for NetworkedMultiplayerENet {
    fn construct() -> Self {
        NetworkedMultiplayerENet::new()
    }
}

unsafe impl GodotObject for NetworkedMultiplayerPeer {
    fn class_name() -> &'static str {
        "NetworkedMultiplayerPeer"
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

impl ToVariant for NetworkedMultiplayerPeer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for NetworkedMultiplayerPeer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for NetworkedMultiplayerPeer {
    type Target = PacketPeer;

    fn deref(&self) -> &PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for NetworkedMultiplayerPeer {
    fn deref_mut(&mut self) -> &mut PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for NetworkedMultiplayerPeer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for NetworkedMultiplayerPeer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for NinePatchRect {
    fn class_name() -> &'static str {
        "NinePatchRect"
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

impl ToVariant for NinePatchRect {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for NinePatchRect {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for NinePatchRect {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for NinePatchRect {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for NinePatchRect {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for NinePatchRect {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for NinePatchRect {
    fn construct() -> Self {
        NinePatchRect::new()
    }
}

unsafe impl GodotObject for Node {
    fn class_name() -> &'static str {
        "Node"
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

impl ToVariant for Node {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Node {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Node {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Node {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Node {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Node {
    fn construct() -> Self {
        Node::new()
    }
}

unsafe impl GodotObject for Node2D {
    fn class_name() -> &'static str {
        "Node2D"
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

impl ToVariant for Node2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Node2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Node2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Node2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Node2D {
    type Target = CanvasItem;

    fn deref(&self) -> &CanvasItem {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Node2D {
    fn deref_mut(&mut self) -> &mut CanvasItem {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Node2D {
    fn construct() -> Self {
        Node2D::new()
    }
}

unsafe impl GodotObject for NoiseTexture {
    fn class_name() -> &'static str {
        "NoiseTexture"
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

impl ToVariant for NoiseTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for NoiseTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for NoiseTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for NoiseTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for NoiseTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for NoiseTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for NoiseTexture {
    fn construct() -> Self {
        NoiseTexture::new()
    }
}

unsafe impl GodotObject for OccluderPolygon2D {
    fn class_name() -> &'static str {
        "OccluderPolygon2D"
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

impl ToVariant for OccluderPolygon2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for OccluderPolygon2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for OccluderPolygon2D {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for OccluderPolygon2D {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for OccluderPolygon2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for OccluderPolygon2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for OccluderPolygon2D {
    fn construct() -> Self {
        OccluderPolygon2D::new()
    }
}

unsafe impl GodotObject for OmniLight {
    fn class_name() -> &'static str {
        "OmniLight"
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

impl ToVariant for OmniLight {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for OmniLight {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for OmniLight {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for OmniLight {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for OmniLight {
    type Target = Light;

    fn deref(&self) -> &Light {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for OmniLight {
    fn deref_mut(&mut self) -> &mut Light {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for OmniLight {
    fn construct() -> Self {
        OmniLight::new()
    }
}

unsafe impl GodotObject for OpenSimplexNoise {
    fn class_name() -> &'static str {
        "OpenSimplexNoise"
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

impl ToVariant for OpenSimplexNoise {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for OpenSimplexNoise {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for OpenSimplexNoise {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for OpenSimplexNoise {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for OpenSimplexNoise {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for OpenSimplexNoise {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for OpenSimplexNoise {
    fn construct() -> Self {
        OpenSimplexNoise::new()
    }
}

unsafe impl GodotObject for OptionButton {
    fn class_name() -> &'static str {
        "OptionButton"
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

impl ToVariant for OptionButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for OptionButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for OptionButton {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for OptionButton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for OptionButton {
    type Target = Button;

    fn deref(&self) -> &Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for OptionButton {
    fn deref_mut(&mut self) -> &mut Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for OptionButton {
    fn construct() -> Self {
        OptionButton::new()
    }
}

unsafe impl GodotObject for PCKPacker {
    fn class_name() -> &'static str {
        "PCKPacker"
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

impl ToVariant for PCKPacker {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PCKPacker {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PCKPacker {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PCKPacker {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PCKPacker {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PCKPacker {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PCKPacker {
    fn construct() -> Self {
        PCKPacker::new()
    }
}

unsafe impl GodotObject for PHashTranslation {
    fn class_name() -> &'static str {
        "PHashTranslation"
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

impl ToVariant for PHashTranslation {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PHashTranslation {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PHashTranslation {
    type Target = Translation;

    fn deref(&self) -> &Translation {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PHashTranslation {
    fn deref_mut(&mut self) -> &mut Translation {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PHashTranslation {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PHashTranslation {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PHashTranslation {
    fn construct() -> Self {
        PHashTranslation::new()
    }
}

unsafe impl GodotObject for PackedDataContainer {
    fn class_name() -> &'static str {
        "PackedDataContainer"
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

impl ToVariant for PackedDataContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PackedDataContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PackedDataContainer {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PackedDataContainer {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PackedDataContainer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PackedDataContainer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PackedDataContainer {
    fn construct() -> Self {
        PackedDataContainer::new()
    }
}

unsafe impl GodotObject for PackedDataContainerRef {
    fn class_name() -> &'static str {
        "PackedDataContainerRef"
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

impl ToVariant for PackedDataContainerRef {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PackedDataContainerRef {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PackedDataContainerRef {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PackedDataContainerRef {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PackedDataContainerRef {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PackedDataContainerRef {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for PackedScene {
    fn class_name() -> &'static str {
        "PackedScene"
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

impl ToVariant for PackedScene {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PackedScene {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PackedScene {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PackedScene {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PackedScene {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PackedScene {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PackedScene {
    fn construct() -> Self {
        PackedScene::new()
    }
}

unsafe impl GodotObject for PacketPeer {
    fn class_name() -> &'static str {
        "PacketPeer"
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

impl ToVariant for PacketPeer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PacketPeer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PacketPeer {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PacketPeer {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PacketPeer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PacketPeer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for PacketPeerGDNative {
    fn class_name() -> &'static str {
        "PacketPeerGDNative"
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

impl ToVariant for PacketPeerGDNative {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PacketPeerGDNative {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PacketPeerGDNative {
    type Target = PacketPeer;

    fn deref(&self) -> &PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PacketPeerGDNative {
    fn deref_mut(&mut self) -> &mut PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PacketPeerGDNative {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PacketPeerGDNative {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PacketPeerGDNative {
    fn construct() -> Self {
        PacketPeerGDNative::new()
    }
}

unsafe impl GodotObject for PacketPeerStream {
    fn class_name() -> &'static str {
        "PacketPeerStream"
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

impl ToVariant for PacketPeerStream {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PacketPeerStream {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PacketPeerStream {
    type Target = PacketPeer;

    fn deref(&self) -> &PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PacketPeerStream {
    fn deref_mut(&mut self) -> &mut PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PacketPeerStream {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PacketPeerStream {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PacketPeerStream {
    fn construct() -> Self {
        PacketPeerStream::new()
    }
}

unsafe impl GodotObject for PacketPeerUDP {
    fn class_name() -> &'static str {
        "PacketPeerUDP"
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

impl ToVariant for PacketPeerUDP {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PacketPeerUDP {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PacketPeerUDP {
    type Target = PacketPeer;

    fn deref(&self) -> &PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PacketPeerUDP {
    fn deref_mut(&mut self) -> &mut PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PacketPeerUDP {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PacketPeerUDP {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PacketPeerUDP {
    fn construct() -> Self {
        PacketPeerUDP::new()
    }
}

unsafe impl GodotObject for Panel {
    fn class_name() -> &'static str {
        "Panel"
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

impl ToVariant for Panel {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Panel {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Panel {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Panel {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Panel {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Panel {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Panel {
    fn construct() -> Self {
        Panel::new()
    }
}

unsafe impl GodotObject for PanelContainer {
    fn class_name() -> &'static str {
        "PanelContainer"
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

impl ToVariant for PanelContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PanelContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for PanelContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for PanelContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PanelContainer {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PanelContainer {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for PanelContainer {
    fn construct() -> Self {
        PanelContainer::new()
    }
}

unsafe impl GodotObject for PanoramaSky {
    fn class_name() -> &'static str {
        "PanoramaSky"
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

impl ToVariant for PanoramaSky {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PanoramaSky {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PanoramaSky {
    type Target = Sky;

    fn deref(&self) -> &Sky {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PanoramaSky {
    fn deref_mut(&mut self) -> &mut Sky {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PanoramaSky {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PanoramaSky {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PanoramaSky {
    fn construct() -> Self {
        PanoramaSky::new()
    }
}

unsafe impl GodotObject for ParallaxBackground {
    fn class_name() -> &'static str {
        "ParallaxBackground"
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

impl ToVariant for ParallaxBackground {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ParallaxBackground {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ParallaxBackground {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ParallaxBackground {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ParallaxBackground {
    type Target = CanvasLayer;

    fn deref(&self) -> &CanvasLayer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ParallaxBackground {
    fn deref_mut(&mut self) -> &mut CanvasLayer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ParallaxBackground {
    fn construct() -> Self {
        ParallaxBackground::new()
    }
}

unsafe impl GodotObject for ParallaxLayer {
    fn class_name() -> &'static str {
        "ParallaxLayer"
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

impl ToVariant for ParallaxLayer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ParallaxLayer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ParallaxLayer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ParallaxLayer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ParallaxLayer {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ParallaxLayer {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ParallaxLayer {
    fn construct() -> Self {
        ParallaxLayer::new()
    }
}

unsafe impl GodotObject for Particles {
    fn class_name() -> &'static str {
        "Particles"
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

impl ToVariant for Particles {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Particles {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Particles {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Particles {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Particles {
    type Target = GeometryInstance;

    fn deref(&self) -> &GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Particles {
    fn deref_mut(&mut self) -> &mut GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Particles {
    fn construct() -> Self {
        Particles::new()
    }
}

unsafe impl GodotObject for Particles2D {
    fn class_name() -> &'static str {
        "Particles2D"
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

impl ToVariant for Particles2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Particles2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Particles2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Particles2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Particles2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Particles2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Particles2D {
    fn construct() -> Self {
        Particles2D::new()
    }
}

unsafe impl GodotObject for ParticlesMaterial {
    fn class_name() -> &'static str {
        "ParticlesMaterial"
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

impl ToVariant for ParticlesMaterial {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ParticlesMaterial {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ParticlesMaterial {
    type Target = Material;

    fn deref(&self) -> &Material {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ParticlesMaterial {
    fn deref_mut(&mut self) -> &mut Material {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ParticlesMaterial {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ParticlesMaterial {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ParticlesMaterial {
    fn construct() -> Self {
        ParticlesMaterial::new()
    }
}

unsafe impl GodotObject for Path {
    fn class_name() -> &'static str {
        "Path"
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

impl ToVariant for Path {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Path {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Path {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Path {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Path {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Path {
    fn construct() -> Self {
        Path::new()
    }
}

unsafe impl GodotObject for Path2D {
    fn class_name() -> &'static str {
        "Path2D"
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

impl ToVariant for Path2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Path2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Path2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Path2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Path2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Path2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Path2D {
    fn construct() -> Self {
        Path2D::new()
    }
}

unsafe impl GodotObject for PathFollow {
    fn class_name() -> &'static str {
        "PathFollow"
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

impl ToVariant for PathFollow {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PathFollow {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for PathFollow {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for PathFollow {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PathFollow {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PathFollow {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for PathFollow {
    fn construct() -> Self {
        PathFollow::new()
    }
}

unsafe impl GodotObject for PathFollow2D {
    fn class_name() -> &'static str {
        "PathFollow2D"
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

impl ToVariant for PathFollow2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PathFollow2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for PathFollow2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for PathFollow2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PathFollow2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PathFollow2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for PathFollow2D {
    fn construct() -> Self {
        PathFollow2D::new()
    }
}

unsafe impl GodotObject for Performance {
    fn class_name() -> &'static str {
        "Performance"
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

impl ToVariant for Performance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Performance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Performance {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Performance {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for PhysicalBone {
    fn class_name() -> &'static str {
        "PhysicalBone"
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

impl ToVariant for PhysicalBone {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PhysicalBone {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for PhysicalBone {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for PhysicalBone {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PhysicalBone {
    type Target = PhysicsBody;

    fn deref(&self) -> &PhysicsBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PhysicalBone {
    fn deref_mut(&mut self) -> &mut PhysicsBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for PhysicalBone {
    fn construct() -> Self {
        PhysicalBone::new()
    }
}

unsafe impl GodotObject for Physics2DDirectBodyState {
    fn class_name() -> &'static str {
        "Physics2DDirectBodyState"
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

impl ToVariant for Physics2DDirectBodyState {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Physics2DDirectBodyState {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Physics2DDirectBodyState {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Physics2DDirectBodyState {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Physics2DDirectBodyStateSW {
    fn class_name() -> &'static str {
        "Physics2DDirectBodyStateSW"
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

impl ToVariant for Physics2DDirectBodyStateSW {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Physics2DDirectBodyStateSW {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Physics2DDirectBodyStateSW {
    type Target = Physics2DDirectBodyState;

    fn deref(&self) -> &Physics2DDirectBodyState {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Physics2DDirectBodyStateSW {
    fn deref_mut(&mut self) -> &mut Physics2DDirectBodyState {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Physics2DDirectSpaceState {
    fn class_name() -> &'static str {
        "Physics2DDirectSpaceState"
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

impl ToVariant for Physics2DDirectSpaceState {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Physics2DDirectSpaceState {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Physics2DDirectSpaceState {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Physics2DDirectSpaceState {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Physics2DServer {
    fn class_name() -> &'static str {
        "Physics2DServer"
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

impl ToVariant for Physics2DServer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Physics2DServer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Physics2DServer {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Physics2DServer {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Physics2DServerSW {
    fn class_name() -> &'static str {
        "Physics2DServerSW"
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

impl ToVariant for Physics2DServerSW {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Physics2DServerSW {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Physics2DServerSW {
    type Target = Physics2DServer;

    fn deref(&self) -> &Physics2DServer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Physics2DServerSW {
    fn deref_mut(&mut self) -> &mut Physics2DServer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Physics2DShapeQueryParameters {
    fn class_name() -> &'static str {
        "Physics2DShapeQueryParameters"
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

impl ToVariant for Physics2DShapeQueryParameters {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Physics2DShapeQueryParameters {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Physics2DShapeQueryParameters {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Physics2DShapeQueryParameters {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Physics2DShapeQueryParameters {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Physics2DShapeQueryParameters {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Physics2DShapeQueryParameters {
    fn construct() -> Self {
        Physics2DShapeQueryParameters::new()
    }
}

unsafe impl GodotObject for Physics2DShapeQueryResult {
    fn class_name() -> &'static str {
        "Physics2DShapeQueryResult"
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

impl ToVariant for Physics2DShapeQueryResult {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Physics2DShapeQueryResult {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Physics2DShapeQueryResult {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Physics2DShapeQueryResult {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Physics2DShapeQueryResult {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Physics2DShapeQueryResult {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for Physics2DTestMotionResult {
    fn class_name() -> &'static str {
        "Physics2DTestMotionResult"
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

impl ToVariant for Physics2DTestMotionResult {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Physics2DTestMotionResult {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Physics2DTestMotionResult {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Physics2DTestMotionResult {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Physics2DTestMotionResult {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Physics2DTestMotionResult {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Physics2DTestMotionResult {
    fn construct() -> Self {
        Physics2DTestMotionResult::new()
    }
}

unsafe impl GodotObject for PhysicsBody {
    fn class_name() -> &'static str {
        "PhysicsBody"
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

impl ToVariant for PhysicsBody {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PhysicsBody {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for PhysicsBody {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PhysicsBody {
    type Target = CollisionObject;

    fn deref(&self) -> &CollisionObject {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PhysicsBody {
    fn deref_mut(&mut self) -> &mut CollisionObject {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for PhysicsBody2D {
    fn class_name() -> &'static str {
        "PhysicsBody2D"
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

impl ToVariant for PhysicsBody2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PhysicsBody2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for PhysicsBody2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PhysicsBody2D {
    type Target = CollisionObject2D;

    fn deref(&self) -> &CollisionObject2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PhysicsBody2D {
    fn deref_mut(&mut self) -> &mut CollisionObject2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for PhysicsDirectBodyState {
    fn class_name() -> &'static str {
        "PhysicsDirectBodyState"
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

impl ToVariant for PhysicsDirectBodyState {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PhysicsDirectBodyState {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PhysicsDirectBodyState {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PhysicsDirectBodyState {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for PhysicsDirectSpaceState {
    fn class_name() -> &'static str {
        "PhysicsDirectSpaceState"
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

impl ToVariant for PhysicsDirectSpaceState {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PhysicsDirectSpaceState {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PhysicsDirectSpaceState {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PhysicsDirectSpaceState {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for PhysicsMaterial {
    fn class_name() -> &'static str {
        "PhysicsMaterial"
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

impl ToVariant for PhysicsMaterial {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PhysicsMaterial {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PhysicsMaterial {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PhysicsMaterial {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PhysicsMaterial {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PhysicsMaterial {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PhysicsMaterial {
    fn construct() -> Self {
        PhysicsMaterial::new()
    }
}

unsafe impl GodotObject for PhysicsServer {
    fn class_name() -> &'static str {
        "PhysicsServer"
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

impl ToVariant for PhysicsServer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PhysicsServer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PhysicsServer {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PhysicsServer {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for PhysicsShapeQueryParameters {
    fn class_name() -> &'static str {
        "PhysicsShapeQueryParameters"
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

impl ToVariant for PhysicsShapeQueryParameters {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PhysicsShapeQueryParameters {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PhysicsShapeQueryParameters {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PhysicsShapeQueryParameters {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PhysicsShapeQueryParameters {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PhysicsShapeQueryParameters {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PhysicsShapeQueryParameters {
    fn construct() -> Self {
        PhysicsShapeQueryParameters::new()
    }
}

unsafe impl GodotObject for PhysicsShapeQueryResult {
    fn class_name() -> &'static str {
        "PhysicsShapeQueryResult"
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

impl ToVariant for PhysicsShapeQueryResult {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PhysicsShapeQueryResult {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PhysicsShapeQueryResult {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PhysicsShapeQueryResult {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PhysicsShapeQueryResult {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PhysicsShapeQueryResult {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for PinJoint {
    fn class_name() -> &'static str {
        "PinJoint"
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

impl ToVariant for PinJoint {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PinJoint {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for PinJoint {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for PinJoint {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PinJoint {
    type Target = Joint;

    fn deref(&self) -> &Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PinJoint {
    fn deref_mut(&mut self) -> &mut Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for PinJoint {
    fn construct() -> Self {
        PinJoint::new()
    }
}

unsafe impl GodotObject for PinJoint2D {
    fn class_name() -> &'static str {
        "PinJoint2D"
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

impl ToVariant for PinJoint2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PinJoint2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for PinJoint2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for PinJoint2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PinJoint2D {
    type Target = Joint2D;

    fn deref(&self) -> &Joint2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PinJoint2D {
    fn deref_mut(&mut self) -> &mut Joint2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for PinJoint2D {
    fn construct() -> Self {
        PinJoint2D::new()
    }
}

unsafe impl GodotObject for PlaneMesh {
    fn class_name() -> &'static str {
        "PlaneMesh"
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

impl ToVariant for PlaneMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PlaneMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PlaneMesh {
    type Target = PrimitiveMesh;

    fn deref(&self) -> &PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PlaneMesh {
    fn deref_mut(&mut self) -> &mut PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PlaneMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PlaneMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PlaneMesh {
    fn construct() -> Self {
        PlaneMesh::new()
    }
}

unsafe impl GodotObject for PlaneShape {
    fn class_name() -> &'static str {
        "PlaneShape"
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

impl ToVariant for PlaneShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PlaneShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PlaneShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PlaneShape {
    fn deref_mut(&mut self) -> &mut Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PlaneShape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PlaneShape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PlaneShape {
    fn construct() -> Self {
        PlaneShape::new()
    }
}

unsafe impl GodotObject for PluginScript {
    fn class_name() -> &'static str {
        "PluginScript"
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

impl ToVariant for PluginScript {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PluginScript {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PluginScript {
    type Target = Script;

    fn deref(&self) -> &Script {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PluginScript {
    fn deref_mut(&mut self) -> &mut Script {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PluginScript {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PluginScript {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PluginScript {
    fn construct() -> Self {
        PluginScript::new()
    }
}

unsafe impl GodotObject for PointMesh {
    fn class_name() -> &'static str {
        "PointMesh"
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

impl ToVariant for PointMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PointMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PointMesh {
    type Target = PrimitiveMesh;

    fn deref(&self) -> &PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PointMesh {
    fn deref_mut(&mut self) -> &mut PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PointMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PointMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PointMesh {
    fn construct() -> Self {
        PointMesh::new()
    }
}

unsafe impl GodotObject for Polygon2D {
    fn class_name() -> &'static str {
        "Polygon2D"
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

impl ToVariant for Polygon2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Polygon2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Polygon2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Polygon2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Polygon2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Polygon2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Polygon2D {
    fn construct() -> Self {
        Polygon2D::new()
    }
}

unsafe impl GodotObject for PolygonPathFinder {
    fn class_name() -> &'static str {
        "PolygonPathFinder"
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

impl ToVariant for PolygonPathFinder {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PolygonPathFinder {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PolygonPathFinder {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PolygonPathFinder {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PolygonPathFinder {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PolygonPathFinder {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PolygonPathFinder {
    fn construct() -> Self {
        PolygonPathFinder::new()
    }
}

unsafe impl GodotObject for Popup {
    fn class_name() -> &'static str {
        "Popup"
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

impl ToVariant for Popup {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Popup {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Popup {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Popup {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Popup {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Popup {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Popup {
    fn construct() -> Self {
        Popup::new()
    }
}

unsafe impl GodotObject for PopupDialog {
    fn class_name() -> &'static str {
        "PopupDialog"
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

impl ToVariant for PopupDialog {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PopupDialog {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for PopupDialog {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for PopupDialog {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PopupDialog {
    type Target = Popup;

    fn deref(&self) -> &Popup {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PopupDialog {
    fn deref_mut(&mut self) -> &mut Popup {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for PopupDialog {
    fn construct() -> Self {
        PopupDialog::new()
    }
}

unsafe impl GodotObject for PopupMenu {
    fn class_name() -> &'static str {
        "PopupMenu"
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

impl ToVariant for PopupMenu {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PopupMenu {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for PopupMenu {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for PopupMenu {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PopupMenu {
    type Target = Popup;

    fn deref(&self) -> &Popup {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PopupMenu {
    fn deref_mut(&mut self) -> &mut Popup {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for PopupMenu {
    fn construct() -> Self {
        PopupMenu::new()
    }
}

unsafe impl GodotObject for PopupPanel {
    fn class_name() -> &'static str {
        "PopupPanel"
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

impl ToVariant for PopupPanel {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PopupPanel {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for PopupPanel {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for PopupPanel {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for PopupPanel {
    type Target = Popup;

    fn deref(&self) -> &Popup {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PopupPanel {
    fn deref_mut(&mut self) -> &mut Popup {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for PopupPanel {
    fn construct() -> Self {
        PopupPanel::new()
    }
}

unsafe impl GodotObject for Position2D {
    fn class_name() -> &'static str {
        "Position2D"
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

impl ToVariant for Position2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Position2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Position2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Position2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Position2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Position2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Position2D {
    fn construct() -> Self {
        Position2D::new()
    }
}

unsafe impl GodotObject for Position3D {
    fn class_name() -> &'static str {
        "Position3D"
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

impl ToVariant for Position3D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Position3D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Position3D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Position3D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Position3D {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Position3D {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Position3D {
    fn construct() -> Self {
        Position3D::new()
    }
}

unsafe impl GodotObject for PrimitiveMesh {
    fn class_name() -> &'static str {
        "PrimitiveMesh"
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

impl ToVariant for PrimitiveMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PrimitiveMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PrimitiveMesh {
    type Target = Mesh;

    fn deref(&self) -> &Mesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PrimitiveMesh {
    fn deref_mut(&mut self) -> &mut Mesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PrimitiveMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PrimitiveMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for PrismMesh {
    fn class_name() -> &'static str {
        "PrismMesh"
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

impl ToVariant for PrismMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for PrismMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for PrismMesh {
    type Target = PrimitiveMesh;

    fn deref(&self) -> &PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for PrismMesh {
    fn deref_mut(&mut self) -> &mut PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for PrismMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for PrismMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for PrismMesh {
    fn construct() -> Self {
        PrismMesh::new()
    }
}

unsafe impl GodotObject for ProceduralSky {
    fn class_name() -> &'static str {
        "ProceduralSky"
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

impl ToVariant for ProceduralSky {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ProceduralSky {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ProceduralSky {
    type Target = Sky;

    fn deref(&self) -> &Sky {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ProceduralSky {
    fn deref_mut(&mut self) -> &mut Sky {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ProceduralSky {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ProceduralSky {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ProceduralSky {
    fn construct() -> Self {
        ProceduralSky::new()
    }
}

unsafe impl GodotObject for ProgressBar {
    fn class_name() -> &'static str {
        "ProgressBar"
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

impl ToVariant for ProgressBar {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ProgressBar {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ProgressBar {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ProgressBar {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ProgressBar {
    type Target = Range;

    fn deref(&self) -> &Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ProgressBar {
    fn deref_mut(&mut self) -> &mut Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ProgressBar {
    fn construct() -> Self {
        ProgressBar::new()
    }
}

unsafe impl GodotObject for ProjectSettings {
    fn class_name() -> &'static str {
        "ProjectSettings"
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

impl ToVariant for ProjectSettings {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ProjectSettings {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ProjectSettings {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ProjectSettings {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for ProximityGroup {
    fn class_name() -> &'static str {
        "ProximityGroup"
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

impl ToVariant for ProximityGroup {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ProximityGroup {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ProximityGroup {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ProximityGroup {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ProximityGroup {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ProximityGroup {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ProximityGroup {
    fn construct() -> Self {
        ProximityGroup::new()
    }
}

unsafe impl GodotObject for ProxyTexture {
    fn class_name() -> &'static str {
        "ProxyTexture"
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

impl ToVariant for ProxyTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ProxyTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ProxyTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ProxyTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ProxyTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ProxyTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ProxyTexture {
    fn construct() -> Self {
        ProxyTexture::new()
    }
}

unsafe impl GodotObject for QuadMesh {
    fn class_name() -> &'static str {
        "QuadMesh"
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

impl ToVariant for QuadMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for QuadMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for QuadMesh {
    type Target = PrimitiveMesh;

    fn deref(&self) -> &PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for QuadMesh {
    fn deref_mut(&mut self) -> &mut PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for QuadMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for QuadMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for QuadMesh {
    fn construct() -> Self {
        QuadMesh::new()
    }
}

unsafe impl GodotObject for RandomNumberGenerator {
    fn class_name() -> &'static str {
        "RandomNumberGenerator"
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

impl ToVariant for RandomNumberGenerator {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RandomNumberGenerator {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for RandomNumberGenerator {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RandomNumberGenerator {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for RandomNumberGenerator {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for RandomNumberGenerator {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for RandomNumberGenerator {
    fn construct() -> Self {
        RandomNumberGenerator::new()
    }
}

unsafe impl GodotObject for Range {
    fn class_name() -> &'static str {
        "Range"
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

impl ToVariant for Range {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Range {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for Range {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Range {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Range {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for RayCast {
    fn class_name() -> &'static str {
        "RayCast"
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

impl ToVariant for RayCast {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RayCast {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for RayCast {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for RayCast {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for RayCast {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RayCast {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for RayCast {
    fn construct() -> Self {
        RayCast::new()
    }
}

unsafe impl GodotObject for RayCast2D {
    fn class_name() -> &'static str {
        "RayCast2D"
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

impl ToVariant for RayCast2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RayCast2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for RayCast2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for RayCast2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for RayCast2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RayCast2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for RayCast2D {
    fn construct() -> Self {
        RayCast2D::new()
    }
}

unsafe impl GodotObject for RayShape {
    fn class_name() -> &'static str {
        "RayShape"
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

impl ToVariant for RayShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RayShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for RayShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RayShape {
    fn deref_mut(&mut self) -> &mut Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for RayShape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for RayShape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for RayShape {
    fn construct() -> Self {
        RayShape::new()
    }
}

unsafe impl GodotObject for RayShape2D {
    fn class_name() -> &'static str {
        "RayShape2D"
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

impl ToVariant for RayShape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RayShape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for RayShape2D {
    type Target = Shape2D;

    fn deref(&self) -> &Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RayShape2D {
    fn deref_mut(&mut self) -> &mut Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for RayShape2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for RayShape2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for RayShape2D {
    fn construct() -> Self {
        RayShape2D::new()
    }
}

unsafe impl GodotObject for RectangleShape2D {
    fn class_name() -> &'static str {
        "RectangleShape2D"
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

impl ToVariant for RectangleShape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RectangleShape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for RectangleShape2D {
    type Target = Shape2D;

    fn deref(&self) -> &Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RectangleShape2D {
    fn deref_mut(&mut self) -> &mut Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for RectangleShape2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for RectangleShape2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for RectangleShape2D {
    fn construct() -> Self {
        RectangleShape2D::new()
    }
}

unsafe impl GodotObject for ReferenceRect {
    fn class_name() -> &'static str {
        "ReferenceRect"
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

impl ToVariant for ReferenceRect {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ReferenceRect {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ReferenceRect {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ReferenceRect {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ReferenceRect {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ReferenceRect {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ReferenceRect {
    fn construct() -> Self {
        ReferenceRect::new()
    }
}

unsafe impl GodotObject for ReflectionProbe {
    fn class_name() -> &'static str {
        "ReflectionProbe"
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

impl ToVariant for ReflectionProbe {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ReflectionProbe {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ReflectionProbe {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ReflectionProbe {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ReflectionProbe {
    type Target = VisualInstance;

    fn deref(&self) -> &VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ReflectionProbe {
    fn deref_mut(&mut self) -> &mut VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ReflectionProbe {
    fn construct() -> Self {
        ReflectionProbe::new()
    }
}

unsafe impl GodotObject for RegEx {
    fn class_name() -> &'static str {
        "RegEx"
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

impl ToVariant for RegEx {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RegEx {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for RegEx {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RegEx {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for RegEx {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for RegEx {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for RegEx {
    fn construct() -> Self {
        RegEx::new()
    }
}

unsafe impl GodotObject for RegExMatch {
    fn class_name() -> &'static str {
        "RegExMatch"
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

impl ToVariant for RegExMatch {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RegExMatch {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for RegExMatch {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RegExMatch {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for RegExMatch {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for RegExMatch {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for RegExMatch {
    fn construct() -> Self {
        RegExMatch::new()
    }
}

unsafe impl GodotObject for RemoteTransform {
    fn class_name() -> &'static str {
        "RemoteTransform"
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

impl ToVariant for RemoteTransform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RemoteTransform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for RemoteTransform {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for RemoteTransform {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for RemoteTransform {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RemoteTransform {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for RemoteTransform {
    fn construct() -> Self {
        RemoteTransform::new()
    }
}

unsafe impl GodotObject for RemoteTransform2D {
    fn class_name() -> &'static str {
        "RemoteTransform2D"
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

impl ToVariant for RemoteTransform2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RemoteTransform2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for RemoteTransform2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for RemoteTransform2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for RemoteTransform2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RemoteTransform2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for RemoteTransform2D {
    fn construct() -> Self {
        RemoteTransform2D::new()
    }
}

unsafe impl GodotObject for Resource {
    fn class_name() -> &'static str {
        "Resource"
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

impl ToVariant for Resource {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Resource {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Resource {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Resource {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Resource {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Resource {
    fn construct() -> Self {
        Resource::new()
    }
}

unsafe impl GodotObject for ResourceFormatLoader {
    fn class_name() -> &'static str {
        "ResourceFormatLoader"
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

impl ToVariant for ResourceFormatLoader {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ResourceFormatLoader {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ResourceFormatLoader {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ResourceFormatLoader {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ResourceFormatLoader {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ResourceFormatLoader {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ResourceFormatLoader {
    fn construct() -> Self {
        ResourceFormatLoader::new()
    }
}

unsafe impl GodotObject for ResourceFormatLoaderCrypto {
    fn class_name() -> &'static str {
        "ResourceFormatLoaderCrypto"
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

impl ToVariant for ResourceFormatLoaderCrypto {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ResourceFormatLoaderCrypto {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ResourceFormatLoaderCrypto {
    type Target = ResourceFormatLoader;

    fn deref(&self) -> &ResourceFormatLoader {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ResourceFormatLoaderCrypto {
    fn deref_mut(&mut self) -> &mut ResourceFormatLoader {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ResourceFormatLoaderCrypto {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ResourceFormatLoaderCrypto {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for ResourceFormatSaver {
    fn class_name() -> &'static str {
        "ResourceFormatSaver"
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

impl ToVariant for ResourceFormatSaver {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ResourceFormatSaver {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ResourceFormatSaver {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ResourceFormatSaver {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ResourceFormatSaver {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ResourceFormatSaver {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ResourceFormatSaver {
    fn construct() -> Self {
        ResourceFormatSaver::new()
    }
}

unsafe impl GodotObject for ResourceFormatSaverCrypto {
    fn class_name() -> &'static str {
        "ResourceFormatSaverCrypto"
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

impl ToVariant for ResourceFormatSaverCrypto {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ResourceFormatSaverCrypto {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ResourceFormatSaverCrypto {
    type Target = ResourceFormatSaver;

    fn deref(&self) -> &ResourceFormatSaver {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ResourceFormatSaverCrypto {
    fn deref_mut(&mut self) -> &mut ResourceFormatSaver {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ResourceFormatSaverCrypto {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ResourceFormatSaverCrypto {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for ResourceImporter {
    fn class_name() -> &'static str {
        "ResourceImporter"
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

impl ToVariant for ResourceImporter {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ResourceImporter {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ResourceImporter {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ResourceImporter {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ResourceImporter {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ResourceImporter {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for ResourceInteractiveLoader {
    fn class_name() -> &'static str {
        "ResourceInteractiveLoader"
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

impl ToVariant for ResourceInteractiveLoader {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ResourceInteractiveLoader {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ResourceInteractiveLoader {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ResourceInteractiveLoader {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ResourceInteractiveLoader {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ResourceInteractiveLoader {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for ResourcePreloader {
    fn class_name() -> &'static str {
        "ResourcePreloader"
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

impl ToVariant for ResourcePreloader {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ResourcePreloader {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ResourcePreloader {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ResourcePreloader {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ResourcePreloader {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ResourcePreloader {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ResourcePreloader {
    fn construct() -> Self {
        ResourcePreloader::new()
    }
}

unsafe impl GodotObject for RichTextEffect {
    fn class_name() -> &'static str {
        "RichTextEffect"
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

impl ToVariant for RichTextEffect {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RichTextEffect {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for RichTextEffect {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RichTextEffect {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for RichTextEffect {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for RichTextEffect {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for RichTextEffect {
    fn construct() -> Self {
        RichTextEffect::new()
    }
}

unsafe impl GodotObject for RichTextLabel {
    fn class_name() -> &'static str {
        "RichTextLabel"
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

impl ToVariant for RichTextLabel {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RichTextLabel {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for RichTextLabel {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for RichTextLabel {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for RichTextLabel {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RichTextLabel {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for RichTextLabel {
    fn construct() -> Self {
        RichTextLabel::new()
    }
}

unsafe impl GodotObject for RigidBody {
    fn class_name() -> &'static str {
        "RigidBody"
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

impl ToVariant for RigidBody {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RigidBody {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for RigidBody {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for RigidBody {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for RigidBody {
    type Target = PhysicsBody;

    fn deref(&self) -> &PhysicsBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RigidBody {
    fn deref_mut(&mut self) -> &mut PhysicsBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for RigidBody {
    fn construct() -> Self {
        RigidBody::new()
    }
}

unsafe impl GodotObject for RigidBody2D {
    fn class_name() -> &'static str {
        "RigidBody2D"
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

impl ToVariant for RigidBody2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RigidBody2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for RigidBody2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for RigidBody2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for RigidBody2D {
    type Target = PhysicsBody2D;

    fn deref(&self) -> &PhysicsBody2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RigidBody2D {
    fn deref_mut(&mut self) -> &mut PhysicsBody2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for RigidBody2D {
    fn construct() -> Self {
        RigidBody2D::new()
    }
}

unsafe impl GodotObject for RootMotionView {
    fn class_name() -> &'static str {
        "RootMotionView"
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

impl ToVariant for RootMotionView {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for RootMotionView {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for RootMotionView {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for RootMotionView {
    type Target = VisualInstance;

    fn deref(&self) -> &VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for RootMotionView {
    fn deref_mut(&mut self) -> &mut VisualInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for SceneState {
    fn class_name() -> &'static str {
        "SceneState"
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

impl ToVariant for SceneState {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SceneState {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SceneState {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SceneState {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SceneState {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SceneState {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for SceneTree {
    fn class_name() -> &'static str {
        "SceneTree"
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

impl ToVariant for SceneTree {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SceneTree {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for SceneTree {
    unsafe fn godot_free(self) { self.free() }
}

impl std::ops::Deref for SceneTree {
    type Target = MainLoop;

    fn deref(&self) -> &MainLoop {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SceneTree {
    fn deref_mut(&mut self) -> &mut MainLoop {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for SceneTree {
    fn construct() -> Self {
        SceneTree::new()
    }
}

unsafe impl GodotObject for SceneTreeTimer {
    fn class_name() -> &'static str {
        "SceneTreeTimer"
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

impl ToVariant for SceneTreeTimer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SceneTreeTimer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SceneTreeTimer {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SceneTreeTimer {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SceneTreeTimer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SceneTreeTimer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for Script {
    fn class_name() -> &'static str {
        "Script"
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

impl ToVariant for Script {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Script {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Script {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Script {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Script {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Script {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for ScriptCreateDialog {
    fn class_name() -> &'static str {
        "ScriptCreateDialog"
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

impl ToVariant for ScriptCreateDialog {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ScriptCreateDialog {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for ScriptCreateDialog {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ScriptCreateDialog {
    type Target = ConfirmationDialog;

    fn deref(&self) -> &ConfirmationDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ScriptCreateDialog {
    fn deref_mut(&mut self) -> &mut ConfirmationDialog {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for ScriptEditor {
    fn class_name() -> &'static str {
        "ScriptEditor"
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

impl ToVariant for ScriptEditor {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ScriptEditor {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for ScriptEditor {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ScriptEditor {
    type Target = PanelContainer;

    fn deref(&self) -> &PanelContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ScriptEditor {
    fn deref_mut(&mut self) -> &mut PanelContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for ScrollBar {
    fn class_name() -> &'static str {
        "ScrollBar"
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

impl ToVariant for ScrollBar {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ScrollBar {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for ScrollBar {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ScrollBar {
    type Target = Range;

    fn deref(&self) -> &Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ScrollBar {
    fn deref_mut(&mut self) -> &mut Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for ScrollContainer {
    fn class_name() -> &'static str {
        "ScrollContainer"
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

impl ToVariant for ScrollContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ScrollContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ScrollContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ScrollContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ScrollContainer {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ScrollContainer {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ScrollContainer {
    fn construct() -> Self {
        ScrollContainer::new()
    }
}

unsafe impl GodotObject for SegmentShape2D {
    fn class_name() -> &'static str {
        "SegmentShape2D"
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

impl ToVariant for SegmentShape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SegmentShape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SegmentShape2D {
    type Target = Shape2D;

    fn deref(&self) -> &Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SegmentShape2D {
    fn deref_mut(&mut self) -> &mut Shape2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SegmentShape2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SegmentShape2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for SegmentShape2D {
    fn construct() -> Self {
        SegmentShape2D::new()
    }
}

unsafe impl GodotObject for Separator {
    fn class_name() -> &'static str {
        "Separator"
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

impl ToVariant for Separator {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Separator {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for Separator {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Separator {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Separator {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Shader {
    fn class_name() -> &'static str {
        "Shader"
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

impl ToVariant for Shader {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Shader {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Shader {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Shader {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Shader {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Shader {
    fn construct() -> Self {
        Shader::new()
    }
}

unsafe impl GodotObject for ShaderMaterial {
    fn class_name() -> &'static str {
        "ShaderMaterial"
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

impl ToVariant for ShaderMaterial {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ShaderMaterial {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ShaderMaterial {
    type Target = Material;

    fn deref(&self) -> &Material {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ShaderMaterial {
    fn deref_mut(&mut self) -> &mut Material {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ShaderMaterial {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ShaderMaterial {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ShaderMaterial {
    fn construct() -> Self {
        ShaderMaterial::new()
    }
}

unsafe impl GodotObject for Shape {
    fn class_name() -> &'static str {
        "Shape"
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

impl ToVariant for Shape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Shape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Shape {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Shape {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Shape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for Shape2D {
    fn class_name() -> &'static str {
        "Shape2D"
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

impl ToVariant for Shape2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Shape2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Shape2D {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Shape2D {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Shape2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Shape2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for ShortCut {
    fn class_name() -> &'static str {
        "ShortCut"
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

impl ToVariant for ShortCut {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ShortCut {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ShortCut {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ShortCut {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ShortCut {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ShortCut {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ShortCut {
    fn construct() -> Self {
        ShortCut::new()
    }
}

unsafe impl GodotObject for Skeleton {
    fn class_name() -> &'static str {
        "Skeleton"
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

impl ToVariant for Skeleton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Skeleton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Skeleton {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Skeleton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Skeleton {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Skeleton {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Skeleton {
    fn construct() -> Self {
        Skeleton::new()
    }
}

unsafe impl GodotObject for Skeleton2D {
    fn class_name() -> &'static str {
        "Skeleton2D"
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

impl ToVariant for Skeleton2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Skeleton2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Skeleton2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Skeleton2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Skeleton2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Skeleton2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Skeleton2D {
    fn construct() -> Self {
        Skeleton2D::new()
    }
}

unsafe impl GodotObject for SkeletonIK {
    fn class_name() -> &'static str {
        "SkeletonIK"
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

impl ToVariant for SkeletonIK {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SkeletonIK {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for SkeletonIK {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for SkeletonIK {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for SkeletonIK {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SkeletonIK {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for SkeletonIK {
    fn construct() -> Self {
        SkeletonIK::new()
    }
}

unsafe impl GodotObject for Skin {
    fn class_name() -> &'static str {
        "Skin"
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

impl ToVariant for Skin {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Skin {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Skin {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Skin {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Skin {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Skin {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Skin {
    fn construct() -> Self {
        Skin::new()
    }
}

unsafe impl GodotObject for SkinReference {
    fn class_name() -> &'static str {
        "SkinReference"
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

impl ToVariant for SkinReference {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SkinReference {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SkinReference {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SkinReference {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SkinReference {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SkinReference {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for Sky {
    fn class_name() -> &'static str {
        "Sky"
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

impl ToVariant for Sky {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Sky {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Sky {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Sky {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Sky {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Sky {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for Slider {
    fn class_name() -> &'static str {
        "Slider"
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

impl ToVariant for Slider {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Slider {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for Slider {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Slider {
    type Target = Range;

    fn deref(&self) -> &Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Slider {
    fn deref_mut(&mut self) -> &mut Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for SliderJoint {
    fn class_name() -> &'static str {
        "SliderJoint"
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

impl ToVariant for SliderJoint {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SliderJoint {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for SliderJoint {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for SliderJoint {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for SliderJoint {
    type Target = Joint;

    fn deref(&self) -> &Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SliderJoint {
    fn deref_mut(&mut self) -> &mut Joint {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for SliderJoint {
    fn construct() -> Self {
        SliderJoint::new()
    }
}

unsafe impl GodotObject for SoftBody {
    fn class_name() -> &'static str {
        "SoftBody"
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

impl ToVariant for SoftBody {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SoftBody {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for SoftBody {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for SoftBody {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for SoftBody {
    type Target = MeshInstance;

    fn deref(&self) -> &MeshInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SoftBody {
    fn deref_mut(&mut self) -> &mut MeshInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for SoftBody {
    fn construct() -> Self {
        SoftBody::new()
    }
}

unsafe impl GodotObject for Spatial {
    fn class_name() -> &'static str {
        "Spatial"
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

impl ToVariant for Spatial {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Spatial {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Spatial {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Spatial {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Spatial {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Spatial {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Spatial {
    fn construct() -> Self {
        Spatial::new()
    }
}

unsafe impl GodotObject for SpatialGizmo {
    fn class_name() -> &'static str {
        "SpatialGizmo"
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

impl ToVariant for SpatialGizmo {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SpatialGizmo {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SpatialGizmo {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SpatialGizmo {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SpatialGizmo {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SpatialGizmo {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for SpatialMaterial {
    fn class_name() -> &'static str {
        "SpatialMaterial"
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

impl ToVariant for SpatialMaterial {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SpatialMaterial {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SpatialMaterial {
    type Target = Material;

    fn deref(&self) -> &Material {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SpatialMaterial {
    fn deref_mut(&mut self) -> &mut Material {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SpatialMaterial {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SpatialMaterial {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for SpatialMaterial {
    fn construct() -> Self {
        SpatialMaterial::new()
    }
}

unsafe impl GodotObject for SpatialVelocityTracker {
    fn class_name() -> &'static str {
        "SpatialVelocityTracker"
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

impl ToVariant for SpatialVelocityTracker {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SpatialVelocityTracker {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SpatialVelocityTracker {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SpatialVelocityTracker {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SpatialVelocityTracker {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SpatialVelocityTracker {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for SpatialVelocityTracker {
    fn construct() -> Self {
        SpatialVelocityTracker::new()
    }
}

unsafe impl GodotObject for SphereMesh {
    fn class_name() -> &'static str {
        "SphereMesh"
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

impl ToVariant for SphereMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SphereMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SphereMesh {
    type Target = PrimitiveMesh;

    fn deref(&self) -> &PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SphereMesh {
    fn deref_mut(&mut self) -> &mut PrimitiveMesh {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SphereMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SphereMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for SphereMesh {
    fn construct() -> Self {
        SphereMesh::new()
    }
}

unsafe impl GodotObject for SphereShape {
    fn class_name() -> &'static str {
        "SphereShape"
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

impl ToVariant for SphereShape {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SphereShape {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SphereShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SphereShape {
    fn deref_mut(&mut self) -> &mut Shape {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SphereShape {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SphereShape {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for SphereShape {
    fn construct() -> Self {
        SphereShape::new()
    }
}

unsafe impl GodotObject for SpinBox {
    fn class_name() -> &'static str {
        "SpinBox"
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

impl ToVariant for SpinBox {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SpinBox {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for SpinBox {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for SpinBox {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for SpinBox {
    type Target = Range;

    fn deref(&self) -> &Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SpinBox {
    fn deref_mut(&mut self) -> &mut Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for SpinBox {
    fn construct() -> Self {
        SpinBox::new()
    }
}

unsafe impl GodotObject for SplitContainer {
    fn class_name() -> &'static str {
        "SplitContainer"
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

impl ToVariant for SplitContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SplitContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for SplitContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for SplitContainer {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SplitContainer {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for SpotLight {
    fn class_name() -> &'static str {
        "SpotLight"
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

impl ToVariant for SpotLight {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SpotLight {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for SpotLight {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for SpotLight {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for SpotLight {
    type Target = Light;

    fn deref(&self) -> &Light {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SpotLight {
    fn deref_mut(&mut self) -> &mut Light {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for SpotLight {
    fn construct() -> Self {
        SpotLight::new()
    }
}

unsafe impl GodotObject for SpringArm {
    fn class_name() -> &'static str {
        "SpringArm"
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

impl ToVariant for SpringArm {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SpringArm {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for SpringArm {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for SpringArm {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for SpringArm {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SpringArm {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for SpringArm {
    fn construct() -> Self {
        SpringArm::new()
    }
}

unsafe impl GodotObject for Sprite {
    fn class_name() -> &'static str {
        "Sprite"
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

impl ToVariant for Sprite {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Sprite {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Sprite {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Sprite {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Sprite {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Sprite {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Sprite {
    fn construct() -> Self {
        Sprite::new()
    }
}

unsafe impl GodotObject for Sprite3D {
    fn class_name() -> &'static str {
        "Sprite3D"
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

impl ToVariant for Sprite3D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Sprite3D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Sprite3D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Sprite3D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Sprite3D {
    type Target = SpriteBase3D;

    fn deref(&self) -> &SpriteBase3D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Sprite3D {
    fn deref_mut(&mut self) -> &mut SpriteBase3D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Sprite3D {
    fn construct() -> Self {
        Sprite3D::new()
    }
}

unsafe impl GodotObject for SpriteBase3D {
    fn class_name() -> &'static str {
        "SpriteBase3D"
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

impl ToVariant for SpriteBase3D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SpriteBase3D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for SpriteBase3D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for SpriteBase3D {
    type Target = GeometryInstance;

    fn deref(&self) -> &GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SpriteBase3D {
    fn deref_mut(&mut self) -> &mut GeometryInstance {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for SpriteFrames {
    fn class_name() -> &'static str {
        "SpriteFrames"
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

impl ToVariant for SpriteFrames {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SpriteFrames {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SpriteFrames {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SpriteFrames {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SpriteFrames {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SpriteFrames {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for SpriteFrames {
    fn construct() -> Self {
        SpriteFrames::new()
    }
}

unsafe impl GodotObject for StaticBody {
    fn class_name() -> &'static str {
        "StaticBody"
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

impl ToVariant for StaticBody {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StaticBody {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for StaticBody {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for StaticBody {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for StaticBody {
    type Target = PhysicsBody;

    fn deref(&self) -> &PhysicsBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StaticBody {
    fn deref_mut(&mut self) -> &mut PhysicsBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for StaticBody {
    fn construct() -> Self {
        StaticBody::new()
    }
}

unsafe impl GodotObject for StaticBody2D {
    fn class_name() -> &'static str {
        "StaticBody2D"
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

impl ToVariant for StaticBody2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StaticBody2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for StaticBody2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for StaticBody2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for StaticBody2D {
    type Target = PhysicsBody2D;

    fn deref(&self) -> &PhysicsBody2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StaticBody2D {
    fn deref_mut(&mut self) -> &mut PhysicsBody2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for StaticBody2D {
    fn construct() -> Self {
        StaticBody2D::new()
    }
}

unsafe impl GodotObject for StreamPeer {
    fn class_name() -> &'static str {
        "StreamPeer"
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

impl ToVariant for StreamPeer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StreamPeer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StreamPeer {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StreamPeer {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StreamPeer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StreamPeer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for StreamPeerBuffer {
    fn class_name() -> &'static str {
        "StreamPeerBuffer"
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

impl ToVariant for StreamPeerBuffer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StreamPeerBuffer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StreamPeerBuffer {
    type Target = StreamPeer;

    fn deref(&self) -> &StreamPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StreamPeerBuffer {
    fn deref_mut(&mut self) -> &mut StreamPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StreamPeerBuffer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StreamPeerBuffer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for StreamPeerBuffer {
    fn construct() -> Self {
        StreamPeerBuffer::new()
    }
}

unsafe impl GodotObject for StreamPeerGDNative {
    fn class_name() -> &'static str {
        "StreamPeerGDNative"
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

impl ToVariant for StreamPeerGDNative {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StreamPeerGDNative {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StreamPeerGDNative {
    type Target = StreamPeer;

    fn deref(&self) -> &StreamPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StreamPeerGDNative {
    fn deref_mut(&mut self) -> &mut StreamPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StreamPeerGDNative {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StreamPeerGDNative {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for StreamPeerGDNative {
    fn construct() -> Self {
        StreamPeerGDNative::new()
    }
}

unsafe impl GodotObject for StreamPeerSSL {
    fn class_name() -> &'static str {
        "StreamPeerSSL"
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

impl ToVariant for StreamPeerSSL {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StreamPeerSSL {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StreamPeerSSL {
    type Target = StreamPeer;

    fn deref(&self) -> &StreamPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StreamPeerSSL {
    fn deref_mut(&mut self) -> &mut StreamPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StreamPeerSSL {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StreamPeerSSL {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for StreamPeerSSL {
    fn construct() -> Self {
        StreamPeerSSL::new()
    }
}

unsafe impl GodotObject for StreamPeerTCP {
    fn class_name() -> &'static str {
        "StreamPeerTCP"
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

impl ToVariant for StreamPeerTCP {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StreamPeerTCP {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StreamPeerTCP {
    type Target = StreamPeer;

    fn deref(&self) -> &StreamPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StreamPeerTCP {
    fn deref_mut(&mut self) -> &mut StreamPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StreamPeerTCP {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StreamPeerTCP {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for StreamPeerTCP {
    fn construct() -> Self {
        StreamPeerTCP::new()
    }
}

unsafe impl GodotObject for StreamTexture {
    fn class_name() -> &'static str {
        "StreamTexture"
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

impl ToVariant for StreamTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StreamTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StreamTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StreamTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StreamTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StreamTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for StreamTexture {
    fn construct() -> Self {
        StreamTexture::new()
    }
}

unsafe impl GodotObject for StyleBox {
    fn class_name() -> &'static str {
        "StyleBox"
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

impl ToVariant for StyleBox {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StyleBox {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StyleBox {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StyleBox {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StyleBox {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StyleBox {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for StyleBoxEmpty {
    fn class_name() -> &'static str {
        "StyleBoxEmpty"
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

impl ToVariant for StyleBoxEmpty {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StyleBoxEmpty {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StyleBoxEmpty {
    type Target = StyleBox;

    fn deref(&self) -> &StyleBox {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StyleBoxEmpty {
    fn deref_mut(&mut self) -> &mut StyleBox {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StyleBoxEmpty {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StyleBoxEmpty {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for StyleBoxEmpty {
    fn construct() -> Self {
        StyleBoxEmpty::new()
    }
}

unsafe impl GodotObject for StyleBoxFlat {
    fn class_name() -> &'static str {
        "StyleBoxFlat"
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

impl ToVariant for StyleBoxFlat {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StyleBoxFlat {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StyleBoxFlat {
    type Target = StyleBox;

    fn deref(&self) -> &StyleBox {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StyleBoxFlat {
    fn deref_mut(&mut self) -> &mut StyleBox {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StyleBoxFlat {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StyleBoxFlat {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for StyleBoxFlat {
    fn construct() -> Self {
        StyleBoxFlat::new()
    }
}

unsafe impl GodotObject for StyleBoxLine {
    fn class_name() -> &'static str {
        "StyleBoxLine"
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

impl ToVariant for StyleBoxLine {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StyleBoxLine {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StyleBoxLine {
    type Target = StyleBox;

    fn deref(&self) -> &StyleBox {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StyleBoxLine {
    fn deref_mut(&mut self) -> &mut StyleBox {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StyleBoxLine {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StyleBoxLine {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for StyleBoxLine {
    fn construct() -> Self {
        StyleBoxLine::new()
    }
}

unsafe impl GodotObject for StyleBoxTexture {
    fn class_name() -> &'static str {
        "StyleBoxTexture"
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

impl ToVariant for StyleBoxTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for StyleBoxTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for StyleBoxTexture {
    type Target = StyleBox;

    fn deref(&self) -> &StyleBox {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for StyleBoxTexture {
    fn deref_mut(&mut self) -> &mut StyleBox {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for StyleBoxTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for StyleBoxTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for StyleBoxTexture {
    fn construct() -> Self {
        StyleBoxTexture::new()
    }
}

unsafe impl GodotObject for SurfaceTool {
    fn class_name() -> &'static str {
        "SurfaceTool"
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

impl ToVariant for SurfaceTool {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for SurfaceTool {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for SurfaceTool {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for SurfaceTool {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for SurfaceTool {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for SurfaceTool {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for SurfaceTool {
    fn construct() -> Self {
        SurfaceTool::new()
    }
}

unsafe impl GodotObject for TCP_Server {
    fn class_name() -> &'static str {
        "TCP_Server"
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

impl ToVariant for TCP_Server {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TCP_Server {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for TCP_Server {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TCP_Server {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for TCP_Server {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for TCP_Server {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for TCP_Server {
    fn construct() -> Self {
        TCP_Server::new()
    }
}

unsafe impl GodotObject for TabContainer {
    fn class_name() -> &'static str {
        "TabContainer"
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

impl ToVariant for TabContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TabContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for TabContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for TabContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for TabContainer {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TabContainer {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for TabContainer {
    fn construct() -> Self {
        TabContainer::new()
    }
}

unsafe impl GodotObject for Tabs {
    fn class_name() -> &'static str {
        "Tabs"
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

impl ToVariant for Tabs {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Tabs {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Tabs {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Tabs {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Tabs {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Tabs {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Tabs {
    fn construct() -> Self {
        Tabs::new()
    }
}

unsafe impl GodotObject for TextEdit {
    fn class_name() -> &'static str {
        "TextEdit"
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

impl ToVariant for TextEdit {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TextEdit {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for TextEdit {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for TextEdit {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for TextEdit {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TextEdit {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for TextEdit {
    fn construct() -> Self {
        TextEdit::new()
    }
}

unsafe impl GodotObject for TextFile {
    fn class_name() -> &'static str {
        "TextFile"
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

impl ToVariant for TextFile {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TextFile {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for TextFile {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TextFile {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for TextFile {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for TextFile {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for TextFile {
    fn construct() -> Self {
        TextFile::new()
    }
}

unsafe impl GodotObject for Texture {
    fn class_name() -> &'static str {
        "Texture"
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

impl ToVariant for Texture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Texture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Texture {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Texture {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for Texture3D {
    fn class_name() -> &'static str {
        "Texture3D"
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

impl ToVariant for Texture3D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Texture3D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Texture3D {
    type Target = TextureLayered;

    fn deref(&self) -> &TextureLayered {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Texture3D {
    fn deref_mut(&mut self) -> &mut TextureLayered {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Texture3D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Texture3D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Texture3D {
    fn construct() -> Self {
        Texture3D::new()
    }
}

unsafe impl GodotObject for TextureArray {
    fn class_name() -> &'static str {
        "TextureArray"
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

impl ToVariant for TextureArray {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TextureArray {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for TextureArray {
    type Target = TextureLayered;

    fn deref(&self) -> &TextureLayered {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TextureArray {
    fn deref_mut(&mut self) -> &mut TextureLayered {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for TextureArray {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for TextureArray {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for TextureArray {
    fn construct() -> Self {
        TextureArray::new()
    }
}

unsafe impl GodotObject for TextureButton {
    fn class_name() -> &'static str {
        "TextureButton"
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

impl ToVariant for TextureButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TextureButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for TextureButton {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for TextureButton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for TextureButton {
    type Target = BaseButton;

    fn deref(&self) -> &BaseButton {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TextureButton {
    fn deref_mut(&mut self) -> &mut BaseButton {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for TextureButton {
    fn construct() -> Self {
        TextureButton::new()
    }
}

unsafe impl GodotObject for TextureLayered {
    fn class_name() -> &'static str {
        "TextureLayered"
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

impl ToVariant for TextureLayered {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TextureLayered {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for TextureLayered {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TextureLayered {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for TextureLayered {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for TextureLayered {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for TextureProgress {
    fn class_name() -> &'static str {
        "TextureProgress"
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

impl ToVariant for TextureProgress {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TextureProgress {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for TextureProgress {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for TextureProgress {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for TextureProgress {
    type Target = Range;

    fn deref(&self) -> &Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TextureProgress {
    fn deref_mut(&mut self) -> &mut Range {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for TextureProgress {
    fn construct() -> Self {
        TextureProgress::new()
    }
}

unsafe impl GodotObject for TextureRect {
    fn class_name() -> &'static str {
        "TextureRect"
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

impl ToVariant for TextureRect {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TextureRect {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for TextureRect {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for TextureRect {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for TextureRect {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TextureRect {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for TextureRect {
    fn construct() -> Self {
        TextureRect::new()
    }
}

unsafe impl GodotObject for Theme {
    fn class_name() -> &'static str {
        "Theme"
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

impl ToVariant for Theme {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Theme {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Theme {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Theme {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Theme {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Theme {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Theme {
    fn construct() -> Self {
        Theme::new()
    }
}

unsafe impl GodotObject for TileMap {
    fn class_name() -> &'static str {
        "TileMap"
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

impl ToVariant for TileMap {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TileMap {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for TileMap {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for TileMap {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for TileMap {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for TileMap {
    fn construct() -> Self {
        TileMap::new()
    }
}

unsafe impl GodotObject for TileSet {
    fn class_name() -> &'static str {
        "TileSet"
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

impl ToVariant for TileSet {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TileSet {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for TileSet {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TileSet {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for TileSet {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for TileSet {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for TileSet {
    fn construct() -> Self {
        TileSet::new()
    }
}

unsafe impl GodotObject for Timer {
    fn class_name() -> &'static str {
        "Timer"
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

impl ToVariant for Timer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Timer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Timer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Timer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Timer {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Timer {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Timer {
    fn construct() -> Self {
        Timer::new()
    }
}

unsafe impl GodotObject for ToolButton {
    fn class_name() -> &'static str {
        "ToolButton"
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

impl ToVariant for ToolButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ToolButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ToolButton {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ToolButton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ToolButton {
    type Target = Button;

    fn deref(&self) -> &Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ToolButton {
    fn deref_mut(&mut self) -> &mut Button {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ToolButton {
    fn construct() -> Self {
        ToolButton::new()
    }
}

unsafe impl GodotObject for TouchScreenButton {
    fn class_name() -> &'static str {
        "TouchScreenButton"
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

impl ToVariant for TouchScreenButton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TouchScreenButton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for TouchScreenButton {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for TouchScreenButton {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for TouchScreenButton {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TouchScreenButton {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for TouchScreenButton {
    fn construct() -> Self {
        TouchScreenButton::new()
    }
}

unsafe impl GodotObject for Translation {
    fn class_name() -> &'static str {
        "Translation"
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

impl ToVariant for Translation {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Translation {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Translation {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Translation {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Translation {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Translation {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Translation {
    fn construct() -> Self {
        Translation::new()
    }
}

unsafe impl GodotObject for TranslationServer {
    fn class_name() -> &'static str {
        "TranslationServer"
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

impl ToVariant for TranslationServer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TranslationServer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for TranslationServer {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TranslationServer {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Tree {
    fn class_name() -> &'static str {
        "Tree"
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

impl ToVariant for Tree {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Tree {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Tree {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Tree {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Tree {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Tree {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Tree {
    fn construct() -> Self {
        Tree::new()
    }
}

unsafe impl GodotObject for TreeItem {
    fn class_name() -> &'static str {
        "TreeItem"
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

impl ToVariant for TreeItem {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TreeItem {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for TreeItem {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TreeItem {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for TriangleMesh {
    fn class_name() -> &'static str {
        "TriangleMesh"
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

impl ToVariant for TriangleMesh {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for TriangleMesh {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for TriangleMesh {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for TriangleMesh {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for TriangleMesh {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for TriangleMesh {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for TriangleMesh {
    fn construct() -> Self {
        TriangleMesh::new()
    }
}

unsafe impl GodotObject for Tween {
    fn class_name() -> &'static str {
        "Tween"
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

impl ToVariant for Tween {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Tween {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Tween {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Tween {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Tween {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Tween {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Tween {
    fn construct() -> Self {
        Tween::new()
    }
}

unsafe impl GodotObject for UPNP {
    fn class_name() -> &'static str {
        "UPNP"
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

impl ToVariant for UPNP {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for UPNP {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for UPNP {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for UPNP {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for UPNP {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for UPNP {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for UPNP {
    fn construct() -> Self {
        UPNP::new()
    }
}

unsafe impl GodotObject for UPNPDevice {
    fn class_name() -> &'static str {
        "UPNPDevice"
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

impl ToVariant for UPNPDevice {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for UPNPDevice {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for UPNPDevice {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for UPNPDevice {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for UPNPDevice {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for UPNPDevice {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for UPNPDevice {
    fn construct() -> Self {
        UPNPDevice::new()
    }
}

unsafe impl GodotObject for UndoRedo {
    fn class_name() -> &'static str {
        "UndoRedo"
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

impl ToVariant for UndoRedo {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for UndoRedo {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for UndoRedo {
    unsafe fn godot_free(self) { self.free() }
}

impl std::ops::Deref for UndoRedo {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for UndoRedo {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for UndoRedo {
    fn construct() -> Self {
        UndoRedo::new()
    }
}

unsafe impl GodotObject for VBoxContainer {
    fn class_name() -> &'static str {
        "VBoxContainer"
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

impl ToVariant for VBoxContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VBoxContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VBoxContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VBoxContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VBoxContainer {
    type Target = BoxContainer;

    fn deref(&self) -> &BoxContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VBoxContainer {
    fn deref_mut(&mut self) -> &mut BoxContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VBoxContainer {
    fn construct() -> Self {
        VBoxContainer::new()
    }
}

unsafe impl GodotObject for VScrollBar {
    fn class_name() -> &'static str {
        "VScrollBar"
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

impl ToVariant for VScrollBar {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VScrollBar {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VScrollBar {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VScrollBar {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VScrollBar {
    type Target = ScrollBar;

    fn deref(&self) -> &ScrollBar {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VScrollBar {
    fn deref_mut(&mut self) -> &mut ScrollBar {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VScrollBar {
    fn construct() -> Self {
        VScrollBar::new()
    }
}

unsafe impl GodotObject for VSeparator {
    fn class_name() -> &'static str {
        "VSeparator"
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

impl ToVariant for VSeparator {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VSeparator {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VSeparator {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VSeparator {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VSeparator {
    type Target = Separator;

    fn deref(&self) -> &Separator {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VSeparator {
    fn deref_mut(&mut self) -> &mut Separator {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VSeparator {
    fn construct() -> Self {
        VSeparator::new()
    }
}

unsafe impl GodotObject for VSlider {
    fn class_name() -> &'static str {
        "VSlider"
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

impl ToVariant for VSlider {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VSlider {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VSlider {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VSlider {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VSlider {
    type Target = Slider;

    fn deref(&self) -> &Slider {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VSlider {
    fn deref_mut(&mut self) -> &mut Slider {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VSlider {
    fn construct() -> Self {
        VSlider::new()
    }
}

unsafe impl GodotObject for VSplitContainer {
    fn class_name() -> &'static str {
        "VSplitContainer"
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

impl ToVariant for VSplitContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VSplitContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VSplitContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VSplitContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VSplitContainer {
    type Target = SplitContainer;

    fn deref(&self) -> &SplitContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VSplitContainer {
    fn deref_mut(&mut self) -> &mut SplitContainer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VSplitContainer {
    fn construct() -> Self {
        VSplitContainer::new()
    }
}

unsafe impl GodotObject for VehicleBody {
    fn class_name() -> &'static str {
        "VehicleBody"
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

impl ToVariant for VehicleBody {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VehicleBody {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VehicleBody {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VehicleBody {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VehicleBody {
    type Target = RigidBody;

    fn deref(&self) -> &RigidBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VehicleBody {
    fn deref_mut(&mut self) -> &mut RigidBody {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VehicleBody {
    fn construct() -> Self {
        VehicleBody::new()
    }
}

unsafe impl GodotObject for VehicleWheel {
    fn class_name() -> &'static str {
        "VehicleWheel"
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

impl ToVariant for VehicleWheel {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VehicleWheel {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VehicleWheel {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VehicleWheel {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VehicleWheel {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VehicleWheel {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VehicleWheel {
    fn construct() -> Self {
        VehicleWheel::new()
    }
}

unsafe impl GodotObject for VideoPlayer {
    fn class_name() -> &'static str {
        "VideoPlayer"
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

impl ToVariant for VideoPlayer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VideoPlayer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VideoPlayer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VideoPlayer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VideoPlayer {
    type Target = Control;

    fn deref(&self) -> &Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VideoPlayer {
    fn deref_mut(&mut self) -> &mut Control {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VideoPlayer {
    fn construct() -> Self {
        VideoPlayer::new()
    }
}

unsafe impl GodotObject for VideoStream {
    fn class_name() -> &'static str {
        "VideoStream"
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

impl ToVariant for VideoStream {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VideoStream {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VideoStream {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VideoStream {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VideoStream {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VideoStream {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for VideoStreamGDNative {
    fn class_name() -> &'static str {
        "VideoStreamGDNative"
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

impl ToVariant for VideoStreamGDNative {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VideoStreamGDNative {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VideoStreamGDNative {
    type Target = VideoStream;

    fn deref(&self) -> &VideoStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VideoStreamGDNative {
    fn deref_mut(&mut self) -> &mut VideoStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VideoStreamGDNative {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VideoStreamGDNative {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VideoStreamGDNative {
    fn construct() -> Self {
        VideoStreamGDNative::new()
    }
}

unsafe impl GodotObject for VideoStreamTheora {
    fn class_name() -> &'static str {
        "VideoStreamTheora"
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

impl ToVariant for VideoStreamTheora {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VideoStreamTheora {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VideoStreamTheora {
    type Target = VideoStream;

    fn deref(&self) -> &VideoStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VideoStreamTheora {
    fn deref_mut(&mut self) -> &mut VideoStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VideoStreamTheora {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VideoStreamTheora {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VideoStreamTheora {
    fn construct() -> Self {
        VideoStreamTheora::new()
    }
}

unsafe impl GodotObject for VideoStreamWebm {
    fn class_name() -> &'static str {
        "VideoStreamWebm"
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

impl ToVariant for VideoStreamWebm {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VideoStreamWebm {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VideoStreamWebm {
    type Target = VideoStream;

    fn deref(&self) -> &VideoStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VideoStreamWebm {
    fn deref_mut(&mut self) -> &mut VideoStream {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VideoStreamWebm {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VideoStreamWebm {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VideoStreamWebm {
    fn construct() -> Self {
        VideoStreamWebm::new()
    }
}

unsafe impl GodotObject for Viewport {
    fn class_name() -> &'static str {
        "Viewport"
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

impl ToVariant for Viewport {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Viewport {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for Viewport {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for Viewport {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for Viewport {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Viewport {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for Viewport {
    fn construct() -> Self {
        Viewport::new()
    }
}

unsafe impl GodotObject for ViewportContainer {
    fn class_name() -> &'static str {
        "ViewportContainer"
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

impl ToVariant for ViewportContainer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ViewportContainer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for ViewportContainer {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for ViewportContainer {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for ViewportContainer {
    type Target = Container;

    fn deref(&self) -> &Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ViewportContainer {
    fn deref_mut(&mut self) -> &mut Container {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for ViewportContainer {
    fn construct() -> Self {
        ViewportContainer::new()
    }
}

unsafe impl GodotObject for ViewportTexture {
    fn class_name() -> &'static str {
        "ViewportTexture"
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

impl ToVariant for ViewportTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ViewportTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ViewportTexture {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ViewportTexture {
    fn deref_mut(&mut self) -> &mut Texture {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for ViewportTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for ViewportTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for ViewportTexture {
    fn construct() -> Self {
        ViewportTexture::new()
    }
}

unsafe impl GodotObject for VisibilityEnabler {
    fn class_name() -> &'static str {
        "VisibilityEnabler"
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

impl ToVariant for VisibilityEnabler {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisibilityEnabler {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VisibilityEnabler {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VisibilityEnabler {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VisibilityEnabler {
    type Target = VisibilityNotifier;

    fn deref(&self) -> &VisibilityNotifier {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisibilityEnabler {
    fn deref_mut(&mut self) -> &mut VisibilityNotifier {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VisibilityEnabler {
    fn construct() -> Self {
        VisibilityEnabler::new()
    }
}

unsafe impl GodotObject for VisibilityEnabler2D {
    fn class_name() -> &'static str {
        "VisibilityEnabler2D"
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

impl ToVariant for VisibilityEnabler2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisibilityEnabler2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VisibilityEnabler2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VisibilityEnabler2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VisibilityEnabler2D {
    type Target = VisibilityNotifier2D;

    fn deref(&self) -> &VisibilityNotifier2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisibilityEnabler2D {
    fn deref_mut(&mut self) -> &mut VisibilityNotifier2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VisibilityEnabler2D {
    fn construct() -> Self {
        VisibilityEnabler2D::new()
    }
}

unsafe impl GodotObject for VisibilityNotifier {
    fn class_name() -> &'static str {
        "VisibilityNotifier"
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

impl ToVariant for VisibilityNotifier {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisibilityNotifier {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VisibilityNotifier {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VisibilityNotifier {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VisibilityNotifier {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisibilityNotifier {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VisibilityNotifier {
    fn construct() -> Self {
        VisibilityNotifier::new()
    }
}

unsafe impl GodotObject for VisibilityNotifier2D {
    fn class_name() -> &'static str {
        "VisibilityNotifier2D"
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

impl ToVariant for VisibilityNotifier2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisibilityNotifier2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for VisibilityNotifier2D {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for VisibilityNotifier2D {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VisibilityNotifier2D {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisibilityNotifier2D {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for VisibilityNotifier2D {
    fn construct() -> Self {
        VisibilityNotifier2D::new()
    }
}

unsafe impl GodotObject for VisualInstance {
    fn class_name() -> &'static str {
        "VisualInstance"
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

impl ToVariant for VisualInstance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualInstance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl QueueFree for VisualInstance {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for VisualInstance {
    type Target = Spatial;

    fn deref(&self) -> &Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualInstance {
    fn deref_mut(&mut self) -> &mut Spatial {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for VisualScript {
    fn class_name() -> &'static str {
        "VisualScript"
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

impl ToVariant for VisualScript {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScript {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScript {
    type Target = Script;

    fn deref(&self) -> &Script {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScript {
    fn deref_mut(&mut self) -> &mut Script {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScript {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScript {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScript {
    fn construct() -> Self {
        VisualScript::new()
    }
}

unsafe impl GodotObject for VisualScriptBasicTypeConstant {
    fn class_name() -> &'static str {
        "VisualScriptBasicTypeConstant"
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

impl ToVariant for VisualScriptBasicTypeConstant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptBasicTypeConstant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptBasicTypeConstant {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptBasicTypeConstant {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptBasicTypeConstant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptBasicTypeConstant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptBasicTypeConstant {
    fn construct() -> Self {
        VisualScriptBasicTypeConstant::new()
    }
}

unsafe impl GodotObject for VisualScriptBuiltinFunc {
    fn class_name() -> &'static str {
        "VisualScriptBuiltinFunc"
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

impl ToVariant for VisualScriptBuiltinFunc {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptBuiltinFunc {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptBuiltinFunc {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptBuiltinFunc {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptBuiltinFunc {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptBuiltinFunc {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptBuiltinFunc {
    fn construct() -> Self {
        VisualScriptBuiltinFunc::new()
    }
}

unsafe impl GodotObject for VisualScriptClassConstant {
    fn class_name() -> &'static str {
        "VisualScriptClassConstant"
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

impl ToVariant for VisualScriptClassConstant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptClassConstant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptClassConstant {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptClassConstant {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptClassConstant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptClassConstant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptClassConstant {
    fn construct() -> Self {
        VisualScriptClassConstant::new()
    }
}

unsafe impl GodotObject for VisualScriptComment {
    fn class_name() -> &'static str {
        "VisualScriptComment"
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

impl ToVariant for VisualScriptComment {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptComment {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptComment {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptComment {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptComment {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptComment {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptComment {
    fn construct() -> Self {
        VisualScriptComment::new()
    }
}

unsafe impl GodotObject for VisualScriptComposeArray {
    fn class_name() -> &'static str {
        "VisualScriptComposeArray"
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

impl ToVariant for VisualScriptComposeArray {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptComposeArray {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptComposeArray {
    type Target = VisualScriptLists;

    fn deref(&self) -> &VisualScriptLists {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptComposeArray {
    fn deref_mut(&mut self) -> &mut VisualScriptLists {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptComposeArray {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptComposeArray {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptComposeArray {
    fn construct() -> Self {
        VisualScriptComposeArray::new()
    }
}

unsafe impl GodotObject for VisualScriptCondition {
    fn class_name() -> &'static str {
        "VisualScriptCondition"
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

impl ToVariant for VisualScriptCondition {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptCondition {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptCondition {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptCondition {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptCondition {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptCondition {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptCondition {
    fn construct() -> Self {
        VisualScriptCondition::new()
    }
}

unsafe impl GodotObject for VisualScriptConstant {
    fn class_name() -> &'static str {
        "VisualScriptConstant"
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

impl ToVariant for VisualScriptConstant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptConstant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptConstant {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptConstant {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptConstant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptConstant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptConstant {
    fn construct() -> Self {
        VisualScriptConstant::new()
    }
}

unsafe impl GodotObject for VisualScriptConstructor {
    fn class_name() -> &'static str {
        "VisualScriptConstructor"
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

impl ToVariant for VisualScriptConstructor {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptConstructor {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptConstructor {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptConstructor {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptConstructor {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptConstructor {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptConstructor {
    fn construct() -> Self {
        VisualScriptConstructor::new()
    }
}

unsafe impl GodotObject for VisualScriptCustomNode {
    fn class_name() -> &'static str {
        "VisualScriptCustomNode"
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

impl ToVariant for VisualScriptCustomNode {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptCustomNode {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptCustomNode {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptCustomNode {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptCustomNode {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptCustomNode {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptCustomNode {
    fn construct() -> Self {
        VisualScriptCustomNode::new()
    }
}

unsafe impl GodotObject for VisualScriptDeconstruct {
    fn class_name() -> &'static str {
        "VisualScriptDeconstruct"
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

impl ToVariant for VisualScriptDeconstruct {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptDeconstruct {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptDeconstruct {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptDeconstruct {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptDeconstruct {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptDeconstruct {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptDeconstruct {
    fn construct() -> Self {
        VisualScriptDeconstruct::new()
    }
}

unsafe impl GodotObject for VisualScriptEmitSignal {
    fn class_name() -> &'static str {
        "VisualScriptEmitSignal"
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

impl ToVariant for VisualScriptEmitSignal {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptEmitSignal {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptEmitSignal {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptEmitSignal {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptEmitSignal {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptEmitSignal {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptEmitSignal {
    fn construct() -> Self {
        VisualScriptEmitSignal::new()
    }
}

unsafe impl GodotObject for VisualScriptEngineSingleton {
    fn class_name() -> &'static str {
        "VisualScriptEngineSingleton"
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

impl ToVariant for VisualScriptEngineSingleton {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptEngineSingleton {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptEngineSingleton {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptEngineSingleton {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptEngineSingleton {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptEngineSingleton {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptEngineSingleton {
    fn construct() -> Self {
        VisualScriptEngineSingleton::new()
    }
}

unsafe impl GodotObject for VisualScriptExpression {
    fn class_name() -> &'static str {
        "VisualScriptExpression"
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

impl ToVariant for VisualScriptExpression {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptExpression {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptExpression {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptExpression {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptExpression {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptExpression {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptExpression {
    fn construct() -> Self {
        VisualScriptExpression::new()
    }
}

unsafe impl GodotObject for VisualScriptFunction {
    fn class_name() -> &'static str {
        "VisualScriptFunction"
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

impl ToVariant for VisualScriptFunction {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptFunction {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptFunction {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptFunction {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptFunction {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptFunction {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptFunction {
    fn construct() -> Self {
        VisualScriptFunction::new()
    }
}

unsafe impl GodotObject for VisualScriptFunctionCall {
    fn class_name() -> &'static str {
        "VisualScriptFunctionCall"
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

impl ToVariant for VisualScriptFunctionCall {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptFunctionCall {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptFunctionCall {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptFunctionCall {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptFunctionCall {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptFunctionCall {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptFunctionCall {
    fn construct() -> Self {
        VisualScriptFunctionCall::new()
    }
}

unsafe impl GodotObject for VisualScriptFunctionState {
    fn class_name() -> &'static str {
        "VisualScriptFunctionState"
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

impl ToVariant for VisualScriptFunctionState {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptFunctionState {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptFunctionState {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptFunctionState {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptFunctionState {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptFunctionState {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptFunctionState {
    fn construct() -> Self {
        VisualScriptFunctionState::new()
    }
}

unsafe impl GodotObject for VisualScriptGlobalConstant {
    fn class_name() -> &'static str {
        "VisualScriptGlobalConstant"
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

impl ToVariant for VisualScriptGlobalConstant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptGlobalConstant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptGlobalConstant {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptGlobalConstant {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptGlobalConstant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptGlobalConstant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptGlobalConstant {
    fn construct() -> Self {
        VisualScriptGlobalConstant::new()
    }
}

unsafe impl GodotObject for VisualScriptIndexGet {
    fn class_name() -> &'static str {
        "VisualScriptIndexGet"
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

impl ToVariant for VisualScriptIndexGet {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptIndexGet {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptIndexGet {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptIndexGet {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptIndexGet {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptIndexGet {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptIndexGet {
    fn construct() -> Self {
        VisualScriptIndexGet::new()
    }
}

unsafe impl GodotObject for VisualScriptIndexSet {
    fn class_name() -> &'static str {
        "VisualScriptIndexSet"
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

impl ToVariant for VisualScriptIndexSet {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptIndexSet {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptIndexSet {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptIndexSet {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptIndexSet {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptIndexSet {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptIndexSet {
    fn construct() -> Self {
        VisualScriptIndexSet::new()
    }
}

unsafe impl GodotObject for VisualScriptInputAction {
    fn class_name() -> &'static str {
        "VisualScriptInputAction"
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

impl ToVariant for VisualScriptInputAction {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptInputAction {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptInputAction {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptInputAction {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptInputAction {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptInputAction {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptInputAction {
    fn construct() -> Self {
        VisualScriptInputAction::new()
    }
}

unsafe impl GodotObject for VisualScriptIterator {
    fn class_name() -> &'static str {
        "VisualScriptIterator"
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

impl ToVariant for VisualScriptIterator {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptIterator {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptIterator {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptIterator {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptIterator {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptIterator {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptIterator {
    fn construct() -> Self {
        VisualScriptIterator::new()
    }
}

unsafe impl GodotObject for VisualScriptLists {
    fn class_name() -> &'static str {
        "VisualScriptLists"
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

impl ToVariant for VisualScriptLists {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptLists {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptLists {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptLists {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptLists {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptLists {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for VisualScriptLocalVar {
    fn class_name() -> &'static str {
        "VisualScriptLocalVar"
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

impl ToVariant for VisualScriptLocalVar {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptLocalVar {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptLocalVar {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptLocalVar {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptLocalVar {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptLocalVar {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptLocalVar {
    fn construct() -> Self {
        VisualScriptLocalVar::new()
    }
}

unsafe impl GodotObject for VisualScriptLocalVarSet {
    fn class_name() -> &'static str {
        "VisualScriptLocalVarSet"
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

impl ToVariant for VisualScriptLocalVarSet {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptLocalVarSet {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptLocalVarSet {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptLocalVarSet {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptLocalVarSet {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptLocalVarSet {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptLocalVarSet {
    fn construct() -> Self {
        VisualScriptLocalVarSet::new()
    }
}

unsafe impl GodotObject for VisualScriptMathConstant {
    fn class_name() -> &'static str {
        "VisualScriptMathConstant"
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

impl ToVariant for VisualScriptMathConstant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptMathConstant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptMathConstant {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptMathConstant {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptMathConstant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptMathConstant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptMathConstant {
    fn construct() -> Self {
        VisualScriptMathConstant::new()
    }
}

unsafe impl GodotObject for VisualScriptNode {
    fn class_name() -> &'static str {
        "VisualScriptNode"
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

impl ToVariant for VisualScriptNode {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptNode {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptNode {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptNode {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptNode {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptNode {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for VisualScriptOperator {
    fn class_name() -> &'static str {
        "VisualScriptOperator"
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

impl ToVariant for VisualScriptOperator {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptOperator {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptOperator {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptOperator {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptOperator {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptOperator {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptOperator {
    fn construct() -> Self {
        VisualScriptOperator::new()
    }
}

unsafe impl GodotObject for VisualScriptPreload {
    fn class_name() -> &'static str {
        "VisualScriptPreload"
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

impl ToVariant for VisualScriptPreload {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptPreload {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptPreload {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptPreload {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptPreload {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptPreload {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptPreload {
    fn construct() -> Self {
        VisualScriptPreload::new()
    }
}

unsafe impl GodotObject for VisualScriptPropertyGet {
    fn class_name() -> &'static str {
        "VisualScriptPropertyGet"
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

impl ToVariant for VisualScriptPropertyGet {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptPropertyGet {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptPropertyGet {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptPropertyGet {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptPropertyGet {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptPropertyGet {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptPropertyGet {
    fn construct() -> Self {
        VisualScriptPropertyGet::new()
    }
}

unsafe impl GodotObject for VisualScriptPropertySet {
    fn class_name() -> &'static str {
        "VisualScriptPropertySet"
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

impl ToVariant for VisualScriptPropertySet {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptPropertySet {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptPropertySet {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptPropertySet {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptPropertySet {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptPropertySet {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptPropertySet {
    fn construct() -> Self {
        VisualScriptPropertySet::new()
    }
}

unsafe impl GodotObject for VisualScriptResourcePath {
    fn class_name() -> &'static str {
        "VisualScriptResourcePath"
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

impl ToVariant for VisualScriptResourcePath {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptResourcePath {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptResourcePath {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptResourcePath {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptResourcePath {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptResourcePath {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptResourcePath {
    fn construct() -> Self {
        VisualScriptResourcePath::new()
    }
}

unsafe impl GodotObject for VisualScriptReturn {
    fn class_name() -> &'static str {
        "VisualScriptReturn"
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

impl ToVariant for VisualScriptReturn {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptReturn {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptReturn {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptReturn {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptReturn {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptReturn {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptReturn {
    fn construct() -> Self {
        VisualScriptReturn::new()
    }
}

unsafe impl GodotObject for VisualScriptSceneNode {
    fn class_name() -> &'static str {
        "VisualScriptSceneNode"
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

impl ToVariant for VisualScriptSceneNode {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptSceneNode {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptSceneNode {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptSceneNode {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptSceneNode {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptSceneNode {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptSceneNode {
    fn construct() -> Self {
        VisualScriptSceneNode::new()
    }
}

unsafe impl GodotObject for VisualScriptSceneTree {
    fn class_name() -> &'static str {
        "VisualScriptSceneTree"
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

impl ToVariant for VisualScriptSceneTree {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptSceneTree {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptSceneTree {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptSceneTree {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptSceneTree {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptSceneTree {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptSceneTree {
    fn construct() -> Self {
        VisualScriptSceneTree::new()
    }
}

unsafe impl GodotObject for VisualScriptSelect {
    fn class_name() -> &'static str {
        "VisualScriptSelect"
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

impl ToVariant for VisualScriptSelect {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptSelect {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptSelect {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptSelect {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptSelect {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptSelect {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptSelect {
    fn construct() -> Self {
        VisualScriptSelect::new()
    }
}

unsafe impl GodotObject for VisualScriptSelf {
    fn class_name() -> &'static str {
        "VisualScriptSelf"
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

impl ToVariant for VisualScriptSelf {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptSelf {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptSelf {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptSelf {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptSelf {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptSelf {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptSelf {
    fn construct() -> Self {
        VisualScriptSelf::new()
    }
}

unsafe impl GodotObject for VisualScriptSequence {
    fn class_name() -> &'static str {
        "VisualScriptSequence"
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

impl ToVariant for VisualScriptSequence {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptSequence {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptSequence {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptSequence {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptSequence {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptSequence {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptSequence {
    fn construct() -> Self {
        VisualScriptSequence::new()
    }
}

unsafe impl GodotObject for VisualScriptSubCall {
    fn class_name() -> &'static str {
        "VisualScriptSubCall"
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

impl ToVariant for VisualScriptSubCall {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptSubCall {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptSubCall {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptSubCall {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptSubCall {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptSubCall {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptSubCall {
    fn construct() -> Self {
        VisualScriptSubCall::new()
    }
}

unsafe impl GodotObject for VisualScriptSwitch {
    fn class_name() -> &'static str {
        "VisualScriptSwitch"
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

impl ToVariant for VisualScriptSwitch {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptSwitch {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptSwitch {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptSwitch {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptSwitch {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptSwitch {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptSwitch {
    fn construct() -> Self {
        VisualScriptSwitch::new()
    }
}

unsafe impl GodotObject for VisualScriptTypeCast {
    fn class_name() -> &'static str {
        "VisualScriptTypeCast"
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

impl ToVariant for VisualScriptTypeCast {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptTypeCast {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptTypeCast {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptTypeCast {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptTypeCast {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptTypeCast {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptTypeCast {
    fn construct() -> Self {
        VisualScriptTypeCast::new()
    }
}

unsafe impl GodotObject for VisualScriptVariableGet {
    fn class_name() -> &'static str {
        "VisualScriptVariableGet"
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

impl ToVariant for VisualScriptVariableGet {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptVariableGet {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptVariableGet {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptVariableGet {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptVariableGet {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptVariableGet {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptVariableGet {
    fn construct() -> Self {
        VisualScriptVariableGet::new()
    }
}

unsafe impl GodotObject for VisualScriptVariableSet {
    fn class_name() -> &'static str {
        "VisualScriptVariableSet"
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

impl ToVariant for VisualScriptVariableSet {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptVariableSet {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptVariableSet {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptVariableSet {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptVariableSet {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptVariableSet {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptVariableSet {
    fn construct() -> Self {
        VisualScriptVariableSet::new()
    }
}

unsafe impl GodotObject for VisualScriptWhile {
    fn class_name() -> &'static str {
        "VisualScriptWhile"
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

impl ToVariant for VisualScriptWhile {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptWhile {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptWhile {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptWhile {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptWhile {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptWhile {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptWhile {
    fn construct() -> Self {
        VisualScriptWhile::new()
    }
}

unsafe impl GodotObject for VisualScriptYield {
    fn class_name() -> &'static str {
        "VisualScriptYield"
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

impl ToVariant for VisualScriptYield {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptYield {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptYield {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptYield {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptYield {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptYield {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptYield {
    fn construct() -> Self {
        VisualScriptYield::new()
    }
}

unsafe impl GodotObject for VisualScriptYieldSignal {
    fn class_name() -> &'static str {
        "VisualScriptYieldSignal"
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

impl ToVariant for VisualScriptYieldSignal {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptYieldSignal {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptYieldSignal {
    type Target = VisualScriptNode;

    fn deref(&self) -> &VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptYieldSignal {
    fn deref_mut(&mut self) -> &mut VisualScriptNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualScriptYieldSignal {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualScriptYieldSignal {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualScriptYieldSignal {
    fn construct() -> Self {
        VisualScriptYieldSignal::new()
    }
}

unsafe impl GodotObject for VisualServer {
    fn class_name() -> &'static str {
        "VisualServer"
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

impl ToVariant for VisualServer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualServer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualServer {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualServer {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for VisualShader {
    fn class_name() -> &'static str {
        "VisualShader"
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

impl ToVariant for VisualShader {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShader {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShader {
    type Target = Shader;

    fn deref(&self) -> &Shader {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShader {
    fn deref_mut(&mut self) -> &mut Shader {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShader {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShader {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShader {
    fn construct() -> Self {
        VisualShader::new()
    }
}

unsafe impl GodotObject for VisualShaderNode {
    fn class_name() -> &'static str {
        "VisualShaderNode"
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

impl ToVariant for VisualShaderNode {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNode {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNode {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNode {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNode {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNode {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for VisualShaderNodeBooleanConstant {
    fn class_name() -> &'static str {
        "VisualShaderNodeBooleanConstant"
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

impl ToVariant for VisualShaderNodeBooleanConstant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeBooleanConstant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeBooleanConstant {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeBooleanConstant {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeBooleanConstant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeBooleanConstant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeBooleanConstant {
    fn construct() -> Self {
        VisualShaderNodeBooleanConstant::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeBooleanUniform {
    fn class_name() -> &'static str {
        "VisualShaderNodeBooleanUniform"
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

impl ToVariant for VisualShaderNodeBooleanUniform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeBooleanUniform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeBooleanUniform {
    type Target = VisualShaderNodeUniform;

    fn deref(&self) -> &VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeBooleanUniform {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeBooleanUniform {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeBooleanUniform {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeBooleanUniform {
    fn construct() -> Self {
        VisualShaderNodeBooleanUniform::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeColorConstant {
    fn class_name() -> &'static str {
        "VisualShaderNodeColorConstant"
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

impl ToVariant for VisualShaderNodeColorConstant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeColorConstant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeColorConstant {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeColorConstant {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeColorConstant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeColorConstant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeColorConstant {
    fn construct() -> Self {
        VisualShaderNodeColorConstant::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeColorFunc {
    fn class_name() -> &'static str {
        "VisualShaderNodeColorFunc"
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

impl ToVariant for VisualShaderNodeColorFunc {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeColorFunc {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeColorFunc {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeColorFunc {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeColorFunc {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeColorFunc {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeColorFunc {
    fn construct() -> Self {
        VisualShaderNodeColorFunc::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeColorOp {
    fn class_name() -> &'static str {
        "VisualShaderNodeColorOp"
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

impl ToVariant for VisualShaderNodeColorOp {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeColorOp {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeColorOp {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeColorOp {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeColorOp {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeColorOp {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeColorOp {
    fn construct() -> Self {
        VisualShaderNodeColorOp::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeColorUniform {
    fn class_name() -> &'static str {
        "VisualShaderNodeColorUniform"
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

impl ToVariant for VisualShaderNodeColorUniform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeColorUniform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeColorUniform {
    type Target = VisualShaderNodeUniform;

    fn deref(&self) -> &VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeColorUniform {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeColorUniform {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeColorUniform {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeColorUniform {
    fn construct() -> Self {
        VisualShaderNodeColorUniform::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeCompare {
    fn class_name() -> &'static str {
        "VisualShaderNodeCompare"
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

impl ToVariant for VisualShaderNodeCompare {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeCompare {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeCompare {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeCompare {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeCompare {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeCompare {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeCompare {
    fn construct() -> Self {
        VisualShaderNodeCompare::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeCubeMap {
    fn class_name() -> &'static str {
        "VisualShaderNodeCubeMap"
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

impl ToVariant for VisualShaderNodeCubeMap {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeCubeMap {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeCubeMap {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeCubeMap {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeCubeMap {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeCubeMap {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeCubeMap {
    fn construct() -> Self {
        VisualShaderNodeCubeMap::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeCubeMapUniform {
    fn class_name() -> &'static str {
        "VisualShaderNodeCubeMapUniform"
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

impl ToVariant for VisualShaderNodeCubeMapUniform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeCubeMapUniform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeCubeMapUniform {
    type Target = VisualShaderNodeTextureUniform;

    fn deref(&self) -> &VisualShaderNodeTextureUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeCubeMapUniform {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeTextureUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeCubeMapUniform {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeCubeMapUniform {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeCubeMapUniform {
    fn construct() -> Self {
        VisualShaderNodeCubeMapUniform::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeCustom {
    fn class_name() -> &'static str {
        "VisualShaderNodeCustom"
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

impl ToVariant for VisualShaderNodeCustom {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeCustom {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeCustom {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeCustom {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeCustom {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeCustom {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeCustom {
    fn construct() -> Self {
        VisualShaderNodeCustom::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeDeterminant {
    fn class_name() -> &'static str {
        "VisualShaderNodeDeterminant"
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

impl ToVariant for VisualShaderNodeDeterminant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeDeterminant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeDeterminant {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeDeterminant {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeDeterminant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeDeterminant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeDeterminant {
    fn construct() -> Self {
        VisualShaderNodeDeterminant::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeDotProduct {
    fn class_name() -> &'static str {
        "VisualShaderNodeDotProduct"
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

impl ToVariant for VisualShaderNodeDotProduct {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeDotProduct {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeDotProduct {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeDotProduct {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeDotProduct {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeDotProduct {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeDotProduct {
    fn construct() -> Self {
        VisualShaderNodeDotProduct::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeExpression {
    fn class_name() -> &'static str {
        "VisualShaderNodeExpression"
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

impl ToVariant for VisualShaderNodeExpression {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeExpression {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeExpression {
    type Target = VisualShaderNodeGroupBase;

    fn deref(&self) -> &VisualShaderNodeGroupBase {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeExpression {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeGroupBase {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeExpression {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeExpression {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeExpression {
    fn construct() -> Self {
        VisualShaderNodeExpression::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeFaceForward {
    fn class_name() -> &'static str {
        "VisualShaderNodeFaceForward"
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

impl ToVariant for VisualShaderNodeFaceForward {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeFaceForward {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeFaceForward {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeFaceForward {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeFaceForward {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeFaceForward {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeFaceForward {
    fn construct() -> Self {
        VisualShaderNodeFaceForward::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeFresnel {
    fn class_name() -> &'static str {
        "VisualShaderNodeFresnel"
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

impl ToVariant for VisualShaderNodeFresnel {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeFresnel {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeFresnel {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeFresnel {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeFresnel {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeFresnel {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeFresnel {
    fn construct() -> Self {
        VisualShaderNodeFresnel::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeGlobalExpression {
    fn class_name() -> &'static str {
        "VisualShaderNodeGlobalExpression"
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

impl ToVariant for VisualShaderNodeGlobalExpression {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeGlobalExpression {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeGlobalExpression {
    type Target = VisualShaderNodeExpression;

    fn deref(&self) -> &VisualShaderNodeExpression {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeGlobalExpression {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeExpression {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeGlobalExpression {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeGlobalExpression {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeGlobalExpression {
    fn construct() -> Self {
        VisualShaderNodeGlobalExpression::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeGroupBase {
    fn class_name() -> &'static str {
        "VisualShaderNodeGroupBase"
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

impl ToVariant for VisualShaderNodeGroupBase {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeGroupBase {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeGroupBase {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeGroupBase {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeGroupBase {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeGroupBase {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeGroupBase {
    fn construct() -> Self {
        VisualShaderNodeGroupBase::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeIf {
    fn class_name() -> &'static str {
        "VisualShaderNodeIf"
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

impl ToVariant for VisualShaderNodeIf {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeIf {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeIf {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeIf {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeIf {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeIf {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeIf {
    fn construct() -> Self {
        VisualShaderNodeIf::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeInput {
    fn class_name() -> &'static str {
        "VisualShaderNodeInput"
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

impl ToVariant for VisualShaderNodeInput {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeInput {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeInput {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeInput {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeInput {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeInput {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeInput {
    fn construct() -> Self {
        VisualShaderNodeInput::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeIs {
    fn class_name() -> &'static str {
        "VisualShaderNodeIs"
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

impl ToVariant for VisualShaderNodeIs {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeIs {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeIs {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeIs {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeIs {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeIs {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeIs {
    fn construct() -> Self {
        VisualShaderNodeIs::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeOuterProduct {
    fn class_name() -> &'static str {
        "VisualShaderNodeOuterProduct"
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

impl ToVariant for VisualShaderNodeOuterProduct {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeOuterProduct {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeOuterProduct {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeOuterProduct {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeOuterProduct {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeOuterProduct {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeOuterProduct {
    fn construct() -> Self {
        VisualShaderNodeOuterProduct::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeOutput {
    fn class_name() -> &'static str {
        "VisualShaderNodeOutput"
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

impl ToVariant for VisualShaderNodeOutput {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeOutput {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeOutput {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeOutput {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeOutput {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeOutput {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for VisualShaderNodeScalarClamp {
    fn class_name() -> &'static str {
        "VisualShaderNodeScalarClamp"
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

impl ToVariant for VisualShaderNodeScalarClamp {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeScalarClamp {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeScalarClamp {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeScalarClamp {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeScalarClamp {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeScalarClamp {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeScalarClamp {
    fn construct() -> Self {
        VisualShaderNodeScalarClamp::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeScalarConstant {
    fn class_name() -> &'static str {
        "VisualShaderNodeScalarConstant"
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

impl ToVariant for VisualShaderNodeScalarConstant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeScalarConstant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeScalarConstant {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeScalarConstant {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeScalarConstant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeScalarConstant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeScalarConstant {
    fn construct() -> Self {
        VisualShaderNodeScalarConstant::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeScalarDerivativeFunc {
    fn class_name() -> &'static str {
        "VisualShaderNodeScalarDerivativeFunc"
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

impl ToVariant for VisualShaderNodeScalarDerivativeFunc {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeScalarDerivativeFunc {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeScalarDerivativeFunc {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeScalarDerivativeFunc {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeScalarDerivativeFunc {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeScalarDerivativeFunc {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeScalarDerivativeFunc {
    fn construct() -> Self {
        VisualShaderNodeScalarDerivativeFunc::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeScalarFunc {
    fn class_name() -> &'static str {
        "VisualShaderNodeScalarFunc"
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

impl ToVariant for VisualShaderNodeScalarFunc {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeScalarFunc {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeScalarFunc {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeScalarFunc {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeScalarFunc {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeScalarFunc {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeScalarFunc {
    fn construct() -> Self {
        VisualShaderNodeScalarFunc::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeScalarInterp {
    fn class_name() -> &'static str {
        "VisualShaderNodeScalarInterp"
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

impl ToVariant for VisualShaderNodeScalarInterp {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeScalarInterp {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeScalarInterp {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeScalarInterp {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeScalarInterp {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeScalarInterp {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeScalarInterp {
    fn construct() -> Self {
        VisualShaderNodeScalarInterp::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeScalarOp {
    fn class_name() -> &'static str {
        "VisualShaderNodeScalarOp"
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

impl ToVariant for VisualShaderNodeScalarOp {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeScalarOp {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeScalarOp {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeScalarOp {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeScalarOp {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeScalarOp {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeScalarOp {
    fn construct() -> Self {
        VisualShaderNodeScalarOp::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeScalarSmoothStep {
    fn class_name() -> &'static str {
        "VisualShaderNodeScalarSmoothStep"
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

impl ToVariant for VisualShaderNodeScalarSmoothStep {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeScalarSmoothStep {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeScalarSmoothStep {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeScalarSmoothStep {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeScalarSmoothStep {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeScalarSmoothStep {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeScalarSmoothStep {
    fn construct() -> Self {
        VisualShaderNodeScalarSmoothStep::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeScalarSwitch {
    fn class_name() -> &'static str {
        "VisualShaderNodeScalarSwitch"
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

impl ToVariant for VisualShaderNodeScalarSwitch {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeScalarSwitch {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeScalarSwitch {
    type Target = VisualShaderNodeSwitch;

    fn deref(&self) -> &VisualShaderNodeSwitch {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeScalarSwitch {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeSwitch {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeScalarSwitch {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeScalarSwitch {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeScalarSwitch {
    fn construct() -> Self {
        VisualShaderNodeScalarSwitch::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeScalarUniform {
    fn class_name() -> &'static str {
        "VisualShaderNodeScalarUniform"
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

impl ToVariant for VisualShaderNodeScalarUniform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeScalarUniform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeScalarUniform {
    type Target = VisualShaderNodeUniform;

    fn deref(&self) -> &VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeScalarUniform {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeScalarUniform {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeScalarUniform {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeScalarUniform {
    fn construct() -> Self {
        VisualShaderNodeScalarUniform::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeSwitch {
    fn class_name() -> &'static str {
        "VisualShaderNodeSwitch"
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

impl ToVariant for VisualShaderNodeSwitch {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeSwitch {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeSwitch {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeSwitch {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeSwitch {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeSwitch {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeSwitch {
    fn construct() -> Self {
        VisualShaderNodeSwitch::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTexture {
    fn class_name() -> &'static str {
        "VisualShaderNodeTexture"
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

impl ToVariant for VisualShaderNodeTexture {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTexture {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTexture {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTexture {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTexture {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTexture {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTexture {
    fn construct() -> Self {
        VisualShaderNodeTexture::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTextureUniform {
    fn class_name() -> &'static str {
        "VisualShaderNodeTextureUniform"
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

impl ToVariant for VisualShaderNodeTextureUniform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTextureUniform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTextureUniform {
    type Target = VisualShaderNodeUniform;

    fn deref(&self) -> &VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTextureUniform {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTextureUniform {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTextureUniform {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTextureUniform {
    fn construct() -> Self {
        VisualShaderNodeTextureUniform::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTextureUniformTriplanar {
    fn class_name() -> &'static str {
        "VisualShaderNodeTextureUniformTriplanar"
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

impl ToVariant for VisualShaderNodeTextureUniformTriplanar {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTextureUniformTriplanar {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTextureUniformTriplanar {
    type Target = VisualShaderNodeTextureUniform;

    fn deref(&self) -> &VisualShaderNodeTextureUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTextureUniformTriplanar {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeTextureUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTextureUniformTriplanar {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTextureUniformTriplanar {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTextureUniformTriplanar {
    fn construct() -> Self {
        VisualShaderNodeTextureUniformTriplanar::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTransformCompose {
    fn class_name() -> &'static str {
        "VisualShaderNodeTransformCompose"
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

impl ToVariant for VisualShaderNodeTransformCompose {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTransformCompose {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTransformCompose {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTransformCompose {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTransformCompose {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTransformCompose {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTransformCompose {
    fn construct() -> Self {
        VisualShaderNodeTransformCompose::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTransformConstant {
    fn class_name() -> &'static str {
        "VisualShaderNodeTransformConstant"
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

impl ToVariant for VisualShaderNodeTransformConstant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTransformConstant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTransformConstant {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTransformConstant {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTransformConstant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTransformConstant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTransformConstant {
    fn construct() -> Self {
        VisualShaderNodeTransformConstant::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTransformDecompose {
    fn class_name() -> &'static str {
        "VisualShaderNodeTransformDecompose"
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

impl ToVariant for VisualShaderNodeTransformDecompose {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTransformDecompose {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTransformDecompose {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTransformDecompose {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTransformDecompose {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTransformDecompose {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTransformDecompose {
    fn construct() -> Self {
        VisualShaderNodeTransformDecompose::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTransformFunc {
    fn class_name() -> &'static str {
        "VisualShaderNodeTransformFunc"
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

impl ToVariant for VisualShaderNodeTransformFunc {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTransformFunc {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTransformFunc {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTransformFunc {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTransformFunc {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTransformFunc {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTransformFunc {
    fn construct() -> Self {
        VisualShaderNodeTransformFunc::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTransformMult {
    fn class_name() -> &'static str {
        "VisualShaderNodeTransformMult"
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

impl ToVariant for VisualShaderNodeTransformMult {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTransformMult {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTransformMult {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTransformMult {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTransformMult {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTransformMult {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTransformMult {
    fn construct() -> Self {
        VisualShaderNodeTransformMult::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTransformUniform {
    fn class_name() -> &'static str {
        "VisualShaderNodeTransformUniform"
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

impl ToVariant for VisualShaderNodeTransformUniform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTransformUniform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTransformUniform {
    type Target = VisualShaderNodeUniform;

    fn deref(&self) -> &VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTransformUniform {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTransformUniform {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTransformUniform {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTransformUniform {
    fn construct() -> Self {
        VisualShaderNodeTransformUniform::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeTransformVecMult {
    fn class_name() -> &'static str {
        "VisualShaderNodeTransformVecMult"
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

impl ToVariant for VisualShaderNodeTransformVecMult {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeTransformVecMult {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeTransformVecMult {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeTransformVecMult {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeTransformVecMult {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeTransformVecMult {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeTransformVecMult {
    fn construct() -> Self {
        VisualShaderNodeTransformVecMult::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeUniform {
    fn class_name() -> &'static str {
        "VisualShaderNodeUniform"
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

impl ToVariant for VisualShaderNodeUniform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeUniform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeUniform {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeUniform {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeUniform {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeUniform {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for VisualShaderNodeVec3Constant {
    fn class_name() -> &'static str {
        "VisualShaderNodeVec3Constant"
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

impl ToVariant for VisualShaderNodeVec3Constant {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVec3Constant {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVec3Constant {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVec3Constant {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVec3Constant {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVec3Constant {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVec3Constant {
    fn construct() -> Self {
        VisualShaderNodeVec3Constant::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVec3Uniform {
    fn class_name() -> &'static str {
        "VisualShaderNodeVec3Uniform"
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

impl ToVariant for VisualShaderNodeVec3Uniform {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVec3Uniform {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVec3Uniform {
    type Target = VisualShaderNodeUniform;

    fn deref(&self) -> &VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVec3Uniform {
    fn deref_mut(&mut self) -> &mut VisualShaderNodeUniform {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVec3Uniform {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVec3Uniform {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVec3Uniform {
    fn construct() -> Self {
        VisualShaderNodeVec3Uniform::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorClamp {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorClamp"
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

impl ToVariant for VisualShaderNodeVectorClamp {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorClamp {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorClamp {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorClamp {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorClamp {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorClamp {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorClamp {
    fn construct() -> Self {
        VisualShaderNodeVectorClamp::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorCompose {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorCompose"
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

impl ToVariant for VisualShaderNodeVectorCompose {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorCompose {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorCompose {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorCompose {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorCompose {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorCompose {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorCompose {
    fn construct() -> Self {
        VisualShaderNodeVectorCompose::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorDecompose {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorDecompose"
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

impl ToVariant for VisualShaderNodeVectorDecompose {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorDecompose {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorDecompose {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorDecompose {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorDecompose {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorDecompose {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorDecompose {
    fn construct() -> Self {
        VisualShaderNodeVectorDecompose::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorDerivativeFunc {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorDerivativeFunc"
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

impl ToVariant for VisualShaderNodeVectorDerivativeFunc {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorDerivativeFunc {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorDerivativeFunc {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorDerivativeFunc {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorDerivativeFunc {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorDerivativeFunc {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorDerivativeFunc {
    fn construct() -> Self {
        VisualShaderNodeVectorDerivativeFunc::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorDistance {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorDistance"
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

impl ToVariant for VisualShaderNodeVectorDistance {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorDistance {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorDistance {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorDistance {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorDistance {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorDistance {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorDistance {
    fn construct() -> Self {
        VisualShaderNodeVectorDistance::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorFunc {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorFunc"
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

impl ToVariant for VisualShaderNodeVectorFunc {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorFunc {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorFunc {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorFunc {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorFunc {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorFunc {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorFunc {
    fn construct() -> Self {
        VisualShaderNodeVectorFunc::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorInterp {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorInterp"
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

impl ToVariant for VisualShaderNodeVectorInterp {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorInterp {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorInterp {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorInterp {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorInterp {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorInterp {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorInterp {
    fn construct() -> Self {
        VisualShaderNodeVectorInterp::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorLen {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorLen"
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

impl ToVariant for VisualShaderNodeVectorLen {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorLen {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorLen {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorLen {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorLen {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorLen {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorLen {
    fn construct() -> Self {
        VisualShaderNodeVectorLen::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorOp {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorOp"
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

impl ToVariant for VisualShaderNodeVectorOp {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorOp {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorOp {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorOp {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorOp {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorOp {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorOp {
    fn construct() -> Self {
        VisualShaderNodeVectorOp::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorRefract {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorRefract"
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

impl ToVariant for VisualShaderNodeVectorRefract {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorRefract {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorRefract {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorRefract {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorRefract {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorRefract {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorRefract {
    fn construct() -> Self {
        VisualShaderNodeVectorRefract::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorScalarMix {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorScalarMix"
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

impl ToVariant for VisualShaderNodeVectorScalarMix {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorScalarMix {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorScalarMix {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorScalarMix {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorScalarMix {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorScalarMix {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorScalarMix {
    fn construct() -> Self {
        VisualShaderNodeVectorScalarMix::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorScalarSmoothStep {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorScalarSmoothStep"
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

impl ToVariant for VisualShaderNodeVectorScalarSmoothStep {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorScalarSmoothStep {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorScalarSmoothStep {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorScalarSmoothStep {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorScalarSmoothStep {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorScalarSmoothStep {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorScalarSmoothStep {
    fn construct() -> Self {
        VisualShaderNodeVectorScalarSmoothStep::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorScalarStep {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorScalarStep"
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

impl ToVariant for VisualShaderNodeVectorScalarStep {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorScalarStep {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorScalarStep {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorScalarStep {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorScalarStep {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorScalarStep {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorScalarStep {
    fn construct() -> Self {
        VisualShaderNodeVectorScalarStep::new()
    }
}

unsafe impl GodotObject for VisualShaderNodeVectorSmoothStep {
    fn class_name() -> &'static str {
        "VisualShaderNodeVectorSmoothStep"
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

impl ToVariant for VisualShaderNodeVectorSmoothStep {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualShaderNodeVectorSmoothStep {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualShaderNodeVectorSmoothStep {
    type Target = VisualShaderNode;

    fn deref(&self) -> &VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualShaderNodeVectorSmoothStep {
    fn deref_mut(&mut self) -> &mut VisualShaderNode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for VisualShaderNodeVectorSmoothStep {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for VisualShaderNodeVectorSmoothStep {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for VisualShaderNodeVectorSmoothStep {
    fn construct() -> Self {
        VisualShaderNodeVectorSmoothStep::new()
    }
}

unsafe impl GodotObject for WeakRef {
    fn class_name() -> &'static str {
        "WeakRef"
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

impl ToVariant for WeakRef {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WeakRef {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WeakRef {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WeakRef {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WeakRef {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WeakRef {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for WeakRef {
    fn construct() -> Self {
        WeakRef::new()
    }
}

unsafe impl GodotObject for WebRTCDataChannel {
    fn class_name() -> &'static str {
        "WebRTCDataChannel"
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

impl ToVariant for WebRTCDataChannel {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WebRTCDataChannel {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WebRTCDataChannel {
    type Target = PacketPeer;

    fn deref(&self) -> &PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WebRTCDataChannel {
    fn deref_mut(&mut self) -> &mut PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WebRTCDataChannel {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WebRTCDataChannel {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for WebRTCDataChannelGDNative {
    fn class_name() -> &'static str {
        "WebRTCDataChannelGDNative"
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

impl ToVariant for WebRTCDataChannelGDNative {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WebRTCDataChannelGDNative {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WebRTCDataChannelGDNative {
    type Target = WebRTCDataChannel;

    fn deref(&self) -> &WebRTCDataChannel {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WebRTCDataChannelGDNative {
    fn deref_mut(&mut self) -> &mut WebRTCDataChannel {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WebRTCDataChannelGDNative {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WebRTCDataChannelGDNative {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for WebRTCDataChannelGDNative {
    fn construct() -> Self {
        WebRTCDataChannelGDNative::new()
    }
}

unsafe impl GodotObject for WebRTCMultiplayer {
    fn class_name() -> &'static str {
        "WebRTCMultiplayer"
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

impl ToVariant for WebRTCMultiplayer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WebRTCMultiplayer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WebRTCMultiplayer {
    type Target = NetworkedMultiplayerPeer;

    fn deref(&self) -> &NetworkedMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WebRTCMultiplayer {
    fn deref_mut(&mut self) -> &mut NetworkedMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WebRTCMultiplayer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WebRTCMultiplayer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for WebRTCMultiplayer {
    fn construct() -> Self {
        WebRTCMultiplayer::new()
    }
}

unsafe impl GodotObject for WebRTCPeerConnection {
    fn class_name() -> &'static str {
        "WebRTCPeerConnection"
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

impl ToVariant for WebRTCPeerConnection {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WebRTCPeerConnection {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WebRTCPeerConnection {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WebRTCPeerConnection {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WebRTCPeerConnection {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WebRTCPeerConnection {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for WebRTCPeerConnection {
    fn construct() -> Self {
        WebRTCPeerConnection::new()
    }
}

unsafe impl GodotObject for WebRTCPeerConnectionGDNative {
    fn class_name() -> &'static str {
        "WebRTCPeerConnectionGDNative"
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

impl ToVariant for WebRTCPeerConnectionGDNative {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WebRTCPeerConnectionGDNative {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WebRTCPeerConnectionGDNative {
    type Target = WebRTCPeerConnection;

    fn deref(&self) -> &WebRTCPeerConnection {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WebRTCPeerConnectionGDNative {
    fn deref_mut(&mut self) -> &mut WebRTCPeerConnection {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WebRTCPeerConnectionGDNative {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WebRTCPeerConnectionGDNative {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for WebRTCPeerConnectionGDNative {
    fn construct() -> Self {
        WebRTCPeerConnectionGDNative::new()
    }
}

unsafe impl GodotObject for WebSocketClient {
    fn class_name() -> &'static str {
        "WebSocketClient"
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

impl ToVariant for WebSocketClient {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WebSocketClient {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WebSocketClient {
    type Target = WebSocketMultiplayerPeer;

    fn deref(&self) -> &WebSocketMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WebSocketClient {
    fn deref_mut(&mut self) -> &mut WebSocketMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WebSocketClient {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WebSocketClient {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for WebSocketClient {
    fn construct() -> Self {
        WebSocketClient::new()
    }
}

unsafe impl GodotObject for WebSocketMultiplayerPeer {
    fn class_name() -> &'static str {
        "WebSocketMultiplayerPeer"
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

impl ToVariant for WebSocketMultiplayerPeer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WebSocketMultiplayerPeer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WebSocketMultiplayerPeer {
    type Target = NetworkedMultiplayerPeer;

    fn deref(&self) -> &NetworkedMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WebSocketMultiplayerPeer {
    fn deref_mut(&mut self) -> &mut NetworkedMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WebSocketMultiplayerPeer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WebSocketMultiplayerPeer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

unsafe impl GodotObject for WebSocketPeer {
    fn class_name() -> &'static str {
        "WebSocketPeer"
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

impl ToVariant for WebSocketPeer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WebSocketPeer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WebSocketPeer {
    type Target = PacketPeer;

    fn deref(&self) -> &PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WebSocketPeer {
    fn deref_mut(&mut self) -> &mut PacketPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WebSocketPeer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WebSocketPeer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for WebSocketPeer {
    fn construct() -> Self {
        WebSocketPeer::new()
    }
}

unsafe impl GodotObject for WebSocketServer {
    fn class_name() -> &'static str {
        "WebSocketServer"
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

impl ToVariant for WebSocketServer {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WebSocketServer {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for WebSocketServer {
    type Target = WebSocketMultiplayerPeer;

    fn deref(&self) -> &WebSocketMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WebSocketServer {
    fn deref_mut(&mut self) -> &mut WebSocketMultiplayerPeer {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for WebSocketServer {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for WebSocketServer {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for WebSocketServer {
    fn construct() -> Self {
        WebSocketServer::new()
    }
}

unsafe impl GodotObject for WindowDialog {
    fn class_name() -> &'static str {
        "WindowDialog"
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

impl ToVariant for WindowDialog {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WindowDialog {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for WindowDialog {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for WindowDialog {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for WindowDialog {
    type Target = Popup;

    fn deref(&self) -> &Popup {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WindowDialog {
    fn deref_mut(&mut self) -> &mut Popup {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for WindowDialog {
    fn construct() -> Self {
        WindowDialog::new()
    }
}

unsafe impl GodotObject for World {
    fn class_name() -> &'static str {
        "World"
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

impl ToVariant for World {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for World {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for World {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for World {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for World {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for World {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for World {
    fn construct() -> Self {
        World::new()
    }
}

unsafe impl GodotObject for World2D {
    fn class_name() -> &'static str {
        "World2D"
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

impl ToVariant for World2D {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for World2D {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for World2D {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for World2D {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for World2D {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for World2D {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for World2D {
    fn construct() -> Self {
        World2D::new()
    }
}

unsafe impl GodotObject for WorldEnvironment {
    fn class_name() -> &'static str {
        "WorldEnvironment"
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

impl ToVariant for WorldEnvironment {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for WorldEnvironment {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for WorldEnvironment {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for WorldEnvironment {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for WorldEnvironment {
    type Target = Node;

    fn deref(&self) -> &Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for WorldEnvironment {
    fn deref_mut(&mut self) -> &mut Node {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for WorldEnvironment {
    fn construct() -> Self {
        WorldEnvironment::new()
    }
}

unsafe impl GodotObject for X509Certificate {
    fn class_name() -> &'static str {
        "X509Certificate"
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

impl ToVariant for X509Certificate {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for X509Certificate {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for X509Certificate {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for X509Certificate {
    fn deref_mut(&mut self) -> &mut Resource {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for X509Certificate {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for X509Certificate {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for X509Certificate {
    fn construct() -> Self {
        X509Certificate::new()
    }
}

unsafe impl GodotObject for XMLParser {
    fn class_name() -> &'static str {
        "XMLParser"
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

impl ToVariant for XMLParser {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for XMLParser {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for XMLParser {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for XMLParser {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for XMLParser {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for XMLParser {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for XMLParser {
    fn construct() -> Self {
        XMLParser::new()
    }
}

unsafe impl GodotObject for YSort {
    fn class_name() -> &'static str {
        "YSort"
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

impl ToVariant for YSort {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for YSort {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl Free for YSort {
    unsafe fn godot_free(self) { self.free() }
}

impl QueueFree for YSort {
    unsafe fn godot_queue_free(&mut self) { self.queue_free() }
}

impl std::ops::Deref for YSort {
    type Target = Node2D;

    fn deref(&self) -> &Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for YSort {
    fn deref_mut(&mut self) -> &mut Node2D {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Instanciable for YSort {
    fn construct() -> Self {
        YSort::new()
    }
}

unsafe impl GodotObject for ClassDB {
    fn class_name() -> &'static str {
        "ClassDB"
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

impl ToVariant for ClassDB {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ClassDB {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ClassDB {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ClassDB {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Directory {
    fn class_name() -> &'static str {
        "Directory"
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

impl ToVariant for Directory {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Directory {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Directory {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Directory {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Directory {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Directory {
    fn construct() -> Self {
        Directory::new()
    }
}

unsafe impl GodotObject for Engine {
    fn class_name() -> &'static str {
        "Engine"
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

impl ToVariant for Engine {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Engine {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Engine {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Engine {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for File {
    fn class_name() -> &'static str {
        "File"
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

impl ToVariant for File {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for File {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for File {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for File {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for File {
    fn construct() -> Self {
        File::new()
    }
}

unsafe impl GodotObject for Geometry {
    fn class_name() -> &'static str {
        "Geometry"
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

impl ToVariant for Geometry {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Geometry {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Geometry {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Geometry {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for JSON {
    fn class_name() -> &'static str {
        "JSON"
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

impl ToVariant for JSON {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for JSON {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for JSON {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for JSON {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Marshalls {
    fn class_name() -> &'static str {
        "Marshalls"
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

impl ToVariant for Marshalls {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Marshalls {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Marshalls {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Marshalls {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Mutex {
    fn class_name() -> &'static str {
        "Mutex"
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

impl ToVariant for Mutex {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Mutex {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Mutex {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Mutex {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Mutex {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Mutex {
    fn construct() -> Self {
        Mutex::new()
    }
}

unsafe impl GodotObject for OS {
    fn class_name() -> &'static str {
        "OS"
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

impl ToVariant for OS {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for OS {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for OS {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for OS {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for ResourceLoader {
    fn class_name() -> &'static str {
        "ResourceLoader"
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

impl ToVariant for ResourceLoader {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ResourceLoader {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ResourceLoader {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ResourceLoader {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for ResourceSaver {
    fn class_name() -> &'static str {
        "ResourceSaver"
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

impl ToVariant for ResourceSaver {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for ResourceSaver {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for ResourceSaver {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for ResourceSaver {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

unsafe impl GodotObject for Semaphore {
    fn class_name() -> &'static str {
        "Semaphore"
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

impl ToVariant for Semaphore {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Semaphore {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Semaphore {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Semaphore {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Semaphore {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Semaphore {
    fn construct() -> Self {
        Semaphore::new()
    }
}

unsafe impl GodotObject for Thread {
    fn class_name() -> &'static str {
        "Thread"
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

impl ToVariant for Thread {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for Thread {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for Thread {
    type Target = Reference;

    fn deref(&self) -> &Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for Thread {
    fn deref_mut(&mut self) -> &mut Reference {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Clone for Thread {
    fn clone(&self) -> Self {
        self.new_ref()
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            if object::unref(self.this) {
                (get_api().godot_object_destroy)(self.this);
            }
        }
    }
}

impl Instanciable for Thread {
    fn construct() -> Self {
        Thread::new()
    }
}

unsafe impl GodotObject for VisualScriptEditor {
    fn class_name() -> &'static str {
        "VisualScriptEditor"
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

impl ToVariant for VisualScriptEditor {
    fn to_variant(&self) -> Variant { Variant::from_object(self) }
}
impl FromVariant for VisualScriptEditor {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> { variant.try_to_object_with_error::<Self>() }
}

impl std::ops::Deref for VisualScriptEditor {
    type Target = Object;

    fn deref(&self) -> &Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl std::ops::DerefMut for VisualScriptEditor {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe {
            std::mem::transmute(self)
        }
    }
}
