fn bar <'a 'b> (x: &'a i32, y: &'b i32)
    -> &'b i32 where 'a: 'b {...}