use macrofied_toolbox::result;

fn main() {
    inner().unwrap();

    fn inner() -> Result<usize, ()> {
        result! {
            @when { foo()? }
            @ok   "will not compile";
        }
    }
}

fn foo() -> Result<usize, ()> {
    Ok(42)
}