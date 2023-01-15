
#[derive(Clone, Debug)]
pub struct Server {
    clients: HashMap<String, Client>,
    clients_count: usize,
}

pub struct CORS {
    enabled: bool,
    origin: String,
    methods: String,
    allowed_headers: String,
    exposed_headers: String,
    credentials: bool,
    max_age: u32,
}

pub struct ServerBuilder {
    ping_timeout: Duration, // How many ms without a pong packet to consider the connection closed
    ping_interval: Duration, // How many ms before sending a new ping packet
    upgrade_timeout: Duration, // How many ms before an unfinished transport upgrade is cancelled
    max_http_buffer_size: usize, // How many bytes or characters a http message can be, before closing the session
    allow_request: Option<Box<dyn Fn(&Request) -> bool + Send + Sync>>, // TODO: Figure out if this is the right way to do this
    transports: Vec<String>, // Transports to allow connections to (ordered by their priority?)
    allow_upgrades: bool, // Indicates whether to allow transport upgrades
    per_message_deflate: bool,
    threshold: usize,
    http_compression: bool,
    cookie: bool,
    ws_engine : Option<Box<dyn WsEngine>>, // TODO: Figure out if this is the right way to do this
    cors: Option<Cors>,
    initial_packet: Option<Packet>,
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self { 
            ping_timeout: Duration::from_millis(20000),
            ping_interval: Duration::from_millis(25000),
            upgrade_timeout: Duration::from_millis(10000),
            max_http_buffer_size: 1E6 as usize,
            allow_request: None,
            transports: vec!["polling".to_string(), "websocket".to_string()],
            allow_upgrades: true,
            per_message_deflate: false,
            threshold: 1024,
            http_compression: false,
            cookie: false,

        }
    }
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn close() {
        // TODO: Implement this
        unimplemented!();
    }

    fn handle_request(request: Request, response: Response) -> Self {
        // TODO: Implement this
        unimplemented!();
    }

    fn handle_upgrade(request: Request, socket: TcpStream, tail: Bytes) -> Self {
        // TODO: Implement this
        unimplemented!();
    }

    fn generate_id() -> String {
        // Generate a random id
        let rng = rand::thread_rng();
        return rng
    }
}

