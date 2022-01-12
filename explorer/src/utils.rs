use anyhow::Error;
use poem::http::StatusCode;

#[allow(non_snake_case)]
#[allow(unreachable_patterns)]
#[allow(unused_variables)]
pub fn handle_fetch_one_err(err: Error) -> poem::Error {
    log::debug!("get_tx err:{}", err.to_string());

    let code = match err {
        RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    poem::Error::from_status(code)
}
