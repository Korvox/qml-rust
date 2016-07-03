#[macro_use]
extern crate qml;

use qml::*;

struct Test;

impl Test {
    pub fn zapoostitGoosya(&self, i: i32, i2: i32) {}
}

Q_OBJECT!(
Test:
    signals:
         fn testname (a: i32, b: i32);

    slots:
         fn zapoostitGoosya(i: i32, i2: i32);
);

fn main() {
    let mut test = Test;
    test.testname(54, 55);
    test.qmeta_slots("zapoostitGoosya", vec![5.into(), 6.into()]);
}