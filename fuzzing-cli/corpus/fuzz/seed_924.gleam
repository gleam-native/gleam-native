pub const seed_value: Bool = True
pub const euler_value: Bool = True
pub const tag_value: Float = 100.0

pub type V0 {
  Ok(value: String, inner: Bool)
  Cv1
  None
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(arguments: Int, this_: Int) -> Bool {
True
}

pub fn main() {
  let z = case {
      let length = tag_value
      let tag_value = "constructor"
      "x"
    } {
    "b" -> [2]
    item | "a" <> item -> [10]
    b | "a" <> b -> {
      let class = euler_value
      [1]
    }
  }
  let seed_value = "data"
  echo z
  echo spin(0, {
    fn(v2, v3) { v2 }(1, True)
  } + {
    5 |> spin(fn(v4) { 4 }(100.0))
  })
  echo case Ok("constructor", False) {
    Ok(_, a) if a && a -> case 3 + 42, 5 {
      _, _ -> tag_value
      4 as whole, v5 if v5 > 5 -> {
        100.0
      } -. tag_value
      3, tag_value -> 0.5
    }
    Ok("a", False) -> case #(False, [1, 42]) {
      #(False as whole, []) if whole -> {
        0.1
      } +. {
        100.0
      }
      #(True, [3, x, ..]) -> tag_value /. {
        10.0
      }
      #(False, [b]) -> tag_value
      v6 -> tag_value
    }
    Ok("abc", v7) as whole -> {
      {
        1.5
      } /. {
        2.0
      }
    } -. {
      fn(v8, v9) { 1.0 }(1, 10.0)
    }
    v10 -> {
      let v10 = {
        let euler_value = "res"
        [100]
      }
      fn(v11, v12) { tag_value }(5, 0)
    }
  }
}
