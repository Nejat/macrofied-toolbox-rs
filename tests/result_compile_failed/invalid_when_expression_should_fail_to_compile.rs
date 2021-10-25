use macrofied_toolbox::result;

fn main() {
    result! {
        @when foo();
        @ok   "will not compile";
    }
}

fn foo() -> usize {
    42
}