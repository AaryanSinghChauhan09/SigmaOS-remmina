//! Utility model for remmina (legacy compat).

#[derive(Debug, Clone)]
pub struct Connection {
    pub name: String,
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub connections: Vec<Connection>,
}

impl Default for Model {
    fn default() -> Self { Self { connections: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_connection(&mut self, name: &str, protocol: &str, host: &str, port: u16, username: &str) {
        self.connections.push(Connection {
            name: name.into(), protocol: protocol.into(),
            host: host.into(), port, username: username.into(),
        });
    }

    pub fn list_connections(&self) -> &[Connection] { &self.connections }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_model() {
        let mut m = Model::new();
        m.add_connection("srv", "rdp", "host", 3389, "user");
        assert_eq!(m.connections.len(), 1);
    }
}
