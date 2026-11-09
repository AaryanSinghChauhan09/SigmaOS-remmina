//! Connection definitions for remmina.

/// Connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnState {
    /// Disconnected
    Disconnected,
    /// Connecting
    Connecting,
    /// Connected
    Connected,
    /// Disconnected2 (alias for compatibility)
    Disconnected2,
    /// Failed
    Failed,
}

/// Connection config
#[derive(Debug, Clone)]
pub struct ConnConfig {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Resolution
    pub resolution: String,
    /// Fullscreen
    pub fullscreen: bool,
    /// Color depth
    pub color_depth: u8,
    /// Compression
    pub compression: bool,
    /// Audio enabled
    pub audio_enabled: bool,
}

impl Default for ConnConfig {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            resolution: "1024x768".into(),
            fullscreen: false,
            color_depth: 24,
            compression: true,
            audio_enabled: false,
        }
    }
}

/// Connection info
#[derive(Debug, Clone, Default)]
pub struct ConnInfo {
    /// Description
    pub description: String,
    /// Group
    pub group: String,
    /// Last connected
    pub last_connected: Option<u64>,
    /// Connection count
    pub connect_count: u32,
}

/// Connection
#[derive(Debug, Clone)]
pub struct Connection {
    /// Connection ID
    pub id: String,
    /// Connection name
    pub name: String,
    /// Protocol kind
    pub protocol: crate::protocol::ProtocolKind,
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Connection state
    pub state: ConnState,
    /// Connection config
    pub config: ConnConfig,
    /// Connection info
    pub info: ConnInfo,
    /// Created timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub last_modified: u64,
}

impl Connection {
    /// Create new connection
    pub fn new(name: String, protocol: crate::protocol::ProtocolKind, host: String, port: u16) -> Self {
        Self {
            id: format!("conn-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            protocol,
            host,
            port,
            state: ConnState::Disconnected,
            config: ConnConfig::default(),
            info: ConnInfo::default(),
            created_at: Self::now(),
            last_modified: Self::now(),
        }
    }
    
    /// Set config
    pub fn set_config(&mut self, config: ConnConfig) {
        self.config = config;
        self.last_modified = Self::now();
    }
    
    /// Set info
    pub fn set_info(&mut self, info: ConnInfo) {
        self.info = info;
        self.last_modified = Self::now();
    }
    
    /// Connect
    pub fn connect(&mut self) {
        self.state = ConnState::Connecting;
        self.last_modified = Self::now();
    }
    
    /// Finish connecting
    pub fn finish_connecting(&mut self) {
        self.state = ConnState::Connected;
        self.info.last_connected = Some(Self::now());
        self.info.connect_count += 1;
        self.last_modified = Self::now();
    }
    
    /// Fail connecting
    pub fn fail(&mut self) {
        self.state = ConnState::Failed;
        self.last_modified = Self::now();
    }
    
    /// Disconnect
    pub fn disconnect(&mut self) {
        self.state = ConnState::Disconnected;
        self.last_modified = Self::now();
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.state == ConnState::Connected
    }
    
    /// Check if connecting
    pub fn is_connecting(&self) -> bool {
        self.state == ConnState::Connecting
    }
    
    /// Get connection age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Get time since last modified
    pub fn get_idle_seconds(&self) -> u64 {
        Self::now() - self.last_modified
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
