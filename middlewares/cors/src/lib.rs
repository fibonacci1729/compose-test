use bindings::{downstream, inbound_http::{InboundHttp, Request, Response}};

struct Component;

impl InboundHttp for Component {
    fn handle_request(req: Request) -> Response {
        println!("ENTER CORS");

        _ = downstream::handle_request(&downstream::Request {
            headers: req.headers,
            params: req.params,
            method: downstream::Method::Get,
            uri: req.uri,
            body: req.body,
        });

        println!("LEAVE CORS");

        Response { 
            headers: None,
            body: None,
            status: 200 
        }
    }
}

bindings::export!(Component);
