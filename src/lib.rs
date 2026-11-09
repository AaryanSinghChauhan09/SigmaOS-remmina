//! SigmaOS-remmina — remote desktop client for SigmaOS.
//!
//! Models remmina's remote connections: RDP/VNC/SSH protocols,
//! connection profiles, sessions, and authentication.

pub mod connection;
pub mod protocol;
pub mod session;
pub mod profile;
pub mod manager;
pub mod options;
pub mod util;

pub use connection::{Connection, ConnState, ConnConfig, ConnInfo};
pub use protocol::{Protocol, ProtocolKind, ProtocolState, ProtocolInfo};
pub use session::{RemoteSession, SessionState, SessionKind, SessionStats};
pub use profile::{Profile, ProfileKind, ProfileState, ProfileInfo};
pub use manager::{RemminaManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};
pub use util::{Connection as UtilConnection, Model};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conn_state() {
        let states = vec![ConnState::Disconnected, ConnState::Connecting, ConnState::Connected, ConnState::Disconnected2, ConnState::Failed];
        assert_eq!(states.len(), 5);
    }
    
    #[test]
    fn test_protocol_kind() {
        let kinds = vec![ProtocolKind::Rdp, ProtocolKind::Vnc, ProtocolKind::Ssh, ProtocolKind::Sftp, ProtocolKind::Telnet, ProtocolKind::Http, ProtocolKind::Custom(String::new())];
        assert_eq!(kinds.len(), 7);
    }
    
    #[test]
    fn test_session_state() {
        let states = vec![SessionState::Idle, SessionState::Connecting, SessionState::Active, SessionState::Paused, SessionState::Disconnected];
        assert_eq!(states.len(), 5);
    }
    
    #[test]
    fn test_session_kind() {
        let kinds = vec![SessionKind::Desktop, SessionKind::Terminal, SessionKind::FileTransfer];
        assert_eq!(kinds.len(), 3);
    }
    
    #[test]
    fn test_profile_kind() {
        let kinds = vec![ProfileKind::Personal, ProfileKind::Work, ProfileKind::Server, ProfileKind::Custom(String::new())];
        assert_eq!(kinds.len(), 4);
    }
    
    #[test]
    fn test_connection_creation() {
        let conn = Connection::new("MyServer".into(), ProtocolKind::Rdp, "192.168.1.100".into(), 3389);
        assert_eq!(conn.name, "MyServer".to_string());
        assert_eq!(conn.protocol, ProtocolKind::Rdp);
        assert_eq!(conn.host, "192.168.1.100".to_string());
        assert_eq!(conn.port, 3389);
        assert_eq!(conn.state, ConnState::Disconnected);
    }
    
    #[test]
    fn test_connection_lifecycle() {
        let mut conn = Connection::new("server".into(), ProtocolKind::Rdp, "host".into(), 3389);
        conn.connect();
        assert_eq!(conn.state, ConnState::Connecting);
        conn.finish_connecting();
        assert_eq!(conn.state, ConnState::Connected);
        conn.disconnect();
        assert_eq!(conn.state, ConnState::Disconnected);
    }
    
    #[test]
    fn test_connection_config() {
        let mut conn = Connection::new("vnc".into(), ProtocolKind::Vnc, "host".into(), 5900);
        conn.set_config(ConnConfig {
            username: "user".into(),
            password: "pass".into(),
            resolution: "1920x1080".into(),
            fullscreen: true,
            ..Default::default()
        });
        
        assert_eq!(conn.config.username, "user".to_string());
        assert!(conn.config.fullscreen);
    }
    
    #[test]
    fn test_protocol_creation() {
        let proto = Protocol::new(ProtocolKind::Rdp);
        assert_eq!(proto.kind, ProtocolKind::Rdp);
        assert_eq!(proto.state, ProtocolState::Idle);
    }
    
    #[test]
    fn test_protocol_info() {
        let mut proto = Protocol::new(ProtocolKind::Vnc);
        proto.set_info(ProtocolInfo {
            default_port: 5900,
            supports_encryption: false,
            supports_compression: true,
            max_resolution: "4096x2160".into(),
            ..Default::default()
        });
        
        assert_eq!(proto.info.default_port, 5900);
        assert!(!proto.info.supports_encryption);
        assert!(proto.info.supports_compression);
    }
    
    #[test]
    fn test_protocol_lifecycle() {
        let mut proto = Protocol::new(ProtocolKind::Rdp);
        proto.activate();
        assert_eq!(proto.state, ProtocolState::Active);
        proto.deactivate();
        assert_eq!(proto.state, ProtocolState::Idle);
    }
    
    #[test]
    fn test_session_creation() {
        let session = RemoteSession::new("session1".into(), SessionKind::Desktop);
        assert_eq!(session.id, "session1".to_string());
        assert_eq!(session.kind, SessionKind::Desktop);
        assert_eq!(session.state, SessionState::Idle);
    }
    
    #[test]
    fn test_session_lifecycle() {
        let mut session = RemoteSession::new("s1".into(), SessionKind::Terminal);
        session.start();
        assert_eq!(session.state, SessionState::Connecting);
        session.finish_connecting();
        assert_eq!(session.state, SessionState::Active);
        session.pause();
        assert_eq!(session.state, SessionState::Paused);
        session.resume();
        assert_eq!(session.state, SessionState::Active);
        session.disconnect();
        assert_eq!(session.state, SessionState::Disconnected);
    }
    
    #[test]
    fn test_session_stats() {
        let mut session = RemoteSession::new("s1".into(), SessionKind::Desktop);
        session.set_stats(SessionStats::new(1024 * 1024 * 10, 1024 * 1024 * 5, 3600));
        
        assert_eq!(session.stats.bytes_sent, 1024 * 1024 * 10);
        assert_eq!(session.stats.bytes_recv, 1024 * 1024 * 5);
        assert_eq!(session.stats.duration_secs, 3600);
    }
    
    #[test]
    fn test_profile_creation() {
        let profile = Profile::new("Work Profile".into(), ProfileKind::Work);
        assert_eq!(profile.name, "Work Profile".to_string());
        assert_eq!(profile.kind, ProfileKind::Work);
        assert_eq!(profile.state, ProfileState::Active);
    }
    
    #[test]
    fn test_profile_info() {
        let mut profile = Profile::new("Home".into(), ProfileKind::Personal);
        profile.set_info(ProfileInfo {
            description: "Home connections".into(),
            auto_connect: true,
            favorite: true,
            last_used: None,
            ..Default::default()
        });
        
        assert!(profile.info.auto_connect);
        assert!(profile.info.favorite);
    }
    
    #[test]
    fn test_profile_disable() {
        let mut profile = Profile::new("test".into(), ProfileKind::Custom("custom".into()));
        profile.disable();
        assert_eq!(profile.state, ProfileState::Disabled);
        profile.enable();
        assert_eq!(profile.state, ProfileState::Active);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = RemminaManager::new();
        assert!(manager.list_connections().is_empty());
        assert!(!manager.list_protocols().is_empty()); // 6 default protocols
        assert!(manager.list_profiles().is_empty());
    }
    
    #[test]
    fn test_add_connection() {
        let mut manager = RemminaManager::new();
        manager.initialize();
        let id = manager.add_connection("Server".into(), ProtocolKind::Rdp, "host".into(), 3389);
        assert!(manager.get_connection(&id).is_some());
    }
    
    #[test]
    fn test_connect() {
        let mut manager = RemminaManager::new();
        manager.initialize();
        let conn_id = manager.add_connection("server".into(), ProtocolKind::Vnc, "host".into(), 5900);
        
        let session_id = manager.connect(&conn_id);
        assert!(manager.get_session(&session_id).is_some());
    }
    
    #[test]
    fn test_add_profile() {
        let mut manager = RemminaManager::new();
        manager.initialize();
        let profile_id = manager.add_profile("Work".into(), ProfileKind::Work);
        assert!(manager.get_profile(&profile_id).is_some());
    }
    
    #[test]
    fn test_add_connection_to_profile() {
        let mut manager = RemminaManager::new();
        manager.initialize();
        let profile_id = manager.add_profile("Work".into(), ProfileKind::Work);
        let conn_id = manager.add_connection("server".into(), ProtocolKind::Rdp, "host".into(), 3389);
        
        assert!(manager.add_connection_to_profile(&profile_id, &conn_id));
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = RemminaManager::new();
        manager.initialize();
        manager.add_connection("s1".into(), ProtocolKind::Rdp, "h1".into(), 3389);
        manager.add_profile("Work".into(), ProfileKind::Work);
        
        let export = manager.export_data();
        let mut manager2 = RemminaManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_connections().len(), 1);
        assert_eq!(manager2.list_profiles().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = RemminaManager::new();
        manager.initialize();
        manager.add_connection("a".into(), ProtocolKind::Rdp, "h".into(), 3389);
        manager.add_connection("b".into(), ProtocolKind::Vnc, "h".into(), 5900);
        manager.add_profile("p1".into(), ProfileKind::Work);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_connections, 2);
        assert_eq!(stats.total_profiles, 1);
    }
}
