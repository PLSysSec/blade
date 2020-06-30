(module
  (import "js" "memory" (memory 1))
  (func (export "encrypt")
    ;; message:
    (local $v0 i32)
    (local $v1 i32)
    ;; key:
    (local $k0 i32)
    (local $k1 i32)
    (local $k2 i32)
    (local $k3 i32)

    ;; local variables
    (local $delta i32)
    (local $sum i32)
    (local $i i32)

    ;; init
    (set_local $delta (i32.const 0x9e3779b9))
    (set_local $sum (i32.const 0))

    ;; load message and key
    (set_local $v0 (i32.load (i32.const 0)))
    (set_local $v1 (i32.load (i32.const 4)))
    (set_local $k0 (i32.load (i32.const 8)))
    (set_local $k1 (i32.load (i32.const 12)))
    (set_local $k2 (i32.load (i32.const 16)))
    (set_local $k3 (i32.load (i32.const 20)))

    ;; loop starts
    (set_local $i (i32.const 0))
    (loop $cycle
      ;; loop body:
      ;; sum += delta;
      (set_local $sum (i32.add (get_local $sum) (get_local $delta)))
      ;; v0 += ((v1<<4) + k0) ^ (v1 + sum) ^ ((v1>>5) + k1);
      (set_local $v0
         (i32.add (get_local $v0)
                  (i32.xor (i32.xor (i32.add (i32.shl (get_local $v1) (i32.const 4)) (get_local $k0))
                                    (i32.add (get_local $v1) (get_local $sum)))
                           (i32.add (i32.shr_u (get_local $v1) (i32.const 5)) (get_local $k1))
               )))
      ;; v1 += ((v0<<4) + k2) ^ (v0 + sum) ^ ((v0>>5) + k3);
      (set_local $v1
         (i32.add (get_local $v1)
                  (i32.xor (i32.xor (i32.add (i32.shl (get_local $v0) (i32.const 4)) (get_local $k2))
                                    (i32.add (get_local $v0) (get_local $sum)))
                           (i32.add (i32.shr_u (get_local $v0) (i32.const 5)) (get_local $k3))
               )))
      ;; loop condition:
      (set_local $i (i32.add (get_local $i) (i32.const 1)))
      (br_if $cycle (i32.lt_u (get_local $i) (i32.const 32)))
    )
    (i32.store (i32.const 0) (get_local $v0))
    (i32.store (i32.const 4) (get_local $v1))
  )

  (func (export "decrypt")
    ;; message:
    (local $v0 i32)
    (local $v1 i32)
    ;; key:
    (local $k0 i32)
    (local $k1 i32)
    (local $k2 i32)
    (local $k3 i32)

    ;; local variables
    (local $delta i32)
    (local $sum i32)
    (local $i i32)

    ;; init
    (set_local $delta (i32.const 0x9e3779b9))
    (set_local $sum (i32.const 0xc6ef3720))

    ;; load message and key
    (set_local $v0 (i32.load (i32.const 0)))
    (set_local $v1 (i32.load (i32.const 4)))
    (set_local $k0 (i32.load (i32.const 8)))
    (set_local $k1 (i32.load (i32.const 12)))
    (set_local $k2 (i32.load (i32.const 16)))
    (set_local $k3 (i32.load (i32.const 20)))

    ;; loop starts
    (set_local $i (i32.const 0))
    (loop $cycle
      ;; loop body:
      ;; v1 -= ((v0<<4) + k2) ^ (v0 + sum) ^ ((v0>>5) + k3);
      (set_local $v1
         (i32.sub (get_local $v1)
                  (i32.xor (i32.xor (i32.add (i32.shl (get_local $v0) (i32.const 4)) (get_local $k2))
                                    (i32.add (get_local $v0) (get_local $sum)))
                           (i32.add (i32.shr_u (get_local $v0) (i32.const 5)) (get_local $k3))
               )))
      ;; v0 -= ((v1<<4) + k0) ^ (v1 + sum) ^ ((v1>>5) + k1);
      (set_local $v0
         (i32.sub (get_local $v0)
                  (i32.xor (i32.xor (i32.add (i32.shl (get_local $v1) (i32.const 4)) (get_local $k0))
                                    (i32.add (get_local $v1) (get_local $sum)))
                           (i32.add (i32.shr_u (get_local $v1) (i32.const 5)) (get_local $k1))
               )))
      ;; sum -= delta;
      (set_local $sum (i32.sub (get_local $sum) (get_local $delta)))
      ;; loop condition:
      (set_local $i (i32.add (get_local $i) (i32.const 1)))
      (br_if $cycle (i32.lt_u (get_local $i) (i32.const 32)))
    )
    (i32.store (i32.const 0) (get_local $v0))
    (i32.store (i32.const 4) (get_local $v1))
  )
)
