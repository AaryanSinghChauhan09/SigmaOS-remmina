//! remmina model — remote desktop client.

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

    pub fn add_connection(&mut self, conn: Connection) { self.connections.push(conn); }

    pub fn connect(&mut self, name: &str) -> bool {
        if let Some(c) = self.connections.iter_mut().find(|c| c.name == name) {
            c.port = c.port.max(1);
            true
        } else { false }
    }

    pub fn disconnect(&mut self, name: &str) -> bool {
        self.connections.iter().any(|c| c.name == name)
    }

    pub fn list(&self) -> &[Connection] { &self.connections }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn connection_connect() {
        let mut r = Model::new();
        r.add_connection(Connection { name: "work".into(), protocol: "rdp".into(), host: "work.example.com".into(), port: 3389, username: "alice".into() });
        assert!(r.connect("work"));
        assert!(r.disconnect("work"));
    }
}
