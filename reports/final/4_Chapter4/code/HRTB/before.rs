fn apply_str<F>(f: F) -> &'static str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = "hello";
    // EXTRACT START
    f(s)
    // EXTRACT END
}
