    #[test]
    fn no_args_from_binary_expr() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    foo($01 + 1$0);
}
"#,
            r#"
fn foo() {
    foo(fun_name());
}

fn $0fun_name() -> i32 {
    1 + 1
}
"#,
        );
    }

    #[test]
    fn no_args_from_binary_expr_in_module() {
        check_assist(
            extract_function,
            r#"
mod bar {
    fn foo() {
        foo($01 + 1$0);
    }
}
"#,
            r#"
mod bar {
    fn foo() {
        foo(fun_name());
    }

    fn $0fun_name() -> i32 {
        1 + 1
    }
}
"#,
        );
    }

    #[test]
    fn no_args_from_binary_expr_indented() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    $0{ 1 + 1 }$0;
}
"#,
            r#"
fn foo() {
    fun_name();
}

fn $0fun_name() -> i32 {
    1 + 1
}
"#,
        );
    }

    #[test]
    fn no_args_from_stmt_with_last_expr() {
        check_assist(
            extract_function,
            r#"
fn foo() -> i32 {
    let k = 1;
    $0let m = 1;
    m + 1$0
}
"#,
            r#"
fn foo() -> i32 {
    let k = 1;
    fun_name()
}

fn $0fun_name() -> i32 {
    let m = 1;
    m + 1
}
"#,
        );
    }

    #[test]
    fn no_args_from_stmt_unit() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let k = 3;
    $0let m = 1;
    let n = m + 1;$0
    let g = 5;
}
"#,
            r#"
fn foo() {
    let k = 3;
    fun_name();
    let g = 5;
}

fn $0fun_name() {
    let m = 1;
    let n = m + 1;
}
"#,
        );
    }

    #[test]
    fn no_args_if() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    $0if true { }$0
}
"#,
            r#"
fn foo() {
    fun_name();
}

fn $0fun_name() {
    if true { }
}
"#,
        );
    }

    #[test]
    fn no_args_if_else() {
        check_assist(
            extract_function,
            r#"
fn foo() -> i32 {
    $0if true { 1 } else { 2 }$0
}
"#,
            r#"
fn foo() -> i32 {
    fun_name()
}

fn $0fun_name() -> i32 {
    if true { 1 } else { 2 }
}
"#,
        );
    }

    #[test]
    fn no_args_if_let_else() {
        check_assist(
            extract_function,
            r#"
fn foo() -> i32 {
    $0if let true = false { 1 } else { 2 }$0
}
"#,
            r#"
fn foo() -> i32 {
    fun_name()
}

fn $0fun_name() -> i32 {
    if let true = false { 1 } else { 2 }
}
"#,
        );
    }

    #[test]
    fn no_args_match() {
        check_assist(
            extract_function,
            r#"
fn foo() -> i32 {
    $0match true {
        true => 1,
        false => 2,
    }$0
}
"#,
            r#"
fn foo() -> i32 {
    fun_name()
}

fn $0fun_name() -> i32 {
    match true {
        true => 1,
        false => 2,
    }
}
"#,
        );
    }

    #[test]
    fn no_args_while() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    $0while true { }$0
}
"#,
            r#"
fn foo() {
    fun_name();
}

fn $0fun_name() {
    while true { }
}
"#,
        );
    }

    #[test]
    fn no_args_for() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    $0for v in &[0, 1] { }$0
}
"#,
            r#"
fn foo() {
    fun_name();
}

fn $0fun_name() {
    for v in &[0, 1] { }
}
"#,
        );
    }

    #[test]
    fn no_args_from_loop_unit() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    $0loop {
        let m = 1;
    }$0
}
"#,
            r#"
fn foo() {
    fun_name()
}

fn $0fun_name() -> ! {
    loop {
        let m = 1;
    }
}
"#,
        );
    }

    #[test]
    fn no_args_from_loop_with_return() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let v = $0loop {
        let m = 1;
        break m;
    }$0;
}
"#,
            r#"
fn foo() {
    let v = fun_name();
}

fn $0fun_name() -> i32 {
    loop {
        let m = 1;
        break m;
    }
}
"#,
        );
    }

    #[test]
    fn no_args_from_match() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let v: i32 = $0match Some(1) {
        Some(x) => x,
        None => 0,
    }$0;
}
"#,
            r#"
fn foo() {
    let v: i32 = fun_name();
}

fn $0fun_name() -> i32 {
    match Some(1) {
        Some(x) => x,
        None => 0,
    }
}
"#,
        );
    }

    #[test]
    fn extract_partial_block_single_line() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let n = 1;
    let mut v = $0n * n;$0
    v += 1;
}
"#,
            r#"
fn foo() {
    let n = 1;
    let mut v = fun_name(n);
    v += 1;
}

fn $0fun_name(n: i32) -> i32 {
    let mut v = n * n;
    v
}
"#,
        );
    }

    #[test]
    fn extract_partial_block() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let m = 2;
    let n = 1;
    let mut v = m $0* n;
    let mut w = 3;$0
    v += 1;
    w += 1;
}
"#,
            r#"
fn foo() {
    let m = 2;
    let n = 1;
    let (mut v, mut w) = fun_name(m, n);
    v += 1;
    w += 1;
}

fn $0fun_name(m: i32, n: i32) -> (i32, i32) {
    let mut v = m * n;
    let mut w = 3;
    (v, w)
}
"#,
        );
    }

    #[test]
    fn argument_form_expr() {
        check_assist(
            extract_function,
            r#"
fn foo() -> u32 {
    let n = 2;
    $0n+2$0
}
"#,
            r#"
fn foo() -> u32 {
    let n = 2;
    fun_name(n)
}

fn $0fun_name(n: u32) -> u32 {
    n+2
}
"#,
        )
    }

    #[test]
    fn argument_used_twice_form_expr() {
        check_assist(
            extract_function,
            r#"
fn foo() -> u32 {
    let n = 2;
    $0n+n$0
}
"#,
            r#"
fn foo() -> u32 {
    let n = 2;
    fun_name(n)
}

fn $0fun_name(n: u32) -> u32 {
    n+n
}
"#,
        )
    }

    #[test]
    fn two_arguments_form_expr() {
        check_assist(
            extract_function,
            r#"
fn foo() -> u32 {
    let n = 2;
    let m = 3;
    $0n+n*m$0
}
"#,
            r#"
fn foo() -> u32 {
    let n = 2;
    let m = 3;
    fun_name(n, m)
}

fn $0fun_name(n: u32, m: u32) -> u32 {
    n+n*m
}
"#,
        )
    }

    #[test]
    fn argument_and_locals() {
        check_assist(
            extract_function,
            r#"
fn foo() -> u32 {
    let n = 2;
    $0let m = 1;
    n + m$0
}
"#,
            r#"
fn foo() -> u32 {
    let n = 2;
    fun_name(n)
}

fn $0fun_name(n: u32) -> u32 {
    let m = 1;
    n + m
}
"#,
        )
    }

    #[test]
    fn in_comment_is_not_applicable() {
        cov_mark::check!(extract_function_in_comment_is_not_applicable);
        check_assist_not_applicable(extract_function, r"fn main() { 1 + /* $0comment$0 */ 1; }");
    }

    #[test]
    fn empty_selection_is_not_applicable() {
        cov_mark::check!(extract_function_empty_selection_is_not_applicable);
        check_assist_not_applicable(
            extract_function,
            r#"
fn main() {
    $0

    $0
}"#,
        );
    }

    #[test]
    fn part_of_expr_stmt() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    $01$0 + 1;
}
"#,
            r#"
fn foo() {
    fun_name() + 1;
}

fn $0fun_name() -> i32 {
    1
}
"#,
        );
    }

    #[test]
    fn function_expr() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    $0bar(1 + 1)$0
}
"#,
            r#"
fn foo() {
    fun_name();
}

