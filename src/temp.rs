use axum::{
    extract::{Multipart, FromRequest, RequestParts},
    handler::post,
    Router,
};
use futures::StreamExt;
use bytes::Bytes;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Debug)]
struct ProbeReturnData {
    uri: String,
    cookies: String,
    referrer: String,
    user_agent: String,
    browser_time: String,
    probe_uid: String,
    origin: String,
    injection_key: String,
    title: String,
    text: String,
    was_iframe: bool,
    dom: String,
    screenshot: Vec<u8>, // This will hold the binary data of the screenshot
}

impl<B> FromRequest<B> for ProbeReturnData
where
    B: http_body::Body<Data = Bytes> + Send,
    B::Error: Into<BoxError>,
{
    type Rejection = std::convert::Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(req).await.unwrap();
        let mut data = ProbeReturnData {
            uri: String::new(),
            cookies: String::new(),
            referrer: String::new(),
            user_agent: String::new(),
            browser_time: String::new(),
            probe_uid: String::new(),
            origin: String::new(),
            injection_key: String::new(),
            title: String::new(),
            text: String::new(),
            was_iframe: false,
            dom: String::new(),
            screenshot: Vec::new(),
        };

        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap().to_string();
            if name == "screenshot" {
                let screenshot_data = FramedRead::new(field.bytes_stream(), BytesCodec::new()).collect::<Vec<_>>().await;
                data.screenshot = screenshot_data.iter().flat_map(|bytes| bytes.iter().copied()).collect();
            } else {
                let value = field.text().await.unwrap();
                match name.as_str() {
                    "uri" => data.uri = value,
                    "cookies" => data.cookies = value,
                    "referrer" => data.referrer = value,
                    "user-agent" => data.user_agent = value,
                    "browser-time" => data.browser_time = value,
                    "probe-uid" => data.probe_uid = value,
                    "origin" => data.origin = value,
                    "injection_key" => data.injection_key = value,
                    "title" => data.title = value,
                    "text" => data.text = value,
                    "was_iframe" => data.was_iframe = value.parse().unwrap(),
                    "dom" => data.dom = value,
                    _ => {}
                }
            }
        }

        Ok(data)
    }
}

async fn handle_probe_return_data(data: ProbeReturnData) -> impl IntoResponse {
    // You can now access the fields of `data` as normal Rust fields.
    println!("Received probe_uid: {}", data.probe_uid);

    // Return a response...
}

fn main() {
    let app = Router::new().route("/probe_return_data", post(handle_probe_return_data));
    // ...
}