//! Session definitions for remmina.

/// Session state
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    /// Idle session
    Idle,
    /// Connecting session
    Connecting,
    /// Active session
    Active,
    /// Paused session
    Paused,
    /// Disconnected session
    Disconnected,
}

/// Session kind
#[derive(Debug, Clone, PartialEq)]
pub enum SessionKind {
    /// Desktop session
    Desktop,
    /// Terminal session
    Terminal,
    /// File transfer session
    FileTransfer,
}

/// Session stats
#[derive(Debug, Clone, Default)]
pub struct SessionStats {
    /// Bytes sent
    pub bytes_sent: u64,
    /// Bytes received
    pub bytes_recv: u64,
    /// Packets sent
    pub packets_sent: u64,
    /// Packets received
    pub packets_recv: u64,
    /// Duration in seconds
    pub duration_secs: u64,
    /// Latency in ms
    pub latency_ms: u64,
}

impl SessionStats {
    /// Create new session stats
    pub fn new(bytes_sent: u64, bytes_recv: u64, duration_secs: u64) -> Self {
        Self {
            bytes_sent,
            bytes_recv,
            packets_sent: 0,
            packets_recv: 0,
            duration_secs,
            latency_ms: 0,
        }
    }
    
    /// Set latency
    pub fn set_latency(&mut self, latency_ms: u64) {
        self.latency_ms = latency_ms;
    }
    
    /// Add bytes sent
    pub fn add_sent(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }
    
    /// Add bytes received
    pub fn add_recv(&mut self, bytes: u64) {
        self.bytes_recv += bytes;
    }
}

/// Remote session
#[derive(Debug, Clone)]
pub struct RemoteSession {
    /// Session ID
    pub id: String,
    /// Session kind
    pub kind: SessionKind,
    /// Session state
    pub state: SessionState,
    /// Connection ID
    pub connection_id: String,
    /// Session stats
    pub stats: SessionStats,
    /// Created timestamp
    pub created_at: u64,
    /// Started timestamp
    pub started_at: Option<u64>,
    /// Finished timestamp
    pub finished_at: Option<u64>,
}

impl RemoteSession {
    /// Create new remote session
    pub fn new(id: String, kind: SessionKind) -> Self {
        Self {
            id,
            kind,
            state: SessionState::Idle,
            connection_id: String::new(),
            stats: SessionStats::default(),
            created_at: Self::now(),
            started_at: None,
            finished_at: None,
        }
    }
    
    /// Set connection
    pub fn set_connection(&mut self, connection_id: String) {
        self.connection_id = connection_id;
    }
    
    /// Set stats
    pub fn set_stats(&mut self, stats: SessionStats) {
        self.stats = stats;
    }
    
    /// Start session
    pub fn start(&mut self) {
        self.state = SessionState::Connecting;
        self.started_at = Some(Self::now());
    }
    
    /// Finish connecting
    pub fn finish_connecting(&mut self) {
        self.state = SessionState::Active;
    }
    
    /// Pause session
    pub fn pause(&mut self) {
        if self.state == SessionState::Active {
            self.state = SessionState::Paused;
        }
    }
    
    /// Resume session
    pub fn resume(&mut self) {
        if self.state == SessionState::Paused {
            self.state = SessionState::Active;
        }
    }
    
    /// Disconnect session
    pub fn disconnect(&mut self) {
        self.state = SessionState::Disconnected;
        self.finished_at = Some(Self::now());
    }
    
    /// Fail session
    pub fn fail(&mut self) {
        self.state = SessionState::Disconnected;
        self.finished_at = Some(Self::now());
    }
    
    /// Check if active
    pub fn is_active(&self) -> bool {
        self.state == SessionState::Active
    }
    
    /// Get session duration in seconds
    pub fn get_duration_seconds(&self) -> u64 {
        if let (Some(start), Some(end)) = (self.started_at, self.finished_at) {
            end - start
        } else {
            Self::now() - self.created_at
        }
    }
    
    /// Get session age in seconds
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