fn $0fun_name() {
    bar(1 + 1)
}
"#,
        )
    }

    #[test]
    fn extract_from_nested() {
        check_assist(
            extract_function,
            r#"
fn main() {
    let x = true;
    let tuple = match x {
        true => ($02 + 2$0, true)
        _ => (0, false)
    };
}
"#,
            r#"
fn main() {
    let x = true;
    let tuple = match x {
        true => (fun_name(), true)
        _ => (0, false)
    };
}

fn $0fun_name() -> i32 {
    2 + 2
}
"#,
        );
    }

    #[test]
    fn param_from_closure() {
        check_assist(
            extract_function,
            r#"
fn main() {
    let lambda = |x: u32| $0x * 2$0;
}
"#,
            r#"
fn main() {
    let lambda = |x: u32| fun_name(x);
}

fn $0fun_name(x: u32) -> u32 {
    x * 2
}
"#,
        );
    }

    #[test]
    fn extract_return_stmt() {
        check_assist(
            extract_function,
            r#"
fn foo() -> u32 {
    $0return 2 + 2$0;
}
"#,
            r#"
fn foo() -> u32 {
    return fun_name();
}

fn $0fun_name() -> u32 {
    2 + 2
}
"#,
        );
    }

    #[test]
    fn does_not_add_extra_whitespace() {
        check_assist(
            extract_function,
            r#"
fn foo() -> u32 {


    $0return 2 + 2$0;
}
"#,
            r#"
fn foo() -> u32 {


    return fun_name();
}

fn $0fun_name() -> u32 {
    2 + 2
}
"#,
        );
    }

    #[test]
    fn break_stmt() {
        check_assist(
            extract_function,
            r#"
fn main() {
    let result = loop {
        $0break 2 + 2$0;
    };
}
"#,
            r#"
fn main() {
    let result = loop {
        break fun_name();
    };
}

fn $0fun_name() -> i32 {
    2 + 2
}
"#,
        );
    }

    #[test]
    fn extract_cast() {
        check_assist(
            extract_function,
            r#"
fn main() {
    let v = $00f32 as u32$0;
}
"#,
            r#"
fn main() {
    let v = fun_name();
}

fn $0fun_name() -> u32 {
    0f32 as u32
}
"#,
        );
    }

    #[test]
    fn return_not_applicable() {
        check_assist_not_applicable(extract_function, r"fn foo() { $0return$0; } ");
    }

    #[test]
    fn method_to_freestanding() {
        check_assist(
            extract_function,
            r#"
struct S;

impl S {
    fn foo(&self) -> i32 {
        $01+1$0
    }
}
"#,
            r#"
struct S;

impl S {
    fn foo(&self) -> i32 {
        fun_name()
    }
}

fn $0fun_name() -> i32 {
    1+1
}
"#,
        );
    }

    #[test]
    fn method_with_reference() {
        check_assist(
            extract_function,
            r#"
struct S { f: i32 };

impl S {
    fn foo(&self) -> i32 {
        $0self.f+self.f$0
    }
}
"#,
            r#"
struct S { f: i32 };

impl S {
    fn foo(&self) -> i32 {
        self.fun_name()
    }

    fn $0fun_name(&self) -> i32 {
        self.f+self.f
    }
}
"#,
        );
    }

    #[test]
    fn method_with_mut() {
        check_assist(
            extract_function,
            r#"
struct S { f: i32 };

impl S {
    fn foo(&mut self) {
        $0self.f += 1;$0
    }
}
"#,
            r#"
struct S { f: i32 };

impl S {
    fn foo(&mut self) {
        self.fun_name();
    }

    fn $0fun_name(&mut self) {
        self.f += 1;
    }
}
"#,
        );
    }

    #[test]
    fn variable_defined_inside_and_used_after_no_ret() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let n = 1;
    $0let k = n * n;$0
    let m = k + 1;
}
"#,
            r#"
fn foo() {
    let n = 1;
    let k = fun_name(n);
    let m = k + 1;
}

fn $0fun_name(n: i32) -> i32 {
    let k = n * n;
    k
}
"#,
        );
    }

    #[test]
    fn variable_defined_inside_and_used_after_mutably_no_ret() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let n = 1;
    $0let mut k = n * n;$0
    k += 1;
}
"#,
            r#"
fn foo() {
    let n = 1;
    let mut k = fun_name(n);
    k += 1;
}

fn $0fun_name(n: i32) -> i32 {
    let mut k = n * n;
    k
}
"#,
        );
    }

    #[test]
    fn two_variables_defined_inside_and_used_after_no_ret() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let n = 1;
    $0let k = n * n;
    let m = k + 2;$0
    let h = k + m;
}
"#,
            r#"
fn foo() {
    let n = 1;
    let (k, m) = fun_name(n);
    let h = k + m;
}

fn $0fun_name(n: i32) -> (i32, i32) {
    let k = n * n;
    let m = k + 2;
    (k, m)
}
"#,
        );
    }

    #[test]
    fn multi_variables_defined_inside_and_used_after_mutably_no_ret() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let n = 1;
    $0let mut k = n * n;
    let mut m = k + 2;
    let mut o = m + 3;
    o += 1;$0
    k += o;
    m = 1;
}
"#,
            r#"
fn foo() {
    let n = 1;
    let (mut k, mut m, o) = fun_name(n);
    k += o;
    m = 1;
}

fn $0fun_name(n: i32) -> (i32, i32, i32) {
    let mut k = n * n;
    let mut m = k + 2;
    let mut o = m + 3;
    o += 1;
    (k, m, o)
}
"#,
        );
    }

    #[test]
    fn nontrivial_patterns_define_variables() {
        check_assist(
            extract_function,
            r#"
struct Counter(i32);
fn foo() {
    $0let Counter(n) = Counter(0);$0
    let m = n;
}
"#,
            r#"
struct Counter(i32);
fn foo() {
    let n = fun_name();
    let m = n;
}

fn $0fun_name() -> i32 {
    let Counter(n) = Counter(0);
    n
}
"#,
        );
    }

    #[test]
    fn struct_with_two_fields_pattern_define_variables() {
        check_assist(
            extract_function,
            r#"
struct Counter { n: i32, m: i32 };
fn foo() {
    $0let Counter { n, m: k } = Counter { n: 1, m: 2 };$0
    let h = n + k;
}
"#,
            r#"
struct Counter { n: i32, m: i32 };
fn foo() {
    let (n, k) = fun_name();
    let h = n + k;
}

fn $0fun_name() -> (i32, i32) {
    let Counter { n, m: k } = Counter { n: 1, m: 2 };
    (n, k)
}
"#,
        );
    }

    #[test]
    fn mut_var_from_outer_scope() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let mut n = 1;
    $0n += 1;$0
    let m = n + 1;
}
"#,
            r#"
fn foo() {
    let mut n = 1;
    fun_name(&mut n);
    let m = n + 1;
}

