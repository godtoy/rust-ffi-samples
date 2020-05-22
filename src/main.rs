#[macro_use]
extern crate shared_library;

use std::path::Path;

shared_library!(Test1,
    pub fn RunDragonBot(),
);

fn main() {
    let test1 = Test1::open(Path::new("dragon.vmp.dll")).unwrap();
    unsafe { (test1.RunDragonBot)() };
}
