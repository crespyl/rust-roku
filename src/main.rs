extern crate reqwest;
extern crate roxmltree;

use std::net::{IpAddr};
use std::collections::HashMap;

#[derive(Debug)]
enum RokuKey {
    Home,
    Rev,
    Fwd,
    Play,
    Select,
    Left,
    Right,
    Down,
    Up,
    Back,
    InstantReplay,
    Info,
    Backspace,
    Search,
    Enter,
    Literal(char),
    Channel(usize),
}

impl RokuKey {
    fn to_str(&self) -> &'static str {
        match *self {
            RokuKey::Home => "Home",
            RokuKey::Rev => "Rev",
            RokuKey::Fwd => "Fwd",
            RokuKey::Play => "Play",
            RokuKey::Select => "Select",
            RokuKey::Left => "Left",
            RokuKey::Right => "Right",
            RokuKey::Down => "Down",
            RokuKey::Up => "Up",
            RokuKey::Back => "Back",
            RokuKey::InstantReplay => "InstantReplay",
            RokuKey::Info => "Info",
            RokuKey::Backspace => "Backspace",
            RokuKey::Search => "Search",
            RokuKey::Enter => "Enter",
            _ => "Home"
        }
    }
}

#[derive(Debug)]
struct RokuApp {
    id: usize,
    name: String,
}

#[derive(Debug)]
struct Roku {
    http: reqwest::Client,
    server_ip: std::net::IpAddr,
    app_list: Vec<RokuApp>,
    device_info: HashMap<String, String>,
    got_app_list: bool,
    got_device_info: bool,
}

impl Roku {
    /// Create a new Roku client
    pub fn new(server_ip: IpAddr) -> Roku {
        let mut r = Roku {
            http: reqwest::Client::new(),
            server_ip: server_ip,
            app_list: vec![],
            device_info: HashMap::new(),
            got_app_list: false,
            got_device_info: false,
        };

        r.fetch_device_info()
            .expect("failed to fetch device info during init!");

        r.fetch_app_list()
            .expect("failed to fetch app list during init!");

        return r;
    }

    /// Send a keypress event
    pub fn keypress(&mut self, key: RokuKey) -> Result<(), Box<reqwest::Error>>{
        let url = format!("http://{}:8060/keypress/{}", self.server_ip, key.to_str());
        self.http.post(&url).send()?;
        Ok(())
    }

    /// Launch a specific app
    pub fn launch_app(&self, app_id: usize) -> Result<(), Box<reqwest::Error>> {
        let url = format!("http://{}:8060/launch/{}", self.server_ip, app_id);
        self.http.post(&url).send()?;
        Ok(())
    }

    /// Get a value from the device info
    pub fn get_device_info(&self, key: &str) -> &str {
        if self.device_info.contains_key(key) {
            return self.device_info.get(key).unwrap()
        } else {
            return &"";
        }
    }

    /// Query device info
    fn fetch_app_list(&mut self) -> Result<(), Box<reqwest::Error>> {
        let url = format!("http://{}:8060/query/apps", self.server_ip);
        let body = self.http.get(&url).send()?
            .text()
            .expect("could not get device info response");

        self.app_list = parse_app_list(&body);
        Ok(())
    }

    /// Query device info
    fn fetch_device_info(&mut self) -> Result<(), Box<reqwest::Error>> {
        let url = format!("http://{}:8060/query/device-info", self.server_ip);
        let body = self.http.get(&url).send()?
            .text()
            .expect("could not get device info response");

        self.device_info = parse_device_info(&body);
        self.got_device_info = true;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut roku = Roku::new("192.168.1.129".parse().unwrap());
    println!("Connected To {}: {}",
             roku.get_device_info("model-name"),
             roku.get_device_info("friendly-device-name"));

    //roku.keypress(RokuKey::Home).unwrap();

    println!("Got App List:");
    for app in roku.app_list {
        println!("{:.<25}:{}", app.name, app.id);
    }

    Ok(())
}

fn parse_app_list(xml: &str) -> Vec<RokuApp> {
    let doc = roxmltree::Document::parse(xml).unwrap();
    let root = doc.root().first_child().unwrap();
    let mut list = vec![];

    for node in root.children() {
        if node.is_element() &&
            node.tag_name().name() == "app" &&
            node.attribute("type") == Some("appl")
        {
            list.push(RokuApp {
                id: node.attribute("id").expect("bad app id!").parse().unwrap(),
                name: node.text().expect("bad app name!").into(),
            });
        }
    }

    return list;
}
fn parse_device_info(xml: &str) -> HashMap<String, String> {
    let doc = roxmltree::Document::parse(xml).unwrap();
    let root = doc.root().first_child().unwrap();
    let mut map = HashMap::new();

    for node in root.children() {
        if node.is_element() {
            map.insert(node.tag_name().name().into(),
                       node.text().unwrap_or("").into());
        }
    }

    return map;
}