fn $0fun_name(n: &mut i32) {
    *n += 1;
}
"#,
        );
    }

    #[test]
    fn mut_field_from_outer_scope() {
        check_assist(
            extract_function,
            r#"
struct C { n: i32 }
fn foo() {
    let mut c = C { n: 0 };
    $0c.n += 1;$0
    let m = c.n + 1;
}
"#,
            r#"
struct C { n: i32 }
fn foo() {
    let mut c = C { n: 0 };
    fun_name(&mut c);
    let m = c.n + 1;
}

fn $0fun_name(c: &mut C) {
    c.n += 1;
}
"#,
        );
    }

    #[test]
    fn mut_nested_field_from_outer_scope() {
        check_assist(
            extract_function,
            r#"
struct P { n: i32}
struct C { p: P }
fn foo() {
    let mut c = C { p: P { n: 0 } };
    let mut v = C { p: P { n: 0 } };
    let u = C { p: P { n: 0 } };
    $0c.p.n += u.p.n;
    let r = &mut v.p.n;$0
    let m = c.p.n + v.p.n + u.p.n;
}
"#,
            r#"
struct P { n: i32}
struct C { p: P }
fn foo() {
    let mut c = C { p: P { n: 0 } };
    let mut v = C { p: P { n: 0 } };
    let u = C { p: P { n: 0 } };
    fun_name(&mut c, &u, &mut v);
    let m = c.p.n + v.p.n + u.p.n;
}

fn $0fun_name(c: &mut C, u: &C, v: &mut C) {
    c.p.n += u.p.n;
    let r = &mut v.p.n;
}
"#,
        );
    }

    #[test]
    fn mut_param_many_usages_stmt() {
        check_assist(
            extract_function,
            r#"
fn bar(k: i32) {}
trait I: Copy {
    fn succ(&self) -> Self;
    fn inc(&mut self) -> Self { let v = self.succ(); *self = v; v }
}
impl I for i32 {
    fn succ(&self) -> Self { *self + 1 }
}
fn foo() {
    let mut n = 1;
    $0n += n;
    bar(n);
    bar(n+1);
    bar(n*n);
    bar(&n);
    n.inc();
    let v = &mut n;
    *v = v.succ();
    n.succ();$0
    let m = n + 1;
}
"#,
            r#"
fn bar(k: i32) {}
trait I: Copy {
    fn succ(&self) -> Self;
    fn inc(&mut self) -> Self { let v = self.succ(); *self = v; v }
}
impl I for i32 {
    fn succ(&self) -> Self { *self + 1 }
}
fn foo() {
    let mut n = 1;
    fun_name(&mut n);
    let m = n + 1;
}

fn $0fun_name(n: &mut i32) {
    *n += *n;
    bar(*n);
    bar(*n+1);
    bar(*n**n);
    bar(&*n);
    n.inc();
    let v = n;
    *v = v.succ();
    n.succ();
}
"#,
        );
    }

    #[test]
    fn mut_param_many_usages_expr() {
        check_assist(
            extract_function,
            r#"
fn bar(k: i32) {}
trait I: Copy {
    fn succ(&self) -> Self;
    fn inc(&mut self) -> Self { let v = self.succ(); *self = v; v }
}
impl I for i32 {
    fn succ(&self) -> Self { *self + 1 }
}
fn foo() {
    let mut n = 1;
    $0{
        n += n;
        bar(n);
        bar(n+1);
        bar(n*n);
        bar(&n);
        n.inc();
        let v = &mut n;
        *v = v.succ();
        n.succ();
    }$0
    let m = n + 1;
}
"#,
            r#"
fn bar(k: i32) {}
trait I: Copy {
    fn succ(&self) -> Self;
    fn inc(&mut self) -> Self { let v = self.succ(); *self = v; v }
}
impl I for i32 {
    fn succ(&self) -> Self { *self + 1 }
}
fn foo() {
    let mut n = 1;
    fun_name(&mut n);
    let m = n + 1;
}

fn $0fun_name(n: &mut i32) {
    *n += *n;
    bar(*n);
    bar(*n+1);
    bar(*n**n);
    bar(&*n);
    n.inc();
    let v = n;
    *v = v.succ();
    n.succ();
}
"#,
        );
    }

    #[test]
    fn mut_param_by_value() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let mut n = 1;
    $0n += 1;$0
}
"#,
            r"
fn foo() {
    let mut n = 1;
    fun_name(n);
}

fn $0fun_name(mut n: i32) {
    n += 1;
}
",
        );
    }

    #[test]
    fn mut_param_because_of_mut_ref() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let mut n = 1;
    $0let v = &mut n;
    *v += 1;$0
    let k = n;
}
"#,
            r#"
fn foo() {
    let mut n = 1;
    fun_name(&mut n);
    let k = n;
}

fn $0fun_name(n: &mut i32) {
    let v = n;
    *v += 1;
}
"#,
        );
    }

    #[test]
    fn mut_param_by_value_because_of_mut_ref() {
        check_assist(
            extract_function,
            r"
fn foo() {
    let mut n = 1;
    $0let v = &mut n;
    *v += 1;$0
}
",
            r#"
fn foo() {
    let mut n = 1;
    fun_name(n);
}

fn $0fun_name(mut n: i32) {
    let v = &mut n;
    *v += 1;
}
"#,
        );
    }

    #[test]
    fn mut_method_call() {
        check_assist(
            extract_function,
            r#"
trait I {
    fn inc(&mut self);
}
impl I for i32 {
    fn inc(&mut self) { *self += 1 }
}
fn foo() {
    let mut n = 1;
    $0n.inc();$0
}
"#,
            r#"
trait I {
    fn inc(&mut self);
}
impl I for i32 {
    fn inc(&mut self) { *self += 1 }
}
fn foo() {
    let mut n = 1;
    fun_name(n);
}

fn $0fun_name(mut n: i32) {
    n.inc();
}
"#,
        );
    }

    #[test]
    fn shared_method_call() {
        check_assist(
            extract_function,
            r#"
trait I {
    fn succ(&self);
}
impl I for i32 {
    fn succ(&self) { *self + 1 }
}
fn foo() {
    let mut n = 1;
    $0n.succ();$0
}
"#,
            r"
trait I {
    fn succ(&self);
}
impl I for i32 {
    fn succ(&self) { *self + 1 }
}
fn foo() {
    let mut n = 1;
    fun_name(n);
}

fn $0fun_name(n: i32) {
    n.succ();
}
",
        );
    }

    #[test]
    fn mut_method_call_with_other_receiver() {
        check_assist(
            extract_function,
            r#"
trait I {
    fn inc(&mut self, n: i32);
}
impl I for i32 {
    fn inc(&mut self, n: i32) { *self += n }
}
fn foo() {
    let mut n = 1;
    $0let mut m = 2;
    m.inc(n);$0
}
"#,
            r"
trait I {
    fn inc(&mut self, n: i32);
}
impl I for i32 {
    fn inc(&mut self, n: i32) { *self += n }
}
fn foo() {
    let mut n = 1;
    fun_name(n);
}

fn $0fun_name(n: i32) {
    let mut m = 2;
    m.inc(n);
}
",
        );
    }

    #[test]
    fn non_copy_without_usages_after() {
        check_assist(
            extract_function,
            r#"
struct Counter(i32);
fn foo() {
    let c = Counter(0);
    $0let n = c.0;$0
}
"#,
            r"
struct Counter(i32);
fn foo() {
    let c = Counter(0);
    fun_name(c);
}

fn $0fun_name(c: Counter) {
    let n = c.0;
}
",
        );
    }

    #[test]
    fn non_copy_used_after() {
        check_assist(
            extract_function,
            r"
struct Counter(i32);
fn foo() {
    let c = Counter(0);
    $0let n = c.0;$0
    let m = c.0;
}
",
            r#"
struct Counter(i32);
fn foo() {
    let c = Counter(0);
    fun_name(&c);
    let m = c.0;
}

fn $0fun_name(c: &Counter) {
    let n = c.0;
}
"#,
        );
    }

    #[test]
    fn copy_used_after() {
        check_assist(
            extract_function,
            r#"
//- minicore: copy
fn foo() {
    let n = 0;
    $0let m = n;$0
    let k = n;
}
"#,
            r#"
fn foo() {
    let n = 0;
    fun_name(n);
    let k = n;
}

fn $0fun_name(n: i32) {
    let m = n;
}
"#,
        )
    }

    #[test]
    fn copy_custom_used_after() {
        check_assist(
            extract_function,
            r#"
//- minicore: copy, derive
#[derive(Clone, Copy)]
struct Counter(i32);
fn foo() {
    let c = Counter(0);
    $0let n = c.0;$0
    let m = c.0;
}
"#,
            r#"
#[derive(Clone, Copy)]
struct Counter(i32);
fn foo() {
    let c = Counter(0);
    fun_name(c);
    let m = c.0;
}

fn $0fun_name(c: Counter) {
    let n = c.0;
}
"#,
        );
    }

    #[test]
    fn indented_stmts() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    if true {
        loop {
            $0let n = 1;
            let m = 2;$0
        }
    }
}
"#,
            r#"
fn foo() {
    if true {
        loop {
            fun_name();
        }
    }
}

