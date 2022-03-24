(set-option :auto_config false)
(set-option :smt.mbqi false)
(set-option :smt.case_split 3)
(set-option :smt.qi.eager_threshold 100.0)
(set-option :smt.delay_units true)
(set-option :smt.arith.solver 2)
(set-option :smt.arith.nl false)

; forall x y : x == 3 ==> y == 4 ==> x != y
; !exists x y : !(x == 3 ==> y == 4 ==> x != y)
(push)
 (declare-const x@ Int)
 (declare-const y@ Int)
 (assert
  (not (=>
    (= x@ 3)
    (=>
     (= y@ 4)
     (not (= x@ y@)))))
 )
 (check-sat)
(pop)


