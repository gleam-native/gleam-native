pub const tag_value: Bool = True

fn arguments(constructor: Int, z: Bool, pair: Bool) -> List(Int) {
case {
      3.14
    } -. {
      0.1
    } {
    a -> [3, 0]
    _ -> case #(1, 100.0), "" {
      #(6, 0.1) as whole, prototype if prototype != "constructor" || prototype == "bc" -> fn(v0) { [7, 42] }(False)
      #(5, 3.14) as whole, _ -> fn(v1, v2) { [0] }("abc", "res")
      #(1, 1.0), _ -> [7, 3]
      _, _ -> []
    }
  }
}

fn default(v3: #(List(Int), List(Int)), v4: Float, n: Float) -> List(Int) {
{
    case [] {
      [4] -> 0
      [1, ..rest] -> 7
      v5 -> 4
    }
  } |> arguments(fn(v6, v7) { True }(3.14, "x"), fn(v8, v9) { True }(5, "bc"))
}

pub fn main() {
  let new = case fn(v10, v11) { 4 }(False, False) {
    constructor -> "res"
    x -> "x"
  }
  let y = {
    {
      1.0
    } /. {
      0.5
    }
  } -. {
    {
      0.0
    } *. {
      1.5
    }
  }
  echo {
    let length = fn(v12, v13) { {
      let length = 2
      let class = "x"
      [3]
    } }("res", 10.0)
    let arguments = case {
        let y = length
        10
      } {
      _ | 0 -> [4, 3]
      4 -> 42 |> arguments(fn(v14) { True }(10), y >. {
        0.5
      })
      a -> []
    }
    arguments
  }
}
