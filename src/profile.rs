//! Profile definitions for remmina.

/// Profile kind
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileKind {
    /// Personal profile
    Personal,
    /// Work profile
    Work,
    /// Server profile
    Server,
    /// Custom profile
    Custom(String),
}

/// Profile state
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileState {
    /// Active profile
    Active,
    /// Disabled profile
    Disabled,
}

/// Profile info
#[derive(Debug, Clone, Default)]
pub struct ProfileInfo {
    /// Description
    pub description: String,
    /// Auto-connect
    pub auto_connect: bool,
    /// Favorite
    pub favorite: bool,
    /// Last used
    pub last_used: Option<u64>,
    /// Connection IDs
    pub connection_ids: Vec<String>,
}

/// Profile
#[derive(Debug, Clone)]
pub struct Profile {
    /// Profile ID
    pub id: String,
    /// Profile name
    pub name: String,
    /// Profile kind
    pub kind: ProfileKind,
    /// Profile state
    pub state: ProfileState,
    /// Profile info
    pub info: ProfileInfo,
    /// Created timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub last_modified: u64,
}

impl Profile {
    /// Create new profile
    pub fn new(name: String, kind: ProfileKind) -> Self {
        Self {
            id: format!("profile-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            kind,
            state: ProfileState::Active,
            info: ProfileInfo::default(),
            created_at: Self::now(),
            last_modified: Self::now(),
        }
    }
    
    /// Set info
    pub fn set_info(&mut self, info: ProfileInfo) {
        self.info = info;
        self.last_modified = Self::now();
    }
    
    /// Add connection to profile
    pub fn add_connection(&mut self, connection_id: String) -> bool {
        if !self.info.connection_ids.contains(&connection_id) {
            self.info.connection_ids.push(connection_id);
            self.last_modified = Self::now();
            return true;
        }
        false
    }
    
    /// Remove connection from profile
    pub fn remove_connection(&mut self, connection_id: &str) -> bool {
        if let Some(pos) = self.info.connection_ids.iter().position(|c| c == connection_id) {
            self.info.connection_ids.remove(pos);
            self.last_modified = Self::now();
            return true;
        }
        false
    }
    
    /// Check if has connection
    pub fn has_connection(&self, connection_id: &str) -> bool {
        self.info.connection_ids.contains(&connection_id.to_string())
    }
    
    /// Get connection count
    pub fn get_connection_count(&self) -> usize {
        self.info.connection_ids.len()
    }
    
    /// Disable profile
    pub fn disable(&mut self) {
        self.state = ProfileState::Disabled;
        self.last_modified = Self::now();
    }
    
    /// Enable profile
    pub fn enable(&mut self) {
        self.state = ProfileState::Active;
        self.last_modified = Self::now();
    }
    
    /// Mark as used
    pub fn mark_used(&mut self) {
        self.info.last_used = Some(Self::now());
        self.last_modified = Self::now();
    }
    
    /// Check if active
    pub fn is_active(&self) -> bool {
        self.state == ProfileState::Active
    }
    
    /// Check if favorite
    pub fn is_favorite(&self) -> bool {
        self.info.favorite
    }
    
    /// Get profile age in seconds
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
