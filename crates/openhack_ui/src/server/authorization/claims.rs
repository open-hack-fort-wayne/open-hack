#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub iat: usize,
}
