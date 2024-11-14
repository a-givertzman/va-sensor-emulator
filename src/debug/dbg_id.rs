pub struct DbgId(pub String);
//
//
impl DbgId {
    pub fn with_parent(dbgid: &DbgId, me: &str) -> Self {
        Self(format!("{}/{}", dbgid, me))
    }
}
//
//
impl std::fmt::Display for DbgId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
//
//
impl std::fmt::Debug for DbgId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
//
//
impl Into<String> for DbgId {
    fn into(self) -> String {
        self.0
    }
}
//
//
impl Into<String> for &DbgId {
    fn into(self) -> String {
        self.0.clone()
    }
}
//
//
impl Clone for DbgId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}