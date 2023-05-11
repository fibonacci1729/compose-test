use bindings::inbound_http::{InboundHttp, Request, Response};

struct Component;

impl InboundHttp for Component {
    fn handle_request(_req: Request) -> Response {
        println!("BINGO SERVICE");
        Response {
            headers: None,
            body: None,
            status: 200,
        }
    }
}

bindings::export!(Component);