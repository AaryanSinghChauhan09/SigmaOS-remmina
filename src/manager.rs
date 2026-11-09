//! Remmina manager.

use std::collections::HashMap;
use crate::connection::{Connection, ConnState};
use crate::protocol::{Protocol, ProtocolKind, ProtocolState};
use crate::session::{RemoteSession, SessionState, SessionKind};
use crate::profile::{Profile, ProfileKind, ProfileState};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max connections
    pub max_connections: usize,
    /// Max sessions
    pub max_sessions: usize,
    /// Max profiles
    pub max_profiles: usize,
    /// Default connection timeout in seconds
    pub connection_timeout_secs: u64,
    /// Auto-reconnect
    pub auto_reconnect: bool,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_connections: 100,
            max_sessions: 50,
            max_profiles: 50,
            connection_timeout_secs: 30,
            auto_reconnect: true,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total connections
    pub total_connections: usize,
    /// Connected connections
    pub connected_connections: usize,
    /// Total sessions
    pub total_sessions: usize,
    /// Active sessions
    pub active_sessions: usize,
    /// Total profiles
    pub total_profiles: usize,
    /// Active profiles
    pub active_profiles: usize,
    /// Total protocols
    pub total_protocols: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Remmina manager
#[derive(Debug, Clone)]
pub struct RemminaManager {
    /// Connections
    connections: HashMap<String, Connection>,
    /// Sessions
    sessions: HashMap<String, RemoteSession>,
    /// Session -> connection mapping
    session_connection: HashMap<String, String>,
    /// Profiles
    profiles: HashMap<String, Profile>,
    /// Protocols
    protocols: HashMap<ProtocolKind, Protocol>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for RemminaManager {
    fn default() -> Self {
        let mut protocols = HashMap::new();
        protocols.insert(ProtocolKind::Rdp, Protocol::new(ProtocolKind::Rdp));
        protocols.insert(ProtocolKind::Vnc, Protocol::new(ProtocolKind::Vnc));
        protocols.insert(ProtocolKind::Ssh, Protocol::new(ProtocolKind::Ssh));
        protocols.insert(ProtocolKind::Sftp, Protocol::new(ProtocolKind::Sftp));
        protocols.insert(ProtocolKind::Telnet, Protocol::new(ProtocolKind::Telnet));
        protocols.insert(ProtocolKind::Http, Protocol::new(ProtocolKind::Http));
        
        Self {
            connections: HashMap::new(),
            sessions: HashMap::new(),
            session_connection: HashMap::new(),
            profiles: HashMap::new(),
            protocols,
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_connections: 0,
                connected_connections: 0,
                total_sessions: 0,
                active_sessions: 0,
                total_profiles: 0,
                active_profiles: 0,
                total_protocols: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl RemminaManager {
    /// Create new remmina manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        let mut protocols = HashMap::new();
        protocols.insert(ProtocolKind::Rdp, Protocol::new(ProtocolKind::Rdp));
        protocols.insert(ProtocolKind::Vnc, Protocol::new(ProtocolKind::Vnc));
        protocols.insert(ProtocolKind::Ssh, Protocol::new(ProtocolKind::Ssh));
        protocols.insert(ProtocolKind::Sftp, Protocol::new(ProtocolKind::Sftp));
        protocols.insert(ProtocolKind::Telnet, Protocol::new(ProtocolKind::Telnet));
        protocols.insert(ProtocolKind::Http, Protocol::new(ProtocolKind::Http));
        
        Self {
            connections: HashMap::new(),
            sessions: HashMap::new(),
            session_connection: HashMap::new(),
            profiles: HashMap::new(),
            protocols,
            options,
            statistics: ManagerStatistics {
                total_connections: 0,
                connected_connections: 0,
                total_sessions: 0,
                active_sessions: 0,
                total_profiles: 0,
                active_profiles: 0,
                total_protocols: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        self.update_statistics();
    }
    
    // Connection methods
    
    /// Add connection
    pub fn add_connection(&mut self, name: String, protocol: ProtocolKind, host: String, port: u16) -> String {
        let conn = Connection::new(name.clone(), protocol, host, port);
        let id = name.clone();
        
        self.connections.insert(id.clone(), conn);
        self.update_statistics();
        id
    }
    
    /// Get connection
    pub fn get_connection(&self, conn_id: &str) -> Option<&Connection> {
        self.connections.get(conn_id)
    }
    
    /// Get connection mutably
    pub fn get_connection_mut(&mut self, conn_id: &str) -> Option<&mut Connection> {
        self.connections.get_mut(conn_id)
    }
    
    /// Get all connections
    pub fn list_connections(&self) -> Vec<&Connection> {
        self.connections.values().collect()
    }
    
    /// Remove connection
    pub fn remove_connection(&mut self, conn_id: &str) -> bool {
        if self.connections.remove(conn_id).is_some() {
            // Remove from profiles
            for profile in self.profiles.values_mut() {
                profile.remove_connection(conn_id);
            }
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get connections by state
    pub fn get_connections_by_state(&self, state: &ConnState) -> Vec<&Connection> {
        self.connections.values()
            .filter(|c| c.state == *state)
            .collect()
    }
    
    /// Get connected connections
    pub fn get_connected_connections(&self) -> Vec<&Connection> {
        self.get_connections_by_state(&ConnState::Connected)
    }
    
    /// Search connections
    pub fn search_connections(&self, query: &str) -> Vec<&Connection> {
        let query_lower = query.to_lowercase();
        self.connections.values()
            .filter(|c| c.name.to_lowercase().contains(&query_lower) ||
                       c.host.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Session methods
    
    /// Connect to connection
    pub fn connect(&mut self, conn_id: &str) -> String {
        let kind = self.connections.get(conn_id)
            .map(|c| match c.protocol {
                ProtocolKind::Rdp | ProtocolKind::Vnc => SessionKind::Desktop,
                ProtocolKind::Ssh | ProtocolKind::Sftp | ProtocolKind::Telnet => SessionKind::Terminal,
                ProtocolKind::Http => SessionKind::FileTransfer,
                ProtocolKind::Custom(_) => SessionKind::Desktop,
            })
            .unwrap_or(SessionKind::Desktop);
        
        let mut session = RemoteSession::new(format!("session-{}", self.sessions.len()), kind);
        session.set_connection(conn_id.to_string());
        session.start();
        session.finish_connecting();
        
        if let Some(conn) = self.connections.get_mut(conn_id) {
            conn.connect();
            conn.finish_connecting();
        }
        
        let id = session.id.clone();
        self.session_connection.insert(id.clone(), conn_id.to_string());
        self.sessions.insert(id.clone(), session);
        self.update_statistics();
        id
    }
    
    /// Get session
    pub fn get_session(&self, session_id: &str) -> Option<&RemoteSession> {
        self.sessions.get(session_id)
    }
    
    /// Get session mutably
    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut RemoteSession> {
        self.sessions.get_mut(session_id)
    }
    
    /// Get all sessions
    pub fn list_sessions(&self) -> Vec<&RemoteSession> {
        self.sessions.values().collect()
    }
    
    /// Remove session
    pub fn remove_session(&mut self, session_id: &str) -> bool {
        self.session_connection.remove(session_id);
        if self.sessions.remove(session_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disconnect session
    pub fn disconnect_session(&mut self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.disconnect();
            
            if let Some(conn_id) = self.session_connection.get(session_id) {
                if let Some(conn) = self.connections.get_mut(conn_id) {
                    conn.disconnect();
                }
            }
            
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Pause session
    pub fn pause_session(&mut self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.pause();
            return true;
        }
        false
    }
    
    /// Resume session
    pub fn resume_session(&mut self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.resume();
            return true;
        }
        false
    }
    
    /// Get sessions by state
    pub fn get_sessions_by_state(&self, state: &SessionState) -> Vec<&RemoteSession> {
        self.sessions.values()
            .filter(|s| s.state == *state)
            .collect()
    }
    
    /// Get active sessions
    pub fn get_active_sessions(&self) -> Vec<&RemoteSession> {
        self.get_sessions_by_state(&SessionState::Active)
    }
    
    /// Get sessions for connection
    pub fn get_connection_sessions(&self, conn_id: &str) -> Vec<&RemoteSession> {
        self.session_connection.iter()
            .filter(|(_, cid)| *cid == conn_id)
            .filter_map(|(sid, _)| self.sessions.get(sid))
            .collect()
    }
    
    // Profile methods
    
    /// Add profile
    pub fn add_profile(&mut self, name: String, kind: ProfileKind) -> String {
        let profile = Profile::new(name.clone(), kind);
        let id = name.clone();
        
        self.profiles.insert(id.clone(), profile);
        self.update_statistics();
        id
    }
    
    /// Get profile
    pub fn get_profile(&self, profile_id: &str) -> Option<&Profile> {
        self.profiles.get(profile_id)
    }
    
    /// Get profile mutably
    pub fn get_profile_mut(&mut self, profile_id: &str) -> Option<&mut Profile> {
        self.profiles.get_mut(profile_id)
    }
    
    /// Get all profiles
    pub fn list_profiles(&self) -> Vec<&Profile> {
        self.profiles.values().collect()
    }
    
    /// Remove profile
    pub fn remove_profile(&mut self, profile_id: &str) -> bool {
        if self.profiles.remove(profile_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add connection to profile
    pub fn add_connection_to_profile(&mut self, profile_id: &str, conn_id: &str) -> bool {
        if let Some(profile) = self.profiles.get_mut(profile_id) {
            return profile.add_connection(conn_id.to_string());
        }
        false
    }
    
    /// Remove connection from profile
    pub fn remove_connection_from_profile(&mut self, profile_id: &str, conn_id: &str) -> bool {
        if let Some(profile) = self.profiles.get_mut(profile_id) {
            return profile.remove_connection(conn_id);
        }
        false
    }
    
    /// Get connections in profile
    pub fn get_profile_connections(&self, profile_id: &str) -> Vec<&Connection> {
        if let Some(profile) = self.profiles.get(profile_id) {
            profile.info.connection_ids.iter()
                .filter_map(|cid| self.connections.get(cid))
                .collect()
        } else {
            vec![]
        }
    }
    
    /// Get profiles by state
    pub fn get_profiles_by_state(&self, state: &ProfileState) -> Vec<&Profile> {
        self.profiles.values()
            .filter(|p| p.state == *state)
            .collect()
    }
    
    /// Get active profiles
    pub fn get_active_profiles(&self) -> Vec<&Profile> {
        self.get_profiles_by_state(&ProfileState::Active)
    }
    
    /// Search profiles
    pub fn search_profiles(&self, query: &str) -> Vec<&Profile> {
        let query_lower = query.to_lowercase();
        self.profiles.values()
            .filter(|p| p.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Protocol methods
    
    /// Get protocol
    pub fn get_protocol(&self, kind: &ProtocolKind) -> Option<&Protocol> {
        self.protocols.get(kind)
    }
    
    /// Get protocol mutably
    pub fn get_protocol_mut(&mut self, kind: &ProtocolKind) -> Option<&mut Protocol> {
        self.protocols.get_mut(kind)
    }
    
    /// Get all protocols
    pub fn list_protocols(&self) -> Vec<&Protocol> {
        self.protocols.values().collect()
    }
    
    /// Add protocol
    pub fn add_protocol(&mut self, kind: ProtocolKind) -> bool {
        if self.protocols.contains_key(&kind) {
            return false;
        }
        self.protocols.insert(kind.clone(), Protocol::new(kind));
        self.update_statistics();
        true
    }
    
    /// Remove protocol
    pub fn remove_protocol(&mut self, kind: &ProtocolKind) -> bool {
        if self.protocols.remove(kind).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Activate protocol
    pub fn activate_protocol(&mut self, kind: &ProtocolKind) -> bool {
        if let Some(proto) = self.protocols.get_mut(kind) {
            proto.activate();
            return true;
        }
        false
    }
    
    /// Deactivate protocol
    pub fn deactivate_protocol(&mut self, kind: &ProtocolKind) -> bool {
        if let Some(proto) = self.protocols.get_mut(kind) {
            proto.deactivate();
            return true;
        }
        false
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_connections = self.connections.len();
        self.statistics.connected_connections = self.connections.values()
            .filter(|c| c.is_connected())
            .count();
        self.statistics.total_sessions = self.sessions.len();
        self.statistics.active_sessions = self.sessions.values()
            .filter(|s| s.is_active())
            .count();
        self.statistics.total_profiles = self.profiles.len();
        self.statistics.active_profiles = self.profiles.values()
            .filter(|p| p.is_active())
            .count();
        self.statistics.total_protocols = self.protocols.len();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            connections: self.connections.values().cloned().collect(),
            profiles: self.profiles.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.connections.clear();
        for conn in export.connections {
            self.connections.insert(conn.id.clone(), conn);
        }
        
        self.profiles.clear();
        for profile in export.profiles {
            self.profiles.insert(profile.id.clone(), profile);
        }
        
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Connections
    pub connections: Vec<Connection>,
    /// Profiles
    pub profiles: Vec<Profile>,
}
