// compile_fail: t!() requires key to exist in en bundle; unknown key must fail at compile time.
use warp_i18n::t;

fn main() {
    let _label = t!("this-key-does-not-exist-in-any-bundle-xyz"); //~ ERROR
}
