extern crate url;
extern crate rand;
extern crate ssdp;
extern crate reqwest;
extern crate roxmltree;
extern crate qmetaobject;

use qmetaobject::*;

use std::ffi::CStr;

mod ui;
mod roku;
mod discovery;

// qrc!(init_qml_resources,
//      "ui" {
//          "ui/remote.qml" as "remote.qml",
//          "ui/RemoteTab.qml" as "RemoteTab.qml",
//          "ui/ChannelTab.qml" as "ChannelTab.qml",
//      }
// );

/// This function initializes the qrc resources (QML files, images, etc) and
/// registers our RokuRemote type so that we can call its methods from QML.
pub fn init_ui() {
    //init_qml_resources();
    qml_register_type::<ui::RokuRemote>(
        CStr::from_bytes_with_nul(b"RokuRemote\0").unwrap(),
        0, 1,
        CStr::from_bytes_with_nul(b"RokuRemote\0").unwrap(),
    );
}

fn main() {
    init_ui();
    let mut engine = QmlEngine::new();
    // engine.load_file("qrc:/ui/remote.qml".into());
    engine.load_file("ui/remote.qml".into());
    engine.exec();
}

