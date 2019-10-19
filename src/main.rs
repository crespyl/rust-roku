extern crate url;
extern crate ssdp;
extern crate reqwest;
extern crate roxmltree;
extern crate qmetaobject;

use qmetaobject::*;

mod ui;
mod roku;
mod discovery;

fn main() {
    ui::init_ui();
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/ui/remote.qml".into());
    engine.exec();
}
