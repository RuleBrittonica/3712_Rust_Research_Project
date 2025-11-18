Definition mutate_x (x : i32) : result i32 :=
  Ok 3%i32.

Definition main : result unit :=
  x <- mutate_x 0%i32; massert (x s= 3%i32).
(* *)
(* *)
(* *)