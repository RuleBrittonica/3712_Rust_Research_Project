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
