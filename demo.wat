(module
  (import "pybadge" "echo_i32" (func $echo_i32 (param i32)))
  (func $update
    i32.const 10
    i32.const 3
    i32.add
    call $echo_i32
  )
)
