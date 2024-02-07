(module
  (import "pybadge" "echo_i32" (func $echo_i32 (param i32)))
  (memory 1)
  (export "update" (func $update))
  (global $g (mut i32) (i32.const 0))
  (func $update
    (global.get $g)
    i32.const 1
    i32.add
    (global.set $g)
    (global.get $g)
    call $echo_i32
  )
)
