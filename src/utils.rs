use cfg_if::cfg_if;
use worker::{Error, Response};

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub async fn map_reqwest_response(response: reqwest::Response) -> Result<Response, Error> {
    let headers = response.headers().clone();
    let mut worker_response = worker::Response::from_bytes(
        response
            .bytes()
            .await
            .map_err(|err| Error::from(err.to_string()))?
            .to_vec(),
    )
    .map_err(|err| Error::from(err.to_string()))?;
    let worker_response_headers = worker_response.headers_mut();

    for (name, value) in headers.iter() {
        worker_response_headers
            .set(
                name.to_string().as_str(),
                value.to_str().map_err(|err| Error::from(err.to_string()))?,
            )
            .map_err(|err| Error::from(err.to_string()))?;
    }

    Ok(worker_response)
}
