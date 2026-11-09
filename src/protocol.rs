//! Protocol definitions for remmina.

/// Protocol kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProtocolKind {
    /// RDP protocol
    Rdp,
    /// VNC protocol
    Vnc,
    /// SSH protocol
    Ssh,
    /// SFTP protocol
    Sftp,
    /// Telnet protocol
    Telnet,
    /// HTTP protocol
    Http,
    /// Custom protocol
    Custom(String),
}

impl ProtocolKind {
    /// Get protocol name
    pub fn name(&self) -> &str {
        match self {
            ProtocolKind::Rdp => "rdp",
            ProtocolKind::Vnc => "vnc",
            ProtocolKind::Ssh => "ssh",
            ProtocolKind::Sftp => "sftp",
            ProtocolKind::Telnet => "telnet",
            ProtocolKind::Http => "http",
            ProtocolKind::Custom(name) => name.as_str(),
        }
    }
    
    /// Get default port
    pub fn default_port(&self) -> u16 {
        match self {
            ProtocolKind::Rdp => 3389,
            ProtocolKind::Vnc => 5900,
            ProtocolKind::Ssh => 22,
            ProtocolKind::Sftp => 22,
            ProtocolKind::Telnet => 23,
            ProtocolKind::Http => 80,
            ProtocolKind::Custom(_) => 0,
        }
    }
}

/// Protocol state
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolState {
    /// Idle protocol
    Idle,
    /// Active protocol
    Active,
    /// Error protocol
    Error,
}

/// Protocol info
#[derive(Debug, Clone, Default)]
pub struct ProtocolInfo {
    /// Default port
    pub default_port: u16,
    /// Supports encryption
    pub supports_encryption: bool,
    /// Supports compression
    pub supports_compression: bool,
    /// Max resolution
    pub max_resolution: String,
    /// Supported color depths
    pub color_depths: Vec<u8>,
}

/// Protocol
#[derive(Debug, Clone)]
pub struct Protocol {
    /// Protocol kind
    pub kind: ProtocolKind,
    /// Protocol state
    pub state: ProtocolState,
    /// Protocol info
    pub info: ProtocolInfo,
    /// Created timestamp
    pub created_at: u64,
}

impl Protocol {
    /// Create new protocol
    pub fn new(kind: ProtocolKind) -> Self {
        Self {
            kind,
            state: ProtocolState::Idle,
            info: ProtocolInfo::default(),
            created_at: Self::now(),
        }
    }
    
    /// Set info
    pub fn set_info(&mut self, info: ProtocolInfo) {
        self.info = info;
    }
    
    /// Activate protocol
    pub fn activate(&mut self) {
        self.state = ProtocolState::Active;
    }
    
    /// Deactivate protocol
    pub fn deactivate(&mut self) {
        self.state = ProtocolState::Idle;
    }
    
    /// Set error
    pub fn set_error(&mut self) {
        self.state = ProtocolState::Error;
    }
    
    /// Check if active
    pub fn is_active(&self) -> bool {
        self.state == ProtocolState::Active
    }
    
    /// Get protocol age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
