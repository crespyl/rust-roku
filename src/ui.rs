use qmetaobject::*;
use std::ffi::CStr;
use std::thread;

use crate::discovery;
use crate::roku::{Roku, RokuKey};

/// This struct is our interface with the Qt/QML environment; we use the
/// qmetaobject macros to connect all the relevant pieces together.
#[derive(QObject, Default)]
pub struct RokuRemote {
    roku: Option<Roku>,

    base: qt_base_class!(trait QObject),
    name: qt_property!(QString; NOTIFY name_changed WRITE set_name),
    status: qt_property!(QString; NOTIFY status_changed WRITE set_status),

    name_changed: qt_signal!(),
    status_changed: qt_signal!(),


    /// Kick off a background thread to search for Roku devices using SSDP
    ///
    /// This thread will use a queued_callback to update the main thread when
    /// the search has finished.
    find_roku: qt_method!(fn find_roku(&mut self) {
        self.set_status("Searching");

        // here we use a QPointer to wrap access to self from our callback closure
        let qptr = QPointer::from(&*self);

        // we use queued_callback to create a function that can be safely called
        // from our background thread and will still execute in the main Qt
        // thread, ensuring that we can still access self and the relevant
        // signals and setters
        let search_done = queued_callback(move |roku: Option<Roku>| {
            qptr.as_pinned().map(|self_| {
                match roku {
                    Some(roku) => {
                        println!("Connected to {}: {}",
                                 roku.get_device_info("model-name"),
                                 roku.get_friendly_name());

                        println!("App List:\n{:#?}", roku.app_list);

                        self_.borrow_mut().set_name(roku.get_friendly_name());
                        self_.borrow_mut().roku = Some(roku);
                    },
                    None => {
                        println!("No devices found!");
                        self_.borrow_mut().set_name("<Not Connected>");
                    }
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
    }),

    launch_netflix: qt_method!(fn launch_netflix(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .launch_app(12)
                .expect("err launching app")
        }
    }),

    launch_youtube: qt_method!(fn launch_youtube(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .launch_app(837)
                .expect("err launching app")
        }
    }),

    launch_twitch: qt_method!(fn launch_twitch(&mut self) {
        if self.roku.is_some() {
            self.roku
                .as_mut()
                .unwrap()
                .launch_app(50539)
                .expect("err launching app")
        }
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
}

qrc!(init_qml_resources,
     "ui" {
         "ui/remote.qml" as "remote.qml",
     }
);

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
