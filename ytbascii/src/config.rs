extern crate reqwest;
extern crate serde;
extern crate chrono;

use futures::executor::block_on;

use serde::{Deserialize, Serialize};
use serde_json::json;

use std::io::Write;

// Random
use rand::Rng;

use crate::{defaults, get_logger};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Server {
  pub url: String,
  last_checked: String,
  status: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ServerList {
  servers: Vec<Server>,
}

#[derive(Clone)]
pub struct InvidiousConfig {
  config_path: String,
  servers: ServerList,
}

pub enum ConfigType {
  Invidious,
}

pub enum Configs {
  Invidious(InvidiousConfig),
}

impl Server {
  fn new(url: String, last_checked: String, status: String) -> Server {
    Server {
      url: url,
      last_checked: last_checked,
      status: status,
    }
  }
  async fn check_status(&mut self) {
    let client = reqwest::Client::new();
    let url = format!("{}/api/v1/stats", self.url);
    let response = client.get(url).send().await;
    let status = match response {
      Ok(_) => "online",
      Err(_) => "offline",
    };
    self.status = String::from(status);
    let now = chrono::offset::Utc::now();
    // write the date in %Y-%m-%d %H:%M:%S format
    self.last_checked = now.format("%Y-%m-%d %H:%M:%S").to_string();

    // Log status in green if online, red if offline, yellow if unknown
    match status {
      "online" => get_logger().success(format!("{} is online", self.url)),
      "offline" => get_logger().error(format!("{} is offline", self.url)),
      _ => get_logger().warn(format!("{} is in an unknown state", self.url)),
    }
  }
}

impl ServerList {
  fn new() -> ServerList {
    ServerList {
      servers: vec![],
    }
  }
  fn add_server(&mut self, server: Server) {
    self.servers.push(server);
  }
  fn check_status(&mut self) {
    // Check if the last check was more than 1 hour ago
    let now = chrono::offset::Utc::now().naive_utc();
    get_logger().debug(format!("Now: {}", now));
    get_logger().debug(format!("Last checked: {}", self.servers[0].last_checked));
    // i need to parse this date in rust : "last_checked": "2023-10-26 12:10:01.159590610 UTC",
    let last_checked = chrono::NaiveDateTime::parse_from_str(self.servers[0].last_checked.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
    let duration = now.signed_duration_since(last_checked);
    if duration.num_hours() < 1 {
      get_logger().info("Servers were checked less than 1 hour ago".to_string());
      return;
    }

    get_logger().info("Checking status of servers".to_string());
    let server_count = self.servers.len();
    let mut index = 0;
    for server in &mut self.servers {
      index += 1;
      block_on(server.check_status());
      get_logger().info(format!("Checked status of server {}/{}", index, server_count));
    }
  }
  fn get_online_servers(&self) -> Vec<&Server> {
    let mut online_servers: Vec<&Server> = vec![];
    for server in &self.servers {
      if server.status == "online" {
        online_servers.push(server);
      }
    }
    online_servers
  }
  fn get_random_server(&self) -> &Server {
    let online_servers = self.get_online_servers();
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..online_servers.len());
    online_servers[random_index]
  }
}

pub trait Config: Send + Sync {
  fn set_config_path(&mut self, path: String);
  fn get_config_path(&self) -> String;
  fn load(&mut self);
  fn save(&self);
  fn exists(&self) -> bool;
}

impl InvidiousConfig {
  pub fn new(path: String) -> InvidiousConfig {
    let mut config = InvidiousConfig {
      config_path: String::from(path.clone()),
      servers: ServerList::new(),
    };

    if !std::path::Path::new(config.config_path.as_str()).exists() {
      get_logger().info(format!("Creating config file at {}", path.as_str()));
      let mut config_file = std::fs::File::create(path.as_str()).unwrap();
      write!(config_file, "{}", defaults::DEFAULT_INVIDIOUS_CONFIG).unwrap();
      config.set_config_path(path);
    }

    config
  }
  pub fn get_random_server(&self) -> &Server {
    self.servers.get_random_server()
  }
  pub fn check_status(&mut self) {
    self.servers.check_status();
  }
}

impl Config for InvidiousConfig {
  fn exists(&self) -> bool {
    let config_path = self.get_config_path();
    std::path::Path::new(&config_path).exists()
  }
  fn set_config_path(&mut self, path: String) {
    self.config_path = path;
  }
  fn get_config_path(&self) -> String {
    self.config_path.clone()
  }
  fn load(&mut self) {
    get_logger().info(format!("Loading config file at {}", self.get_config_path()));
    let config_path = self.get_config_path();
    let config_file = std::fs::File::open(config_path).unwrap();
    let config: serde_json::Value = serde_json::from_reader(config_file).unwrap();
    let servers = config["servers"].as_array().unwrap();
    for server in servers {
      let url = server["url"].as_str().unwrap();
      let last_checked = server["last_checked"].as_str().unwrap();
      let status = server["status"].as_str().unwrap();
      self.servers.add_server(Server::new(String::from(url), String::from(last_checked), String::from(status)));
    }
  }
  fn save(&self) {
    get_logger().info(format!("Saving config file at {}", self.get_config_path()));
    let config_path = self.get_config_path();
    let mut config_file = std::fs::File::create(config_path).unwrap();
    // Write the config file with %Y-%m-%d %H:%M:%S format
    let mut servers: Vec<serde_json::Value> = vec![];
    for server in &self.servers.servers {
      let server_json = json!({
        "url": server.url,
        "last_checked": server.last_checked,
        "status": server.status,
      });
      servers.push(server_json);
    }
    let config = json!({
      "servers": servers,
    });
    write!(config_file, "{}", config.to_string()).unwrap();
  }
}
