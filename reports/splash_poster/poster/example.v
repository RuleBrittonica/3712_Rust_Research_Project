// Original
Definition test : result unit :=
  x <- i32_add 0%i32 1%i32;
  massert (x s= 1%i32).
Definition main : result unit :=
  test.

// Refactored
Definition incr (x : i32) : result i32 :=
  i32_add x 1%i32.
Definition test : result unit :=
  x <- incr 0%i32;
  massert (x s= 1%i32).
Definition main : result unit :=
  test.