fn $0fun_name() {
    let n = 1;
    let m = 2;
}
"#,
        );
    }

    #[test]
    fn indented_stmts_inside_mod() {
        check_assist(
            extract_function,
            r#"
mod bar {
    fn foo() {
        if true {
            loop {
                $0let n = 1;
                let m = 2;$0
            }
        }
    }
}
"#,
            r#"
mod bar {
    fn foo() {
        if true {
            loop {
                fun_name();
            }
        }
    }

    fn $0fun_name() {
        let n = 1;
        let m = 2;
    }
}
"#,
        );
    }

    #[test]
    fn break_loop() {
        check_assist(
            extract_function,
            r#"
//- minicore: option
fn foo() {
    loop {
        let n = 1;
        $0let m = n + 1;
        break;
        let k = 2;$0
        let h = 1 + k;
    }
}
"#,
            r#"
fn foo() {
    loop {
        let n = 1;
        let k = match fun_name(n) {
            Some(value) => value,
            None => break,
        };
        let h = 1 + k;
    }
}

fn $0fun_name(n: i32) -> Option<i32> {
    let m = n + 1;
    return None;
    let k = 2;
    Some(k)
}
"#,
        );
    }

    #[test]
    fn return_to_parent() {
        check_assist(
            extract_function,
            r#"
//- minicore: copy, result
fn foo() -> i64 {
    let n = 1;
    $0let m = n + 1;
    return 1;
    let k = 2;$0
    (n + k) as i64
}
"#,
            r#"
fn foo() -> i64 {
    let n = 1;
    let k = match fun_name(n) {
        Ok(value) => value,
        Err(value) => return value,
    };
    (n + k) as i64
}

fn $0fun_name(n: i32) -> Result<i32, i64> {
    let m = n + 1;
    return Err(1);
    let k = 2;
    Ok(k)
}
"#,
        );
    }

    #[test]
    fn break_and_continue() {
        cov_mark::check!(external_control_flow_break_and_continue);
        check_assist_not_applicable(
            extract_function,
            r#"
fn foo() {
    loop {
        let n = 1;
        $0let m = n + 1;
        break;
        let k = 2;
        continue;
        let k = k + 1;$0
        let r = n + k;
    }
}
"#,
        );
    }

    #[test]
    fn return_and_break() {
        cov_mark::check!(external_control_flow_return_and_bc);
        check_assist_not_applicable(
            extract_function,
            r#"
fn foo() {
    loop {
        let n = 1;
        $0let m = n + 1;
        break;
        let k = 2;
        return;
        let k = k + 1;$0
        let r = n + k;
    }
}
"#,
        );
    }

    #[test]
    fn break_loop_with_if() {
        check_assist(
            extract_function,
            r#"
//- minicore: try
fn foo() {
    loop {
        let mut n = 1;
        $0let m = n + 1;
        break;
        n += m;$0
        let h = 1 + n;
    }
}
"#,
            r#"
use core::ops::ControlFlow;

fn foo() {
    loop {
        let mut n = 1;
        if let ControlFlow::Break(_) = fun_name(&mut n) {
            break;
        }
        let h = 1 + n;
    }
}

fn $0fun_name(n: &mut i32) -> ControlFlow<()> {
    let m = *n + 1;
    return ControlFlow::Break(());
    *n += m;
    ControlFlow::Continue(())
}
"#,
        );
    }

    #[test]
    fn break_loop_nested() {
        check_assist(
            extract_function,
            r#"
//- minicore: try
fn foo() {
    loop {
        let mut n = 1;
        $0let m = n + 1;
        if m == 42 {
            break;
        }$0
        let h = 1;
    }
}
"#,
            r#"
use core::ops::ControlFlow;

fn foo() {
    loop {
        let mut n = 1;
        if let ControlFlow::Break(_) = fun_name(n) {
            break;
        }
        let h = 1;
    }
}

fn $0fun_name(n: i32) -> ControlFlow<()> {
    let m = n + 1;
    if m == 42 {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}
"#,
        );
    }

    #[test]
    fn break_loop_nested_labeled() {
        check_assist(
            extract_function,
            r#"
//- minicore: try
fn foo() {
    'bar: loop {
        loop {
            $0break 'bar;$0
        }
    }
}
"#,
            r#"
use core::ops::ControlFlow;

fn foo() {
    'bar: loop {
        loop {
            if let ControlFlow::Break(_) = fun_name() {
                break 'bar;
            }
        }
    }
}

fn $0fun_name() -> ControlFlow<()> {
    return ControlFlow::Break(());
    ControlFlow::Continue(())
}
"#,
        );
    }

    #[test]
    fn continue_loop_nested_labeled() {
        check_assist(
            extract_function,
            r#"
//- minicore: try
fn foo() {
    'bar: loop {
        loop {
            $0continue 'bar;$0
        }
    }
}
"#,
            r#"
use core::ops::ControlFlow;

fn foo() {
    'bar: loop {
        loop {
            if let ControlFlow::Break(_) = fun_name() {
                continue 'bar;
            }
        }
    }
}

fn $0fun_name() -> ControlFlow<()> {
    return ControlFlow::Break(());
    ControlFlow::Continue(())
}
"#,
        );
    }

    #[test]
    fn return_from_nested_loop() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    loop {
        let n = 1;$0
        let k = 1;
        loop {
            return;
        }
        let m = k + 1;$0
        let h = 1 + m;
    }
}
"#,
            r#"
fn foo() {
    loop {
        let n = 1;
        let m = match fun_name() {
            Some(value) => value,
            None => return,
        };
        let h = 1 + m;
    }
}

fn $0fun_name() -> Option<i32> {
    let k = 1;
    loop {
        return None;
    }
    let m = k + 1;
    Some(m)
}
"#,
        );
    }

    #[test]
    fn break_from_nested_loop() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    loop {
        let n = 1;
        $0let k = 1;
        loop {
            break;
        }
        let m = k + 1;$0
        let h = 1 + m;
    }
}
"#,
            r#"
fn foo() {
    loop {
        let n = 1;
        let m = fun_name();
        let h = 1 + m;
    }
}

fn $0fun_name() -> i32 {
    let k = 1;
    loop {
        break;
    }
    let m = k + 1;
    m
}
"#,
        );
    }

    #[test]
    fn break_from_nested_and_outer_loops() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    loop {
        let n = 1;
        $0let k = 1;
        loop {
            break;
        }
        if k == 42 {
            break;
        }
        let m = k + 1;$0
        let h = 1 + m;
    }
}
"#,
            r#"
fn foo() {
    loop {
        let n = 1;
        let m = match fun_name() {
            Some(value) => value,
            None => break,
        };
        let h = 1 + m;
    }
}

fn $0fun_name() -> Option<i32> {
    let k = 1;
    loop {
        break;
    }
    if k == 42 {
        return None;
    }
    let m = k + 1;
    Some(m)
}
"#,
        );
    }

    #[test]
    fn return_from_nested_fn() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    loop {
        let n = 1;
        $0let k = 1;
        fn test() {
            return;
        }
        let m = k + 1;$0
        let h = 1 + m;
    }
}
"#,
            r#"
fn foo() {
    loop {
        let n = 1;
        let m = fun_name();
        let h = 1 + m;
    }
}

fn $0fun_name() -> i32 {
    let k = 1;
    fn test() {
        return;
    }
    let m = k + 1;
    m
}
"#,
        );
    }

    #[test]
    fn break_with_value() {
        check_assist(
            extract_function,
            r#"
fn foo() -> i32 {
    loop {
        let n = 1;
        $0let k = 1;
        if k == 42 {
            break 3;
        }
        let m = k + 1;$0
        let h = 1;
    }
}
"#,
            r#"
fn foo() -> i32 {
    loop {
        let n = 1;
        if let Some(value) = fun_name() {
            break value;
        }
        let h = 1;
    }
}

fn $0fun_name() -> Option<i32> {
    let k = 1;
    if k == 42 {
        return Some(3);
    }
    let m = k + 1;
    None
}
"#,
        );
    }

    #[test]
    fn break_with_value_and_label() {
        check_assist(
            extract_function,
            r#"
fn foo() -> i32 {
    'bar: loop {
        let n = 1;
        $0let k = 1;
        if k == 42 {
            break 'bar 4;
        }
        let m = k + 1;$0
        let h = 1;
    }
}
"#,
            r#"
fn foo() -> i32 {
    'bar: loop {
        let n = 1;
        if let Some(value) = fun_name() {
            break 'bar value;
        }
        let h = 1;
    }
}

