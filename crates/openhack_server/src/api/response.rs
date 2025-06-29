use ::serde::Serialize;
use axum::{
    Json,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

pub struct ResponseWithHeaders<T>(Response<T>, HeaderMap);

#[derive(Debug)]
pub enum Response<T> {
    Data(T),
    Error(String),
}

impl<T> Response<T> {
    pub fn with_headers(self, headers: HeaderMap) -> ResponseWithHeaders<T> {
        ResponseWithHeaders(self, headers)
    }
}

impl<T: Serialize> Response<T> {
    pub fn success(data: T) -> Self {
        Self::Data(data)
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self::Error(message.into())
    }

    pub fn from_result<E>(result: Result<T, E>) -> Self
    where
        E: ToString,
    {
        match result {
            Ok(data) => Self::success(data),
            Err(error) => Self::failure(error.to_string()),
        }
    }
}

impl<T: Serialize> Serialize for Response<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use ::serde::ser::SerializeMap;

        match self {
            Response::Data(data) => {
                // Serialize Data(T) transparently (directly as T)
                data.serialize(serializer)
            }
            Response::Error(message) => {
                // Serialize Error as a tagged object
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("status", "error")?;
                map.serialize_entry("message", message)?;
                map.end()
            }
        }
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Data(data) => (StatusCode::OK, Json(Self::Data(data))).into_response(),
            Self::Error(message) => {
                (StatusCode::BAD_REQUEST, Json(Self::Error(message))).into_response()
            }
        }
    }
}

impl<T: Serialize> IntoResponse for ResponseWithHeaders<T> {
    fn into_response(self) -> axum::response::Response {
        let mut response = self.0.into_response();
        *response.headers_mut() = self.1;
        response
    }
}
