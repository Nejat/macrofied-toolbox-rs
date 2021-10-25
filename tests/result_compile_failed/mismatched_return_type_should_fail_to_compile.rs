use macrofied_toolbox::result;

fn main() -> Result<(), String> {
    result! {
        @when foo()?;
        @ok   "will not compile";
    }

    Ok(())
}

fn foo() -> Result<usize, &'static str> {
    Ok(42)
}