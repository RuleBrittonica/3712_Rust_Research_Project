Definition mutate_x (x : i32) : result unit :=
  Ok tt.

Definition main : result unit :=
  let i := core_clone_impls_CloneI32_clone 0%i32 in
  _ <- mutate_x i;
  massert (0%i32 s= 3%i32).