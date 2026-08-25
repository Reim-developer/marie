use std::any::Any;

pub const fn not_used(var: &dyn Any) {
    _ = var;
}
