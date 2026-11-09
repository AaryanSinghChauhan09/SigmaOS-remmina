//! Command-line options for remmina.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// Connect mode
    Connect(String),
    /// List mode
    List,
    /// Edit mode
    Edit(String),
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Connection name
    pub connection: String,
    /// Protocol
    pub protocol: String,
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Username
    pub username: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            connection: String::new(),
            protocol: "rdp".into(),
            host: String::new(),
            port: 0,
            username: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
