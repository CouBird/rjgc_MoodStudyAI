pub const ROLE_USER: &str = "user";
pub const ROLE_ADMIN: &str = "admin";

pub fn is_admin(role: &str) -> bool {
    role == ROLE_ADMIN
}
