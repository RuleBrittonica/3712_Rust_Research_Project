(** [extract::bump_extracted]:
    Source: 'src/main.rs', lines 1:0-8:1 *)
Definition bump_extracted
  {N : usize} (coremarkerCopyArrayI32NInst : core_marker_Copy (array i32 N))
  (arr : array i32 N) (i : usize) (mark : bool) :
  result unit
  :=
  i1 <- array_index_usize arr i;
  if i1 s= 0%i32
  then (_ <- i32_add i1 1%i32; _ <- array_index_mut_usize arr i; Ok tt)
  else (_ <- i32_add i1 1%i32; _ <- array_index_mut_usize arr i; Ok tt)
.

(** [extract::bump]: loop 0:
    Source: 'src/main.rs', lines 16:4-19:5 *)
Fixpoint bump_loop0
  {N : usize} (coremarkerCopyArrayI32NInst : core_marker_Copy (array i32 N))
  (arr : array i32 N) (k : usize) (mark : bool) (i : usize) :
  result unit
  :=
  if i s< k
  then (
    a <- coremarkerCopyArrayI32NInst.(cloneInst).(core_clone_Clone_clone) arr;
    _ <- bump_extracted coremarkerCopyArrayI32NInst a i mark;
    i1 <- usize_add i 1%usize;
    bump_loop0 coremarkerCopyArrayI32NInst arr k mark i1)
  else Ok tt
.

(** [extract::bump]: loop 1:
    Source: 'src/main.rs', lines 16:4-19:5 *)
Fixpoint bump_loop1
  {N : usize} (coremarkerCopyArrayI32NInst : core_marker_Copy (array i32 N))
  (arr : array i32 N) (mark : bool) (i : usize) :
  result unit
  :=
  if i s< N
  then (
    a <- coremarkerCopyArrayI32NInst.(cloneInst).(core_clone_Clone_clone) arr;
    _ <- bump_extracted coremarkerCopyArrayI32NInst a i mark;
    i1 <- usize_add i 1%usize;
    bump_loop1 coremarkerCopyArrayI32NInst arr mark i1)
  else Ok tt
.

(** [extract::bump]:
    Source: 'src/main.rs', lines 9:0-20:1 *)
Definition bump
  {N : usize} (coremarkerCopyArrayI32NInst : core_marker_Copy (array i32 N))
  (arr : array i32 N) (k : usize) (mark : bool) :
  result ((array i32 N) * bool)
  :=
  if k s< N
  then (
    _ <- bump_loop0 coremarkerCopyArrayI32NInst arr k mark 0%usize;
    Ok (arr, mark))
  else (
    _ <- bump_loop1 coremarkerCopyArrayI32NInst arr mark 0%usize;
    Ok (arr, mark))
.

(** [extract::main]:
    Source: 'src/main.rs', lines 21:0-27:1 *)
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

End Extract.
