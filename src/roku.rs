use std::collections::HashMap;
use url::Host;

#[derive(Debug)]
#[allow(dead_code)]
pub enum RokuKey {
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
}

impl RokuKey {
    pub fn to_str(&self) -> &'static str {
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
        }
    }
}

impl Default for RokuKey {
    fn default() -> Self {
        RokuKey::Select
    }
}

#[derive(Clone, Debug, Default)]
pub struct RokuApp {
    pub id: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct Roku {
    http: reqwest::Client,
    pub host: Host,
    pub app_list: Vec<RokuApp>,
    pub device_info: HashMap<String, String>,
}

#[allow(dead_code)]
impl Roku {
    /// Create a new Roku client
    pub fn new(host: Host) -> Roku {
        let mut r = Roku {
            host,
            http: reqwest::Client::new(),
            app_list: vec![],
            device_info: HashMap::new(),
        };

        r.fetch_device_info()
            .expect("failed to fetch device info during init!");
        r.fetch_app_list()
            .expect("failed to fetch app list during init!");

        r
    }

    /// Get the friendly name of the current device
    pub fn get_friendly_name(&self) -> &str {
        self.get_device_info("friendly-device-name")
    }

    /// Send a keypress event
    pub fn keypress(&mut self, key: RokuKey) -> Result<(), Box<reqwest::Error>>{
        let url = format!("http://{}:8060/keypress/{}", self.host, key.to_str());
        self.http.post(&url).send()?;
        Ok(())
    }

    /// Launch a specific app
    pub fn launch_app(&self, app_id: usize) -> Result<(), Box<reqwest::Error>> {
        let url = format!("http://{}:8060/launch/{}", self.host, app_id);
        self.http.post(&url).send()?;
        Ok(())
    }

    /// Get a value from the device info
    pub fn get_device_info(&self, key: &str) -> &str {
        if self.device_info.contains_key(key) {
            self.device_info.get(key).unwrap()
        } else {
            &""
        }
    }

    /// Query device info
    fn fetch_app_list(&mut self) -> Result<(), Box<reqwest::Error>> {
        let url = format!("http://{}:8060/query/apps", self.host);
        let body = self.http.get(&url).send()?
            .text()
            .expect("could not get device info response");

        self.app_list = parse_app_list(&body);
        Ok(())
    }

    /// Query device info
    fn fetch_device_info(&mut self) -> Result<(), Box<reqwest::Error>> {
        let url = format!("http://{}:8060/query/device-info", self.host);
        let body = self.http.get(&url).send()?
            .text()
            .expect("could not get device info response");

        self.device_info = parse_device_info(&body);
        Ok(())
    }
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

    list
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

    map
}
