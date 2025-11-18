Definition bump
  {N : usize} (coremarkerCopyArrayI32NInst : core_marker_Copy (array i32 N))
  (arr : array i32 N) (k : usize) (mark : bool) :
  result ((array i32 N) * bool)
  :=
  if k s< N then bump_loop0 k arr mark 0%usize else bump_loop1 arr mark 0%usize
.

(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)
(* *)