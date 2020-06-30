(module
  (memory (export "memory") 1)
  ;; mem[0..3] datalen, mem[4..11] bitlen, mem[12..43] state
  ;; mem[44..299] m, mem[300..363] data, mem[364..619] k
  ;; mem[620..651] hash (output), mem[652..] input

  (func $init
    (local $i i32)
    (i32.store (i32.const 0) (i32.const 0)) ;; datalen = 0
    (i64.store (i32.const 4) (i64.const 0)) ;; bitlen = 0
    ;; clear old m
    (set_local $i (i32.const 0))
    (block
      (loop
        (br_if 1 (i32.ge_u (get_local $i) (i32.const 64)))
          (i32.store (i32.add (i32.const 44) (i32.mul (get_local $i) (i32.const 4))) (i32.const 0))
          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (br 0)
      )
    )
    ;; clear old data
    (set_local $i (i32.const 0))
    (block
      (loop
        (br_if 1 (i32.ge_u (get_local $i) (i32.const 64)))
          (i32.store8 (i32.add (i32.const 300) (get_local $i)) (i32.const 0))
          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (br 0)
      )
    )
    ;; clear old hash
    (set_local $i (i32.const 0))
    (block
      (loop
        (br_if 1 (i32.ge_u (get_local $i) (i32.const 32)))
          (i32.store8 (i32.add (i32.const 620) (get_local $i)) (i32.const 0))
          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (br 0)
      )
    )
    (i32.store (i32.const 12) (i32.const 0x6a09e667))
    (i32.store (i32.const 16) (i32.const 0xbb67ae85))
    (i32.store (i32.const 20) (i32.const 0x3c6ef372))
    (i32.store (i32.const 24) (i32.const 0xa54ff53a))
    (i32.store (i32.const 28) (i32.const 0x510e527f))
    (i32.store (i32.const 32) (i32.const 0x9b05688c))
    (i32.store (i32.const 36) (i32.const 0x1f83d9ab))
    (i32.store (i32.const 40) (i32.const 0x5be0cd19))
  )

  (func $transform
    (local $a i32)
    (local $b i32)
    (local $c i32)
    (local $d i32)
    (local $e i32)
    (local $f i32)
    (local $g i32)
    (local $h i32)
    (local $i i32)
    (local $j i32)
    (local $t1 i32)
    (local $t2 i32)
    (local $m i32)

    (set_local $m (i32.const 44))
    (set_local $i (i32.const 0))
    (set_local $j (i32.const 0))
    (block
      (loop
        (br_if 1 (i32.ge_u (get_local $i) (i32.const 16)))
          (i32.store
            (i32.add (get_local $m) (i32.mul (get_local $i) (i32.const 4)))
            (i32.or
              (i32.shl (i32.load8_u (i32.add (i32.const 300) (get_local $j))) (i32.const 24))
              (i32.or
                (i32.shl (i32.load8_u (i32.add (i32.const 300) (i32.add (get_local $j) (i32.const 1)))) (i32.const 16))
                (i32.or
                  (i32.shl (i32.load8_u (i32.add (i32.const 300) (i32.add (get_local $j) (i32.const 2)))) (i32.const 8))
                  (i32.load8_u (i32.add (i32.const 300) (i32.add (get_local $j) (i32.const 3))))
                )
              )
            )
          )
          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (set_local $j (i32.add (get_local $j) (i32.const 4)))
          (br 0)
      )
    )
    (block
      (loop
        (br_if 1 (i32.ge_u (get_local $i) (i32.const 64)))
          (i32.store
            (i32.add (get_local $m) (i32.mul (get_local $i) (i32.const 4)))
            (i32.add
              (i32.load (i32.add (get_local $m) (i32.mul (i32.sub (get_local $i) (i32.const 7)) (i32.const 4))))
              (i32.add
                (i32.load (i32.add (get_local $m) (i32.mul (i32.sub (get_local $i) (i32.const 16)) (i32.const 4))))
                (i32.add
                  ;; SIG1(m[i - 2])
                  (i32.xor
                    (i32.rotr (i32.load (i32.add (get_local $m) (i32.mul (i32.sub (get_local $i) (i32.const 2)) (i32.const 4)))) (i32.const 17))
                    (i32.xor
                      (i32.rotr (i32.load (i32.add (get_local $m) (i32.mul (i32.sub (get_local $i) (i32.const 2)) (i32.const 4)))) (i32.const 19))
                      (i32.shr_u (i32.load (i32.add (get_local $m) (i32.mul (i32.sub (get_local $i) (i32.const 2)) (i32.const 4)))) (i32.const 10))
                    )
                  )
                  ;; SIG0(m[i - 15])
                  (i32.xor
                    (i32.rotr (i32.load (i32.add (get_local $m) (i32.mul (i32.sub (get_local $i) (i32.const 15)) (i32.const 4)))) (i32.const 7))
                    (i32.xor
                      (i32.rotr (i32.load (i32.add (get_local $m) (i32.mul (i32.sub (get_local $i) (i32.const 15)) (i32.const 4)))) (i32.const 18))
                      (i32.shr_u (i32.load (i32.add (get_local $m) (i32.mul (i32.sub (get_local $i) (i32.const 15)) (i32.const 4)))) (i32.const 3))
                    )
                  )
                )
              )
            )
          )
          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (br 0)
      )
    )

    (set_local $a (i32.load (i32.const 12)))
    (set_local $b (i32.load (i32.const 16)))
    (set_local $c (i32.load (i32.const 20)))
    (set_local $d (i32.load (i32.const 24)))
    (set_local $e (i32.load (i32.const 28)))
    (set_local $f (i32.load (i32.const 32)))
    (set_local $g (i32.load (i32.const 36)))
    (set_local $h (i32.load (i32.const 40)))

    (set_local $i (i32.const 0))
    (block
      (loop
        (br_if 1 (i32.ge_u (get_local $i) (i32.const 64)))
          (set_local $t1
            (i32.add
              (get_local $h)
              (i32.add
                (i32.load (i32.add (i32.const 364) (i32.mul (get_local $i) (i32.const 4))))
                (i32.add
                  (i32.load (i32.add (i32.const 44) (i32.mul (get_local $i) (i32.const 4))))
                  (i32.add
                    ;; EP1(e)
                    (i32.xor
                      (i32.rotr (get_local $e) (i32.const 6))
                      (i32.xor
                        (i32.rotr (get_local $e) (i32.const 11))
                        (i32.rotr (get_local $e) (i32.const 25))
                      )
                    )
                    ;; CH(e,f,g)
                    (i32.xor
                      (i32.and (get_local $e) (get_local $f))
                      (i32.and (i32.xor (get_local $e) (i32.const -1)) (get_local $g))
                    )
                  )
                )
              )
            )
          )
          (set_local $t2
            (i32.add
              ;; EP0(a)
              (i32.xor
                (i32.rotr (get_local $a) (i32.const 2))
                (i32.xor
                  (i32.rotr (get_local $a) (i32.const 13))
                  (i32.rotr (get_local $a) (i32.const 22))
                )
              )
              ;; MAJ(a,b,c)
              (i32.xor
                (i32.and (get_local $a) (get_local $b))
                (i32.xor
                  (i32.and (get_local $a) (get_local $c))
                  (i32.and (get_local $b) (get_local $c))
                )
              )
            )
          )
          (set_local $h (get_local $g))
          (set_local $g (get_local $f))
          (set_local $f (get_local $e))
          (set_local $e (i32.add (get_local $d) (get_local $t1)))
          (set_local $d (get_local $c))
          (set_local $c (get_local $b))
          (set_local $b (get_local $a))
          (set_local $a (i32.add (get_local $t1) (get_local $t2)))
          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (br 0)
      )
    )

    (i32.store (i32.const 12) (i32.add (i32.load (i32.const 12)) (get_local $a)))
    (i32.store (i32.const 16) (i32.add (i32.load (i32.const 16)) (get_local $b)))
    (i32.store (i32.const 20) (i32.add (i32.load (i32.const 20)) (get_local $c)))
    (i32.store (i32.const 24) (i32.add (i32.load (i32.const 24)) (get_local $d)))
    (i32.store (i32.const 28) (i32.add (i32.load (i32.const 28)) (get_local $e)))
    (i32.store (i32.const 32) (i32.add (i32.load (i32.const 32)) (get_local $f)))
    (i32.store (i32.const 36) (i32.add (i32.load (i32.const 36)) (get_local $g)))
    (i32.store (i32.const 40) (i32.add (i32.load (i32.const 40)) (get_local $h)))
  )

  (func $update (param $inputlen i32)
    (local $i i32)

    (set_local $i (i32.const 0))
    (block
      (loop
        (br_if 1 (i32.ge_u (get_local $i) (get_local $inputlen)))
          (i32.store8
            (i32.add (i32.const 300) (i32.load (i32.const 0)))
            (i32.load8_u (i32.add (i32.const 652) (get_local $i)))
          )
          (i32.store (i32.const 0) (i32.add (i32.load (i32.const 0)) (i32.const 1)))
          (if (i32.eq (i32.load (i32.const 0)) (i32.const 64))
            (then
              (call $transform)
              (i64.store (i32.const 4) (i64.add (i64.load (i32.const 4)) (i64.const 512)))
              (i32.store (i32.const 0) (i32.const 0))
            )
          )
          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (br 0)
      )
    )
  )

  (func $final
    (local $i i32)

    (set_local $i (i32.load (i32.const 0)))
    (if (i32.lt_u (get_local $i) (i32.const 56))
      (then
        (i32.store8 (i32.add (i32.const 300) (get_local $i)) (i32.const 0x80))
        (set_local $i (i32.add (get_local $i) (i32.const 1)))
        (block
          (loop
            (br_if 1 (i32.ge_u (get_local $i) (i32.const 56)))
              (i32.store8 (i32.add (i32.const 300) (get_local $i)) (i32.const 0x00))
              (set_local $i (i32.add (get_local $i) (i32.const 1)))
              (br 0)
          )
        )
      )
      (else
        (i32.store8 (i32.add (i32.const 300) (get_local $i)) (i32.const 0x80))
        (set_local $i (i32.add (get_local $i) (i32.const 1)))
        (block
          (loop
            (br_if 1 (i32.ge_u (get_local $i) (i32.const 64)))
              (i32.store8 (i32.add (i32.const 300) (get_local $i)) (i32.const 0x00))
              (set_local $i (i32.add (get_local $i) (i32.const 1)))
              (br 0)
          )
        )
        (call $transform)
        ;; memset(data, 0, 56)
        (set_local $i (i32.const 0))
        (block
          (loop
            (br_if 1 (i32.ge_u (get_local $i) (i32.const 56)))
              (i32.store8 (i32.add (i32.const 300) (get_local $i)) (i32.const 0x00))
              (set_local $i (i32.add (get_local $i) (i32.const 1)))
              (br 0)
          )
        )
      )
    )
    (i64.store
      (i32.const 4)
      (i64.add
        (i64.load (i32.const 4))
        (i64.mul (i64.extend_u/i32 (i32.load (i32.const 0))) (i64.const 8))
      )
    )
    (i32.store8 (i32.const 356) (i32.load8_u (i32.const 11)))
    (i32.store8 (i32.const 357) (i32.load8_u (i32.const 10)))
    (i32.store8 (i32.const 358) (i32.load8_u (i32.const 9)))
    (i32.store8 (i32.const 359) (i32.load8_u (i32.const 8)))
    (i32.store8 (i32.const 360) (i32.load8_u (i32.const 7)))
    (i32.store8 (i32.const 361) (i32.load8_u (i32.const 6)))
    (i32.store8 (i32.const 362) (i32.load8_u (i32.const 5)))
    (i32.store8 (i32.const 363) (i32.load8_u (i32.const 4)))
    (call $transform)
    (set_local $i (i32.const 0))
    (block
      (loop
        (br_if 1 (i32.ge_u (get_local $i) (i32.const 4)))
          (i32.store8
            (i32.add (i32.const 620) (get_local $i))
            (i32.load8_u (i32.sub (i32.const 15) (get_local $i)))
          )
          (i32.store8
            (i32.add (i32.const 620) (i32.add (get_local $i) (i32.const 4)))
            (i32.load8_u (i32.sub (i32.const 19) (get_local $i)))
          )
          (i32.store8
            (i32.add (i32.const 620) (i32.add (get_local $i) (i32.const 8)))
            (i32.load8_u (i32.sub (i32.const 23) (get_local $i)))
          )
          (i32.store8
            (i32.add (i32.const 620) (i32.add (get_local $i) (i32.const 12)))
            (i32.load8_u (i32.sub (i32.const 27) (get_local $i)))
          )
          (i32.store8
            (i32.add (i32.const 620) (i32.add (get_local $i) (i32.const 16)))
            (i32.load8_u (i32.sub (i32.const 31) (get_local $i)))
          )
          (i32.store8
            (i32.add (i32.const 620) (i32.add (get_local $i) (i32.const 20)))
            (i32.load8_u (i32.sub (i32.const 35) (get_local $i)))
          )
          (i32.store8
            (i32.add (i32.const 620) (i32.add (get_local $i) (i32.const 24)))
            (i32.load8_u (i32.sub (i32.const 39) (get_local $i)))
          )
          (i32.store8
            (i32.add (i32.const 620) (i32.add (get_local $i) (i32.const 28)))
            (i32.load8_u (i32.sub (i32.const 43) (get_local $i)))
          )
          (set_local $i (i32.add (get_local $i) (i32.const 1)))
          (br 0)
      )
    )
  )

  (export "init" (func $init))
  (export "update" (func $update))
  (export "final" (func $final))
)
