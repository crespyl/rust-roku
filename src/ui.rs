use qmetaobject::*;
use url::Url;
use std::ffi::CStr;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};

use crate::discovery;
use crate::roku::{Roku, RokuKey};

qrc!(init_qml_resources,
     "ui" {
         "ui/remote.qml" as "remote.qml",
     }
);

/// This struct is our interface with the Qt/QML environment; we use the
/// qmetaobject macros to connect all the relevant pieces together.
#[derive(QObject, Default)]
pub struct RokuRemote {
    roku: Option<Roku>,
    urls: Arc<Mutex<Vec<Url>>>,
    query_running: Arc<AtomicBool>,

    base: qt_base_class!(trait QObject),
    name: qt_property!(QString; NOTIFY name_changed),
    status: qt_property!(QString; NOTIFY status_changed),

    name_changed: qt_signal!(),
    status_changed: qt_signal!(),

    /// This method polls the status of the worker and if the worker is stopped,
    /// will connect to the first found device.
    ///
    /// This method gets called by a Timer from the QML side, since I'm not sure
    /// how to safely trigger a signal or event on self from a rust thread.
    check_search: qt_method!(fn check_search(&mut self) {
        if self.query_running.load(Ordering::Relaxed) {
            self.status = "Searching".into();
            self.status_changed();
        } else {
            self.status = "Idle".into();
            self.status_changed();

            let urls = self.urls.lock().unwrap();
            if !urls.is_empty() {
                let roku = Roku::new(urls[0].host().unwrap().to_owned());

                println!("Connected to {}: {}",
                         roku.get_device_info("model-name"),
                         roku.get_friendly_name());

                self.name = roku.get_friendly_name().into();
                self.name_changed();

                self.roku = Some(roku);
            } else {
                println!("No devices found!");
                self.name = "<Not Connected>".into();
                self.name_changed();
            }
        }
    }),

    /// Kick off a background thread to search for Roku devices using SSDP
    ///
    /// This thread will use the atomic query_running to indicate whether it's
    /// still running or not, and will use the urls Mutex to guard access to the
    /// list
    find_roku: qt_method!(fn find_roku(&mut self) {
        self.status = "Searching".into();
        self.status_changed();

        let _running = Arc::clone(&self.query_running);
        let _urls = Arc::clone(&self.urls);
        thread::spawn(move || {
            _running.store(true, Ordering::Relaxed);

            // this is a slow blocking call, since we have to wait for some time
            // so devices have a chance to respond
            let mut found = discovery::find_roku_urls();

            let mut urls = _urls.lock().unwrap();
            urls.clear();
            urls.append(&mut found);

            _running.store(false, Ordering::Relaxed)
        });
    }),

    home: qt_method!(fn home(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Home)
                .expect("err sending keypress");
        }
    }),

    back: qt_method!(fn back(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Back)
                .expect("err sending keypress");
        }
    }),

    select: qt_method!(fn select(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Select)
                .expect("err sending keypress");
        }
    }),

    up: qt_method!(fn up(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Up)
                .expect("err sending keypress");
        }
    }),

    down: qt_method!(fn down(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Down)
                .expect("err sending keypress");
        }
    }),

    left: qt_method!(fn left(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Left)
                .expect("err sending keypress");
        }
    }),

    right: qt_method!(fn right(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Right)
                .expect("err sending keypress");
        }
    }),

    play: qt_method!(fn play(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Play)
                .expect("err sending keypress");
        }
    }),

    rev: qt_method!(fn rev(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Rev)
                .expect("err sending keypress");
        }
    }),

    fwd: qt_method!(fn fwd(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Fwd)
                .expect("err sending keypress");
        }
    }),

    instant_replay: qt_method!(fn instant_replay(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::InstantReplay)
                .expect("err sending keypress");
        }
    }),

    info: qt_method!(fn info(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .keypress(RokuKey::Info)
                .expect("err sending keypress");
        }
    }),
}

/// This function initializes the qrc resources (QML files, images, etc) and
/// registers our RokuRemote type so that we can call its methods from QML.
pub fn init_ui() {
    init_qml_resources();
    qml_register_type::<RokuRemote>(
        CStr::from_bytes_with_nul(b"RokuRemote\0").unwrap(),
        0,
        1,
        CStr::from_bytes_with_nul(b"RokuRemote\0").unwrap(),
    );
}
