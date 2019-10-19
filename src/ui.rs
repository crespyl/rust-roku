use std::ffi::CStr;
use qmetaobject::*;

use crate::roku::{Roku, RokuKey};
use crate::discovery;

qrc!(init_qml_resources,
     "ui" {
         "ui/remote.qml" as "remote.qml",
         "ui/triangle_red.png" as "triangle_red.png"
     }
);

/// This struct is our interface with the Qt/QML environment; we use the
/// qmetaobject macros to connect all the relevant pieces together.
#[derive(QObject, Default)]
pub struct RokuRemote {
    roku: Option<Roku>,

    base: qt_base_class!(trait QObject),
    name: qt_property!(QString; NOTIFY name_changed),

    name_changed: qt_signal!(),

    find_roku: qt_method!(fn find_roku(&mut self) {
        println!("Beginning SSDP search");
        let urls = discovery::find_roku_urls();
        for url in urls.iter() {
            println!("Found Roku at: {:?}", url.host());
        }

        if urls.len() == 0 {
            println!("Could not find a Roku!");
        }

        let roku = Roku::new(urls[0].host().unwrap().to_owned());
        println!("Connected To {}: {}",
                 roku.get_device_info("model-name"),
                 roku.get_friendly_name());

        self.name = roku.get_friendly_name().into();
        self.name_changed();
        self.roku = Some(roku);
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
    qml_register_type::<RokuRemote>(CStr::from_bytes_with_nul(b"RokuRemote\0").unwrap(),
                                    0, 1,
                                    CStr::from_bytes_with_nul(b"RokuRemote\0").unwrap());
}
