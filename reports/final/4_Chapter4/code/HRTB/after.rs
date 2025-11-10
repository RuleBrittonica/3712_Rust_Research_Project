fn apply_str<F>(f: F) -> &'static str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    call_higher(f)
}

fn call_higher<F>(f: F) -> &'static str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = "hello";
    f(s)
}