fn $0fun_name() -> Option<i32> {
    let k = 1;
    if k == 42 {
        return Some(4);
    }
    let m = k + 1;
    None
}
"#,
        );
    }

    #[test]
    fn break_with_value_and_return() {
        check_assist(
            extract_function,
            r#"
fn foo() -> i64 {
    loop {
        let n = 1;$0
        let k = 1;
        if k == 42 {
            break 3;
        }
        let m = k + 1;$0
        let h = 1 + m;
    }
}
"#,
            r#"
fn foo() -> i64 {
    loop {
        let n = 1;
        let m = match fun_name() {
            Ok(value) => value,
            Err(value) => break value,
        };
        let h = 1 + m;
    }
}

fn $0fun_name() -> Result<i32, i64> {
    let k = 1;
    if k == 42 {
        return Err(3);
    }
    let m = k + 1;
    Ok(m)
}
"#,
        );
    }

    #[test]
    fn try_option() {
        check_assist(
            extract_function,
            r#"
//- minicore: option
fn bar() -> Option<i32> { None }
fn foo() -> Option<()> {
    let n = bar()?;
    $0let k = foo()?;
    let m = k + 1;$0
    let h = 1 + m;
    Some(())
}
"#,
            r#"
fn bar() -> Option<i32> { None }
fn foo() -> Option<()> {
    let n = bar()?;
    let m = fun_name()?;
    let h = 1 + m;
    Some(())
}

fn $0fun_name() -> Option<i32> {
    let k = foo()?;
    let m = k + 1;
    Some(m)
}
"#,
        );
    }

    #[test]
    fn try_option_unit() {
        check_assist(
            extract_function,
            r#"
//- minicore: option
fn foo() -> Option<()> {
    let n = 1;
    $0let k = foo()?;
    let m = k + 1;$0
    let h = 1 + n;
    Some(())
}
"#,
            r#"
fn foo() -> Option<()> {
    let n = 1;
    fun_name()?;
    let h = 1 + n;
    Some(())
}

fn $0fun_name() -> Option<()> {
    let k = foo()?;
    let m = k + 1;
    Some(())
}
"#,
        );
    }

    #[test]
    fn try_result() {
        check_assist(
            extract_function,
            r#"
//- minicore: result
fn foo() -> Result<(), i64> {
    let n = 1;
    $0let k = foo()?;
    let m = k + 1;$0
    let h = 1 + m;
    Ok(())
}
"#,
            r#"
fn foo() -> Result<(), i64> {
    let n = 1;
    let m = fun_name()?;
    let h = 1 + m;
    Ok(())
}

fn $0fun_name() -> Result<i32, i64> {
    let k = foo()?;
    let m = k + 1;
    Ok(m)
}
"#,
        );
    }

    #[test]
    fn try_option_with_return() {
        check_assist(
            extract_function,
            r#"
//- minicore: option
fn foo() -> Option<()> {
    let n = 1;
    $0let k = foo()?;
    if k == 42 {
        return None;
    }
    let m = k + 1;$0
    let h = 1 + m;
    Some(())
}
"#,
            r#"
fn foo() -> Option<()> {
    let n = 1;
    let m = fun_name()?;
    let h = 1 + m;
    Some(())
}

fn $0fun_name() -> Option<i32> {
    let k = foo()?;
    if k == 42 {
        return None;
    }
    let m = k + 1;
    Some(m)
}
"#,
        );
    }

    #[test]
    fn try_result_with_return() {
        check_assist(
            extract_function,
            r#"
//- minicore: result
fn foo() -> Result<(), i64> {
    let n = 1;
    $0let k = foo()?;
    if k == 42 {
        return Err(1);
    }
    let m = k + 1;$0
    let h = 1 + m;
    Ok(())
}
"#,
            r#"
fn foo() -> Result<(), i64> {
    let n = 1;
    let m = fun_name()?;
    let h = 1 + m;
    Ok(())
}

fn $0fun_name() -> Result<i32, i64> {
    let k = foo()?;
    if k == 42 {
        return Err(1);
    }
    let m = k + 1;
    Ok(m)
}
"#,
        );
    }

    #[test]
    fn try_and_break() {
        cov_mark::check!(external_control_flow_try_and_bc);
        check_assist_not_applicable(
            extract_function,
            r#"
//- minicore: option
fn foo() -> Option<()> {
    loop {
        let n = Some(1);
        $0let m = n? + 1;
        break;
        let k = 2;
        let k = k + 1;$0
        let r = n + k;
    }
    Some(())
}
"#,
        );
    }

    #[test]
    fn try_and_return_ok() {
        check_assist(
            extract_function,
            r#"
//- minicore: result
fn foo() -> Result<(), i64> {
    let n = 1;
    $0let k = foo()?;
    if k == 42 {
        return Ok(1);
    }
    let m = k + 1;$0
    let h = 1 + m;
    Ok(())
}
"#,
            r#"
fn foo() -> Result<(), i64> {
    let n = 1;
    let m = fun_name()?;
    let h = 1 + m;
    Ok(())
}

fn $0fun_name() -> Result<i32, i64> {
    let k = foo()?;
    if k == 42 {
        return Ok(1);
    }
    let m = k + 1;
    Ok(m)
}
"#,
        );
    }

    #[test]
    fn param_usage_in_macro() {
        check_assist(
            extract_function,
            r#"
macro_rules! m {
    ($val:expr) => { $val };
}

fn foo() {
    let n = 1;
    $0let k = n * m!(n);$0
    let m = k + 1;
}
"#,
            r#"
macro_rules! m {
    ($val:expr) => { $val };
}

fn foo() {
    let n = 1;
    let k = fun_name(n);
    let m = k + 1;
}

fn $0fun_name(n: i32) -> i32 {
    let k = n * m!(n);
    k
}
"#,
        );
    }

    #[test]
    fn param_usage_in_macro_with_nested_tt() {
        check_assist(
            extract_function,
            r#"
macro_rules! m {
    ($val:expr) => { $val };
}

fn foo() {
    let n = 1;
    let t = 1;
    $0let k = n * m!((n) + { t });$0
    let m = k + 1;
}
"#,
            r#"
macro_rules! m {
    ($val:expr) => { $val };
}

fn foo() {
    let n = 1;
    let t = 1;
    let k = fun_name(n, t);
    let m = k + 1;
}

fn $0fun_name(n: i32, t: i32) -> i32 {
    let k = n * m!((n) + { t });
    k
}
"#,
        )
    }

    #[test]
    fn param_usage_in_macro_with_nested_tt_2() {
        check_assist(
            extract_function,
            r#"
macro_rules! m {
    ($val:expr) => { $val };
}

struct S(i32);
impl S {
    fn foo(&self) {
        let n = 1;
        $0let k = n * m!((n) + { self.0 });$0
        let m = k + 1;
    }
}
"#,
            r#"
macro_rules! m {
    ($val:expr) => { $val };
}

struct S(i32);
impl S {
    fn foo(&self) {
        let n = 1;
        let k = self.fun_name(n);
        let m = k + 1;
    }

    fn $0fun_name(&self, n: i32) -> i32 {
        let k = n * m!((n) + { self.0 });
        k
    }
}
"#,
        )
    }

    #[test]
    fn extract_with_await() {
        check_assist(
            extract_function,
            r#"
//- minicore: future
fn main() {
    $0some_function().await;$0
}

async fn some_function() {

}
"#,
            r#"
fn main() {
    fun_name().await;
}

async fn $0fun_name() {
    some_function().await;
}

async fn some_function() {

}
"#,
        );
    }

    #[test]
    fn extract_with_await_and_result_not_producing_match_expr() {
        check_assist(
            extract_function,
            r#"
//- minicore: future, result
async fn foo() -> Result<(), ()> {
    $0async {}.await;
    Err(())?$0
}
"#,
            r#"
async fn foo() -> Result<(), ()> {
    fun_name().await
}

async fn $0fun_name() -> Result<(), ()> {
    async {}.await;
    Err(())?
}
"#,
        );
    }

    #[test]
    fn extract_with_await_and_result_producing_match_expr() {
        check_assist(
            extract_function,
            r#"
//- minicore: future
async fn foo() -> i32 {
    loop {
        let n = 1;$0
        let k = async { 1 }.await;
        if k == 42 {
            break 3;
        }
        let m = k + 1;$0
        let h = 1 + m;
    }
}
"#,
            r#"
async fn foo() -> i32 {
    loop {
        let n = 1;
        let m = match fun_name().await {
            Ok(value) => value,
            Err(value) => break value,
        };
        let h = 1 + m;
    }
}

async fn $0fun_name() -> Result<i32, i32> {
    let k = async { 1 }.await;
    if k == 42 {
        return Err(3);
    }
    let m = k + 1;
    Ok(m)
}
"#,
        );
    }

    #[test]
    fn extract_with_await_in_args() {
        check_assist(
            extract_function,
            r#"
//- minicore: future
fn main() {
    $0function_call("a", some_function().await);$0
}

async fn some_function() {

}
"#,
            r#"
fn main() {
    fun_name().await;
}

async fn $0fun_name() {
    function_call("a", some_function().await);
}

async fn some_function() {

}
"#,
        );
    }

    #[test]
    fn extract_does_not_extract_standalone_blocks() {
        check_assist_not_applicable(
            extract_function,
            r#"
fn main() $0{}$0
"#,
        );
    }

    #[test]
    fn extract_adds_comma_for_match_arm() {
        check_assist(
            extract_function,
            r#"
fn main() {
    match 6 {
        100 => $0{ 100 }$0
        _ => 0,
    };
}
"#,
            r#"
fn main() {
    match 6 {
        100 => fun_name(),
        _ => 0,
    };
}

fn $0fun_name() -> i32 {
    100
}
"#,
        );
        check_assist(
            extract_function,
            r#"
fn main() {
    match 6 {
        100 => $0{ 100 }$0,
        _ => 0,
    };
}
"#,
            r#"
fn main() {
    match 6 {
        100 => fun_name(),
        _ => 0,
    };
}

fn $0fun_name() -> i32 {
    100
}
"#,
        );

        // Makes sure no semicolon is added for unit-valued match arms
        check_assist(
            extract_function,
            r#"
fn main() {
    match () {
        _ => $0()$0,
    }
}
"#,
            r#"
fn main() {
    match () {
        _ => fun_name(),
    }
}

fn $0fun_name() {
    ()
}
"#,
        )
    }

    #[test]
    fn extract_does_not_tear_comments_apart() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    /*$0*/
    foo();
    foo();
    /*$0*/
}
"#,
            r#"
