use macrofied_toolbox::result;

fn main() {
    let value = result! {
        @ok     "This will fail: {}", 42
        @error return
    };

    assert_eq!(42, value);
}