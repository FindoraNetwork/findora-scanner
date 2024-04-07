use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, (StatusCode, String)>;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    let err_msg = err.to_string();
    if err_msg.contains("now rows") {
        return (StatusCode::NOT_FOUND, "not found".to_string());
    }

    (StatusCode::INTERNAL_SERVER_ERROR, err_msg)
}