fn foo() {
    fun_name();
}

fn $0fun_name() {
    /**/
    foo();
    foo();
    /**/
}
"#,
        );
    }

    #[test]
    fn extract_does_not_tear_body_apart() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    $0foo();
}$0
"#,
            r#"
fn foo() {
    fun_name();
}

fn $0fun_name() {
    foo();
}
"#,
        );
    }

    #[test]
    fn extract_does_not_wrap_res_in_res() {
        check_assist(
            extract_function,
            r#"
//- minicore: result, try
fn foo() -> Result<(), i64> {
    $0Result::<i32, i64>::Ok(0)?;
    Ok(())$0
}
"#,
            r#"
fn foo() -> Result<(), i64> {
    fun_name()
}

fn $0fun_name() -> Result<(), i64> {
    Result::<i32, i64>::Ok(0)?;
    Ok(())
}
"#,
        );
    }

    #[test]
    fn extract_knows_const() {
        check_assist(
            extract_function,
            r#"
const fn foo() {
    $0()$0
}
"#,
            r#"
const fn foo() {
    fun_name();
}

const fn $0fun_name() {
    ()
}
"#,
        );
        check_assist(
            extract_function,
            r#"
const FOO: () = {
    $0()$0
};
"#,
            r#"
const FOO: () = {
    fun_name();
};

const fn $0fun_name() {
    ()
}
"#,
        );
    }

    #[test]
    fn extract_does_not_move_outer_loop_vars() {
        check_assist(
            extract_function,
            r#"
//- minicore: iterator
fn foo() {
    let mut x = 5;
    for _ in 0..10 {
        $0x += 1;$0
    }
}
"#,
            r#"
fn foo() {
    let mut x = 5;
    for _ in 0..10 {
        fun_name(&mut x);
    }
}

fn $0fun_name(x: &mut i32) {
    *x += 1;
}
"#,
        );
        check_assist(
            extract_function,
            r#"
//- minicore: iterator
fn foo() {
    for _ in 0..10 {
        let mut x = 5;
        $0x += 1;$0
    }
}
"#,
            r#"
fn foo() {
    for _ in 0..10 {
        let mut x = 5;
        fun_name(x);
    }
}

fn $0fun_name(mut x: i32) {
    x += 1;
}
"#,
        );
        check_assist(
            extract_function,
            r#"
//- minicore: iterator
fn foo() {
    loop {
        let mut x = 5;
        for _ in 0..10 {
            $0x += 1;$0
        }
    }
}
"#,
            r#"
fn foo() {
    loop {
        let mut x = 5;
        for _ in 0..10 {
            fun_name(&mut x);
        }
    }
}

fn $0fun_name(x: &mut i32) {
    *x += 1;
}
"#,
        );
    }

    // regression test for #9822
    #[test]
    fn extract_mut_ref_param_has_no_mut_binding_in_loop() {
        check_assist(
            extract_function,
            r#"
struct Foo;
impl Foo {
    fn foo(&mut self) {}
}
fn foo() {
    let mut x = Foo;
    while false {
        let y = &mut x;
        $0y.foo();$0
    }
    let z = x;
}
"#,
            r#"
struct Foo;
impl Foo {
    fn foo(&mut self) {}
}
fn foo() {
    let mut x = Foo;
    while false {
        let y = &mut x;
        fun_name(y);
    }
    let z = x;
}

fn $0fun_name(y: &mut Foo) {
    y.foo();
}
"#,
        );
    }

    #[test]
    fn extract_with_macro_arg() {
        check_assist(
            extract_function,
            r#"
macro_rules! m {
    ($val:expr) => { $val };
}
fn main() {
    let bar = "bar";
    $0m!(bar);$0
}
"#,
            r#"
macro_rules! m {
    ($val:expr) => { $val };
}
fn main() {
    let bar = "bar";
    fun_name(bar);
}

fn $0fun_name(bar: &str) {
    m!(bar);
}
"#,
        );
    }

    #[test]
    fn unresolveable_types_default_to_placeholder() {
        check_assist(
            extract_function,
            r#"
fn foo() {
    let a = __unresolved;
    let _ = $0{a}$0;
}
"#,
            r#"
fn foo() {
    let a = __unresolved;
    let _ = fun_name(a);
}

fn $0fun_name(a: _) -> _ {
    a
}
"#,
        );
    }

    #[test]
    fn reference_mutable_param_with_further_usages() {
        check_assist(
            extract_function,
            r#"
pub struct Foo {
    field: u32,
}

pub fn testfn(arg: &mut Foo) {
    $0arg.field = 8;$0
    // Simulating access after the extracted portion
    arg.field = 16;
}
"#,
            r#"
pub struct Foo {
    field: u32,
}

pub fn testfn(arg: &mut Foo) {
    fun_name(arg);
    // Simulating access after the extracted portion
    arg.field = 16;
}

fn $0fun_name(arg: &mut Foo) {
    arg.field = 8;
}
"#,
        );
    }

    #[test]
    fn reference_mutable_param_without_further_usages() {
        check_assist(
            extract_function,
            r#"
pub struct Foo {
    field: u32,
}

pub fn testfn(arg: &mut Foo) {
    $0arg.field = 8;$0
}
"#,
            r#"
pub struct Foo {
    field: u32,
}

pub fn testfn(arg: &mut Foo) {
    fun_name(arg);
}

fn $0fun_name(arg: &mut Foo) {
    arg.field = 8;
}
"#,
        );
    }
    #[test]
    fn does_not_import_control_flow() {
        check_assist(
            extract_function,
            r#"
//- minicore: try
fn func() {
    $0let cf = "I'm ControlFlow";$0
}
"#,
            r#"
fn func() {
    fun_name();
}

fn $0fun_name() {
    let cf = "I'm ControlFlow";
}
"#,
        );
    }

    #[test]
    fn extract_function_copies_comment_at_start() {
        check_assist(
            extract_function,
            r#"
fn func() {
    let i = 0;
    $0// comment here!
    let x = 0;$0
}
"#,
            r#"
fn func() {
    let i = 0;
    fun_name();
}

fn $0fun_name() {
    // comment here!
    let x = 0;
}
"#,
        );
    }

    #[test]
    fn extract_function_copies_comment_in_between() {
        check_assist(
            extract_function,
            r#"
fn func() {
    let i = 0;$0
    let a = 0;
    // comment here!
    let x = 0;$0
}
"#,
            r#"
fn func() {
    let i = 0;
    fun_name();
}

fn $0fun_name() {
    let a = 0;
    // comment here!
    let x = 0;
}
"#,
        );
    }

    #[test]
    fn extract_function_copies_comment_at_end() {
        check_assist(
            extract_function,
            r#"
fn func() {
    let i = 0;
    $0let x = 0;
    // comment here!$0
}
"#,
            r#"
fn func() {
    let i = 0;
    fun_name();
}

fn $0fun_name() {
    let x = 0;
    // comment here!
}
"#,
        );
    }

    #[test]
    fn extract_function_copies_comment_indented() {
        check_assist(
            extract_function,
            r#"
fn func() {
    let i = 0;
    $0let x = 0;
    while(true) {
        // comment here!
    }$0
}
"#,
            r#"
fn func() {
    let i = 0;
    fun_name();
}

fn $0fun_name() {
    let x = 0;
    while(true) {
        // comment here!
    }
}
"#,
        );
    }

    #[test]
    fn extract_function_does_preserve_whitespace() {
        check_assist(
            extract_function,
            r#"
fn func() {
    let i = 0;
    $0let a = 0;

    let x = 0;$0
}
"#,
            r#"
fn func() {
    let i = 0;
    fun_name();
}

fn $0fun_name() {
    let a = 0;

    let x = 0;
}
"#,
        );
    }

    #[test]
    fn extract_function_long_form_comment() {
        check_assist(
            extract_function,
            r#"
fn func() {
    let i = 0;
    $0/* a comment */
    let x = 0;$0
}
"#,
            r#"
fn func() {
    let i = 0;
    fun_name();
}

fn $0fun_name() {
    /* a comment */
    let x = 0;
}
"#,
        );
    }

    #[test]
    fn it_should_not_generate_duplicate_function_names() {
        check_assist(
            extract_function,
            r#"
fn fun_name() {
    $0let x = 0;$0
}
"#,
            r#"
fn fun_name() {
    fun_name1();
}

fn $0fun_name1() {
    let x = 0;
}
"#,
        );
    }

    #[test]
    fn should_increment_suffix_until_it_finds_space() {
        check_assist(
            extract_function,
            r#"
fn fun_name1() {
    let y = 0;
}

fn fun_name() {
    $0let x = 0;$0
}
"#,
            r#"
fn fun_name1() {
    let y = 0;
}

fn fun_name() {
    fun_name2();
}

fn $0fun_name2() {
    let x = 0;
}
"#,
        );
    }

    #[test]
    fn extract_method_from_trait_impl() {
        check_assist(
            extract_function,
            r#"
struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        $0self.0 + 2$0
    }
}
"#,
            r#"
struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        self.fun_name()
    }
}

impl Struct {
    fn $0fun_name(&self) -> i32 {
        self.0 + 2
    }
}
"#,
        );
    }

    #[test]
    fn extract_method_from_trait_with_existing_non_empty_impl_block() {
        check_assist(
            extract_function,
            r#"
struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl Struct {
    fn foo() {}
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        $0self.0 + 2$0
    }
}
"#,
            r#"
struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl Struct {
    fn foo() {}

    fn $0fun_name(&self) -> i32 {
        self.0 + 2
    }
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        self.fun_name()
    }
}
"#,
        )
    }

    #[test]
    fn extract_function_from_trait_with_existing_non_empty_impl_block() {
        check_assist(
            extract_function,
            r#"
struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl Struct {
    fn foo() {}
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        let three_squared = $03 * 3$0;
        self.0 + three_squared
    }
}
"#,
            r#"
struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl Struct {
    fn foo() {}
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        let three_squared = fun_name();
        self.0 + three_squared
    }
}

fn $0fun_name() -> i32 {
    3 * 3
}
"#,
        )
    }

    #[test]
    fn extract_method_from_trait_with_multiple_existing_impl_blocks() {
        check_assist(
            extract_function,
            r#"
struct Struct(i32);
struct StructBefore(i32);
struct StructAfter(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl StructBefore {
    fn foo(){}
}

impl Struct {
    fn foo(){}
}

impl StructAfter {
    fn foo(){}
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        $0self.0 + 2$0
    }
}
"#,
            r#"
struct Struct(i32);
struct StructBefore(i32);
struct StructAfter(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl StructBefore {
    fn foo(){}
}

impl Struct {
    fn foo(){}

    fn $0fun_name(&self) -> i32 {
        self.0 + 2
    }
}

impl StructAfter {
    fn foo(){}
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        self.fun_name()
    }
}
"#,
        )
    }

    #[test]
    fn extract_method_from_trait_with_multiple_existing_trait_impl_blocks() {
        check_assist(
            extract_function,
            r#"
struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}
trait TraitBefore {
    fn before(&self) -> i32;
}
trait TraitAfter {
    fn after(&self) -> i32;
}

impl TraitBefore for Struct {
    fn before(&self) -> i32 {
        42
    }
}

impl Struct {
    fn foo(){}
}

impl TraitAfter for Struct {
    fn after(&self) -> i32 {
        42
    }
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        $0self.0 + 2$0
    }
}
"#,
            r#"
struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}
trait TraitBefore {
    fn before(&self) -> i32;
}
trait TraitAfter {
    fn after(&self) -> i32;
}

impl TraitBefore for Struct {
    fn before(&self) -> i32 {
        42
    }
}

impl Struct {
    fn foo(){}

    fn $0fun_name(&self) -> i32 {
        self.0 + 2
    }
}

impl TraitAfter for Struct {
    fn after(&self) -> i32 {
        42
    }
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        self.fun_name()
    }
}
"#,
        )
    }

    #[test]
    fn closure_arguments() {
        check_assist(
            extract_function,
            r#"
fn parent(factor: i32) {
    let v = &[1, 2, 3];

    $0v.iter().map(|it| it * factor);$0
}
"#,
            r#"
fn parent(factor: i32) {
    let v = &[1, 2, 3];

    fun_name(v, factor);
}

fn $0fun_name(v: &[i32; 3], factor: i32) {
    v.iter().map(|it| it * factor);
}
"#,
        );
    }

    #[test]
    fn preserve_generics() {
        check_assist(
            extract_function,
            r#"
fn func<T: Debug>(i: T) {
    $0foo(i);$0
}
"#,
            r#"
fn func<T: Debug>(i: T) {
    fun_name(i);
}

fn $0fun_name<T: Debug>(i: T) {
    foo(i);
}
"#,
        );
    }

    #[test]
    fn dont_emit_type_with_hidden_lifetime_parameter() {
        // FIXME: We should emit a `<T: Debug>` generic argument for the generated function
        check_assist(
            extract_function,
            r#"
struct Struct<'a, T>(&'a T);
fn func<T: Debug>(i: Struct<'_, T>) {
    $0foo(i);$0
}
"#,
            r#"
struct Struct<'a, T>(&'a T);
fn func<T: Debug>(i: Struct<'_, T>) {
    fun_name(i);
}

