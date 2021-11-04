use macrofied_toolbox::option;

fn main() {
    inner().unwrap();

    fn inner() -> Option<usize> {
        option! {
            @when { foo()? }
            @some "will not compile";
        }
    }
}

fn foo() -> Option<usize> {
    Some(42)
}