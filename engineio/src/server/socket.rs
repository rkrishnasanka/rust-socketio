enum ReadyState {
    Opening,
    Open,
    Closing,
    Closed,
}

pub struct Socket {
    id : String,
    server: Server,
    request: object, // Not sure how this would be handled, in the javascript version this is a request object that I need to use
    upgraded: bool,
    ready_state: ReadyState,
    transport: Transport,
}

pub impl Socket {
    pub fn new(server: Server, request: object, transport: Transport) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            server,
            request,
            upgraded: false,
            ready_state: ReadyState::Opening,
            transport,
        }
    }

    pub fn send(&self, data: String, callback: Option<fn()>, compress: Option<bool>) {
        // Set compress to ture if it is not set
        let compress = match compress {
            Some(compress) => compress,
            None => true,
        };

        unimplemented!()
    }

    pub fn close(&self) {
        unimplemented!()
    }

}