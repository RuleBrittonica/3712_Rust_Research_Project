(** [orig::bump]: loop 0:
    Source: 'src/main.rs', lines 9:4-15:5 *)
Fixpoint bump_loop0
  {N : usize} (k : usize) (arr : array i32 N) (mark : bool) (i : usize) :
  result ((array i32 N) * bool)
  :=
  if i s< k
  then (
    i1 <- array_index_usize arr i;
    if i1 s= 0%i32
    then (
      i2 <- i32_add i1 1%i32;
      arr1 <- array_update_usize arr i i2;
      i3 <- usize_add i 1%usize;
      bump_loop0 k arr1 true i3)
    else (
      i2 <- i32_add i1 1%i32;
      arr1 <- array_update_usize arr i i2;
      i3 <- usize_add i 1%usize;
      bump_loop0 k arr1 mark i3))
  else Ok (arr, mark)
.

(** [orig::bump]: loop 1:
    Source: 'src/main.rs', lines 9:4-15:5 *)
Fixpoint bump_loop1
  {N : usize} (arr : array i32 N) (mark : bool) (i : usize) :
  result ((array i32 N) * bool)
  :=
  if i s< N
  then (
    i1 <- array_index_usize arr i;
    if i1 s= 0%i32
    then (
      i2 <- i32_add i1 1%i32;
      arr1 <- array_update_usize arr i i2;
      i3 <- usize_add i 1%usize;
      bump_loop1 arr1 true i3)
    else (
      i2 <- i32_add i1 1%i32;
      arr1 <- array_update_usize arr i i2;
      i3 <- usize_add i 1%usize;
      bump_loop1 arr1 mark i3))
  else Ok (arr, mark)
.

(** [orig::bump]:
    Source: 'src/main.rs', lines 1:0-16:1 *)
Definition bump
  {N : usize} (coremarkerCopyArrayI32NInst : core_marker_Copy (array i32 N))
  (arr : array i32 N) (k : usize) (mark : bool) :
  result ((array i32 N) * bool)
  :=
  if k s< N then bump_loop0 k arr mark 0%usize else bump_loop1 arr mark 0%usize
.

(** [orig::main]:
    Source: 'src/main.rs', lines 18:0-24:1 *)
Definition main : result unit :=
  p <-
    bump (core_marker_CopyArray 4%usize core_marker_CopyI32)
      (mk_array 4%usize [ 0%i32; 0%i32; 5%i32; 9%i32 ]) 3%usize false;
  let (a, touched_zero) := p in
  b <-
    core_array_equality_PartialEqArrayArray_eq core_cmp_PartialEqI32 a
      (mk_array 4%usize [ 1%i32; 1%i32; 6%i32; 9%i32 ]);
  if b then massert touched_zero else Fail_ Failure
.

End Orig.