fn $0fun_name(i: Struct<'_, T>) {
    foo(i);
}
"#,
        );
    }

    #[test]
    fn preserve_generics_from_body() {
        check_assist(
            extract_function,
            r#"
fn func<T: Default>() -> T {
    $0T::default()$0
}
"#,
            r#"
fn func<T: Default>() -> T {
    fun_name()
}

fn $0fun_name<T: Default>() -> T {
    T::default()
}
"#,
        );
    }

    #[test]
    fn filter_unused_generics() {
        check_assist(
            extract_function,
            r#"
fn func<T: Debug, U: Copy>(i: T, u: U) {
    bar(u);
    $0foo(i);$0
}
"#,
            r#"
fn func<T: Debug, U: Copy>(i: T, u: U) {
    bar(u);
    fun_name(i);
}

fn $0fun_name<T: Debug>(i: T) {
    foo(i);
}
"#,
        );
    }

    #[test]
    fn empty_generic_param_list() {
        check_assist(
            extract_function,
            r#"
fn func<T: Debug>(t: T, i: u32) {
    bar(t);
    $0foo(i);$0
}
"#,
            r#"
fn func<T: Debug>(t: T, i: u32) {
    bar(t);
    fun_name(i);
}

fn $0fun_name(i: u32) {
    foo(i);
}
"#,
        );
    }

    #[test]
    fn preserve_where_clause() {
        check_assist(
            extract_function,
            r#"
fn func<T>(i: T) where T: Debug {
    $0foo(i);$0
}
"#,
            r#"
fn func<T>(i: T) where T: Debug {
    fun_name(i);
}

fn $0fun_name<T>(i: T) where T: Debug {
    foo(i);
}
"#,
        );
    }

    #[test]
    fn filter_unused_where_clause() {
        check_assist(
            extract_function,
            r#"
fn func<T, U>(i: T, u: U) where T: Debug, U: Copy {
    bar(u);
    $0foo(i);$0
}
"#,
            r#"
fn func<T, U>(i: T, u: U) where T: Debug, U: Copy {
    bar(u);
    fun_name(i);
}

fn $0fun_name<T>(i: T) where T: Debug {
    foo(i);
}
"#,
        );
    }

    #[test]
    fn nested_generics() {
        check_assist(
            extract_function,
            r#"
struct Struct<T: Into<i32>>(T);
impl <T: Into<i32> + Copy> Struct<T> {
    fn func<V: Into<i32>>(&self, v: V) -> i32 {
        let t = self.0;
        $0t.into() + v.into()$0
    }
}
"#,
            r#"
struct Struct<T: Into<i32>>(T);
impl <T: Into<i32> + Copy> Struct<T> {
    fn func<V: Into<i32>>(&self, v: V) -> i32 {
        let t = self.0;
        fun_name(t, v)
    }
}

fn $0fun_name<T: Into<i32> + Copy, V: Into<i32>>(t: T, v: V) -> i32 {
    t.into() + v.into()
}
"#,
        );
    }

    #[test]
    fn filters_unused_nested_generics() {
        check_assist(
            extract_function,
            r#"
struct Struct<T: Into<i32>, U: Debug>(T, U);
impl <T: Into<i32> + Copy, U: Debug> Struct<T, U> {
    fn func<V: Into<i32>>(&self, v: V) -> i32 {
        let t = self.0;
        $0t.into() + v.into()$0
    }
}
"#,
            r#"
struct Struct<T: Into<i32>, U: Debug>(T, U);
impl <T: Into<i32> + Copy, U: Debug> Struct<T, U> {
    fn func<V: Into<i32>>(&self, v: V) -> i32 {
        let t = self.0;
        fun_name(t, v)
    }
}

fn $0fun_name<T: Into<i32> + Copy, V: Into<i32>>(t: T, v: V) -> i32 {
    t.into() + v.into()
}
"#,
        );
    }

    #[test]
    fn nested_where_clauses() {
        check_assist(
            extract_function,
            r#"
struct Struct<T>(T) where T: Into<i32>;
impl <T> Struct<T> where T: Into<i32> + Copy {
    fn func<V>(&self, v: V) -> i32 where V: Into<i32> {
        let t = self.0;
        $0t.into() + v.into()$0
    }
}
"#,
            r#"
struct Struct<T>(T) where T: Into<i32>;
impl <T> Struct<T> where T: Into<i32> + Copy {
    fn func<V>(&self, v: V) -> i32 where V: Into<i32> {
        let t = self.0;
        fun_name(t, v)
    }
}

fn $0fun_name<T, V>(t: T, v: V) -> i32 where T: Into<i32> + Copy, V: Into<i32> {
    t.into() + v.into()
}
"#,
        );
    }

    #[test]
    fn filters_unused_nested_where_clauses() {
        check_assist(
            extract_function,
            r#"
struct Struct<T, U>(T, U) where T: Into<i32>, U: Debug;
impl <T, U> Struct<T, U> where T: Into<i32> + Copy, U: Debug {
    fn func<V>(&self, v: V) -> i32 where V: Into<i32> {
        let t = self.0;
        $0t.into() + v.into()$0
    }
}
"#,
            r#"
struct Struct<T, U>(T, U) where T: Into<i32>, U: Debug;
impl <T, U> Struct<T, U> where T: Into<i32> + Copy, U: Debug {
    fn func<V>(&self, v: V) -> i32 where V: Into<i32> {
        let t = self.0;
        fun_name(t, v)
    }
}

fn $0fun_name<T, V>(t: T, v: V) -> i32 where T: Into<i32> + Copy, V: Into<i32> {
    t.into() + v.into()
}
"#,
        );
    }

    #[test]
    fn tail_expr_no_extra_control_flow() {
        check_assist(
            extract_function,
            r#"
//- minicore: result
fn fallible() -> Result<(), ()> {
    $0if true {
        return Err(());
    }
    Ok(())$0
}
"#,
            r#"
fn fallible() -> Result<(), ()> {
    fun_name()
}

fn $0fun_name() -> Result<(), ()> {
    if true {
        return Err(());
    }
    Ok(())
}
"#,
        );
    }

    #[test]
    fn non_tail_expr_of_tail_expr_loop() {
        check_assist(
            extract_function,
            r#"
pub fn f() {
    loop {
        $0if true {
            continue;
        }$0

        if false {
            break;
        }
    }
}
"#,
            r#"
pub fn f() {
    loop {
        if let ControlFlow::Break(_) = fun_name() {
            continue;
        }

        if false {
            break;
        }
    }
}

fn $0fun_name() -> ControlFlow<()> {
    if true {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}
"#,
        );
    }

    #[test]
    fn non_tail_expr_of_tail_if_block() {
        // FIXME: double semicolon
        check_assist(
            extract_function,
            r#"
//- minicore: option, try
fn f() -> Option<()> {
    if true {
        let a = $0if true {
            Some(())?
        } else {
            ()
        }$0;
        Some(a)
    } else {
        None
    }
}
"#,
            r#"
fn f() -> Option<()> {
    if true {
        let a = fun_name()?;
        Some(a)
    } else {
        None
    }
}

fn $0fun_name() -> Option<()> {
    Some(if true {
        Some(())?
    } else {
        ()
    })
}
"#,
        );
    }

    #[test]
    fn tail_expr_of_tail_block_nested() {
        check_assist(
            extract_function,
            r#"
//- minicore: option, try
fn f() -> Option<()> {
    if true {
        $0{
            let a = if true {
                Some(())?
            } else {
                ()
            };
            Some(a)
        }$0
    } else {
        None
    }
}
"#,
            r#"
fn f() -> Option<()> {
    if true {
        fun_name()
    } else {
        None
    }
}

fn $0fun_name() -> Option<()> {
    let a = if true {
        Some(())?
    } else {
        ()
    };
    Some(a)
}
"#,
        );
    }

    #[test]
    fn non_tail_expr_with_comment_of_tail_expr_loop() {
        check_assist(
            extract_function,
            r#"
pub fn f() {
    loop {
        $0// A comment
        if true {
            continue;
        }$0
        if false {
            break;
        }
    }
}
"#,
            r#"
pub fn f() {
    loop {
        if let ControlFlow::Break(_) = fun_name() {
            continue;
        }
        if false {
            break;
        }
    }
}

fn $0fun_name() -> ControlFlow<()> {
    // A comment
    if true {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}
"#,
        );
    }

    #[test]
    fn comments_in_block_expr() {
        check_assist(
            extract_function,
            r#"
fn f() {
    let c = $0{
        // comment 1
        let a = 2 + 3;
        // comment 2
        let b = 5;
        a + b
    }$0;
}
"#,
            r#"
fn f() {
    let c = fun_name();
}

fn $0fun_name() -> i32 {
    // comment 1
    let a = 2 + 3;
    // comment 2
    let b = 5;
    a + b
}
"#