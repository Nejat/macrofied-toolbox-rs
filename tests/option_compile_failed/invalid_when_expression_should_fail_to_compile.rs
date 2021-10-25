use macrofied_toolbox::option;

fn main() {
    option! {
        @when foo();
        @some "will not compile";
    }
}

fn foo() -> usize {
    42
}