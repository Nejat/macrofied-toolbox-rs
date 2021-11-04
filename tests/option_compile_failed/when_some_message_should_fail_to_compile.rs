use macrofied_toolbox::option;

fn main() {
    let value = option! {
        @some "This will fail: {}", 42
        @none return
    };

    assert_eq!(42, value);
}