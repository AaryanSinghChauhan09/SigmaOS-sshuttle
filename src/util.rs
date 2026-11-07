//! sshuttle model — transparent proxy over SSH.

#[derive(Debug, Clone)]
pub struct Tunnel {
    pub id: String,
    pub host: String,
    pub exclude: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub tunnels: Vec<Tunnel>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { tunnels: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, host: &str, exclude: &[String]) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.tunnels.push(Tunnel { id: id.clone(), host: host.into(), exclude: exclude.to_vec(), status: "active".into() });
        id
    }

    pub fn delete(&mut self, id: &str) -> bool {
        if let Some(pos) = self.tunnels.iter().position(|t| t.id == id) {
            self.tunnels.remove(pos);
            true
        } else { false }
    }

    pub fn list(&self) -> &[Tunnel] { &self.tunnels }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tunnel_create_delete() {
        let mut s = Model::new();
        let id = s.create("server.example.com", &["10.0.0.0/8".into()]);
        assert!(s.delete(&id));
    }
}
