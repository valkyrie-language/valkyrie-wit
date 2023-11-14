(func $offsetFromCoordinate (param $x i32) (param $y i32) (result i32)
  (i32.mul
    (i32.add
      (i32.mul
        (i32.const 50)
        (get_local $y)
      )
      (get_local $x)
    )
    (i32.const 4)
  )
)