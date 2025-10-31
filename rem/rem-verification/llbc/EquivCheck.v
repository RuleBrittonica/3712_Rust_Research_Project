(** EquivCheck.v **)
Require Import Primitives.
Import Primitives.
Require Import SampleProject.
Require Import SampleProjectRef.
Require Import Coq.ZArith.ZArith.
Local Open Scope Primitives_scope.

(**
   Lemma: For all x, the function ref_incr in SampleProject is equal to
   ref_incr in SampleProjectRef.

   In SampleProject, ref_incr is defined directly:
      ref_incr (x : i32) := i32_add x 1%i32.
   In SampleProjectRef, ref_incr is defined as:
      ref_incr (x : i32) := fun_name x,
   and fun_name is defined as:
      fun_name (x : i32) := i32_add x 1%i32.

   Thus, after unfolding, both sides reduce to the same expression.
**)
Lemma ref_incr_equiv : forall (x : i32),
  SampleProject.ref_incr x = SampleProjectRef.ref_incr x.
Proof.
  intros x.
  unfold SampleProject.ref_incr.
  unfold SampleProjectRef.ref_incr.
  reflexivity.
Qed.

Goal True.
  idtac "Proofs succeeded!".
  exact I.
Qed.
