use qmetaobject::*;
use std::thread;

use crate::discovery;
use crate::roku::{Roku, RokuKey, RokuApp};

/// This struct is our interface with the Qt/QML environment; we use the
/// qmetaobject macros to connect all the relevant pieces together.
#[derive(QObject, Default)]
pub struct RokuRemote {
    base: qt_base_class!(trait QAbstractListModel),

    roku: Option<Roku>,

    name: qt_property!(QString; NOTIFY name_changed WRITE set_name),
    name_changed: qt_signal!(),

    status: qt_property!(QString; NOTIFY status_changed WRITE set_status),
    status_changed: qt_signal!(),

    apps: Vec<RokuApp>,
    count: qt_property!(usize; NOTIFY count_changed),
    count_changed: qt_signal!(),

    find_roku: qt_method!(fn (&mut self)),

    launch_app: qt_method!(fn (&mut self, app_id: usize)),
    launch_netflix: qt_method!(fn (&mut self)),
    launch_youtube: qt_method!(fn (&mut self)),
    launch_twitch: qt_method!(fn (&mut self)),

    home: qt_method!(fn (&mut self)),
    back: qt_method!(fn (&mut self)),
    select: qt_method!(fn (&mut self)),
    up: qt_method!(fn (&mut self)),
    down: qt_method!(fn (&mut self)),
    left: qt_method!(fn (&mut self)),
    right: qt_method!(fn (&mut self)),
    play: qt_method!(fn (&mut self)),
    rev: qt_method!(fn (&mut self)),
    fwd: qt_method!(fn (&mut self)),
    instant_replay: qt_method!(fn (&mut self)),
    info: qt_method!(fn (&mut self)),
}

impl RokuRemote {
    /// Property setter for name, will trigger the name_changed signal
    fn set_name<T: Into<QString>>(&mut self, name: T) {
        self.name = name.into();
        self.name_changed();
    }

    /// Property setter for status, will trigger the status_changed signal
    fn set_status<T: Into<QString>>(&mut self, status: T) {
        self.status = status.into();
        self.status_changed();
    }

    /// Property setter for the app/channel list, will trigger the count_changed signal
    fn set_apps(&mut self, apps: &[RokuApp]) {
        self.apps.clear();
        (self as &mut dyn QAbstractListModel).begin_reset_model();
        for app in apps {
            self.apps.push(app.clone());
        }
        (self as &mut dyn QAbstractListModel).end_reset_model();
        self.count = self.apps.len();
        self.count_changed();
    }

    /// Kick off a background thread to search for Roku devices using SSDP
    ///
    /// This thread will use a queued_callback to update the main thread when
    /// the search has finished.
    fn find_roku(&mut self) {
        self.set_status("Searching");

        // here we use a QPointer to wrap access to self from our callback closure
        let qptr = QPointer::from(&*self);

        // we use queued_callback to create a function that can be safely called
        // from our background thread and will still execute in the main Qt
        // thread, ensuring that we can still access self and the relevant
        // signals and setters
        let search_done = queued_callback(move |roku: Option<Roku>| {
            qptr.as_pinned().map(|self_| {
                if let Some(roku) = roku {
                    println!("Connected to {}: {}",
                             roku.get_device_info("model-name"),
                             roku.get_friendly_name());

                    self_.borrow_mut().set_name(roku.get_friendly_name());
                    self_.borrow_mut().set_apps(&roku.app_list);

                    println!("App List:\n{:#?}", self_.borrow().apps);

                    self_.borrow_mut().roku = Some(roku);
                } else {
                    println!("No devices found!");
                    self_.borrow_mut().set_name("<Not Connected>");
                }

                self_.borrow_mut().set_status("Idle");
            });
        });

        thread::spawn(move || {
            // this is a slow blocking call, since we have to wait for some time
            // so devices have a chance to respond
            let found = discovery::find_roku_urls();

            if !found.is_empty() {
                let roku = Roku::new(found[0].host().unwrap().to_owned());
                search_done(Some(roku));
            } else {
                search_done(None);
            }
        });
    }

    fn launch_app(&mut self, app_id: usize) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .launch_app(app_id)
                .expect("err launching app")
        }
    }

    fn launch_netflix(&mut self) {
        self.launch_app(12)
    }

    fn launch_youtube(&mut self) {
        self.launch_app(837)
    }

    fn launch_twitch(&mut self) {
        self.launch_app(50539)
    }

    fn home(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Home)
                .expect("err sending keypress");
        }
    }

    fn back(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Back)
                .expect("err sending keypress");
        }
    }

    fn select(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Select)
                .expect("err sending keypress");
        }
    }

    fn up(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Up)
                .expect("err sending keypress");
        }
    }

    fn down(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Down)
                .expect("err sending keypress");
        }
    }

    fn left(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Left)
                .expect("err sending keypress");
        }
    }

    fn right(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Right)
                .expect("err sending keypress");
        }
    }

    fn play(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Play)
                .expect("err sending keypress");
        }
    }

    fn rev(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Rev)
                .expect("err sending keypress");
        }
    }

    fn fwd(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Fwd)
                .expect("err sending keypress");
        }
    }

    fn instant_replay(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::InstantReplay)
                .expect("err sending keypress");
        }
    }

    fn info(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Info)
                .expect("err sending keypress");
        }
    }
}

/// Make RokuRemote a ListModel, with each available app/channel being an item
/// row_count does what it says on the tin, data fetches the value of a
/// particular "role" for the given index, and role_names maps internal role ids
/// to QML-side variable names
impl QAbstractListModel for RokuRemote {
    fn row_count(&self) -> i32 {
        self.apps.len() as i32
    }

    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let idx = index.row() as usize;
        if idx < self.apps.len() {
            if role == USER_ROLE { // name
                QString::from(self.apps[idx].name.clone()).into()
            } else if role == USER_ROLE + 1 { // appId
                ((self.apps[idx].id) as i32).into()
            } else if role == USER_ROLE + 2 { // colorCode
                use rand::prelude::*;

                // TODO use actual channel icons
                let (r,g,b): (u8,u8,u8) = (rand::thread_rng().gen_range(0,255),
                                           rand::thread_rng().gen_range(0,255),
                                           rand::thread_rng().gen_range(0,255));
                let s = format!("#{:02x}{:02x}{:02x}",r,g,b);
                QString::from(s).into()
            } else {
                QVariant::default()
            }
        } else {
            QVariant::default()
        }
    }

    fn role_names(&self) -> std::collections::HashMap<i32, QByteArray> {
        let mut map = std::collections::HashMap::new();
        map.insert(USER_ROLE, "name".into());
        map.insert(USER_ROLE+1, "appId".into());
        map.insert(USER_ROLE+2, "colorCode".into());
        map
    }
}
