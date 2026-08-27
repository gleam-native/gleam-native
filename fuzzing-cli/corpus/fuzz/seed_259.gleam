pub const limit_value: Int = 10
pub const seed_value: String = "ab"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: #(Int, String), this_: String, v0: #(Float, Int)) -> Float {
case <<"abc":utf8, "res":utf8>>, "constructor" {
    <<_:utf8>> as whole, "constructor" -> 100.0
    <<5:8, _:little-signed-16, _:utf8>>, "res" as whole if whole != "res" -> fn(v1) { {
      0.1
    } /. {
      2.0
    } }(True)
    _, "ab" <> rest -> 0.1
    v2, v3 -> {
      {
        2.0
      } -. {
        0.1
      }
    } -. {
      {
        0.1
      } *. {
        1.0
      }
    }
  }
}

pub fn main() {
  let pair = [4]
  let pair = case {
      let rest = False
      #(True, 100.0)
    }, pair {
    #(False, 0.1), [6, 0, ..] as whole -> limit_value
    #(False, v4) as whole, [3, ..rest] as it -> limit_value - 5
    _, v5 -> limit_value * limit_value
  }
  echo case [2, 2] {
    [pair, ..rest] -> {
      let value = seed_value
      let rest = {
        0.5
      } >. {
        1.5
      }
      []
    }
    [] -> case [] |> walk(10) {
      3 | 2 -> {
        let l = True
        let pair = False
        [3, 1]
      }
      item -> fn(v6, v7) { [2] }(False, "x")
    }
    [] -> {
      let seed_value = walk([1], limit_value)
      [42]
    }
    v8 -> case "x", seed_value <> seed_value {
      "constructor", v9 if v9 != "bc" -> [42, 10]
      "ab", "bc" <> rest -> v8
      v10, v11 -> []
    }
  }
  echo case pair, fn(v12, v13) { pair }(2.0, False) {
    _, 1 as whole -> f0(#(2, "data"), seed_value, {
      let z = [0, 10]
      #(0.25, 7)
    })
    9, v14 -> 0.25
    _, v15 -> 3.14
  }
  echo fn(v16) { {
    seed_value <> seed_value
  } <> {
    fn(v17) { "x" }(True)
  } }(False)
  echo {
    fn(v18) { f0(#(5, "data"), seed_value, #(3.14, 4)) }("ab")
  } >=. {
    case limit_value {
      9 | 5 -> {
        1.5
      } -. {
        0.25
      }
      4 as whole if whole <= 5 -> 0.0
      _ -> {
        let z = "bc"
        0.5
      }
    }
  }
}
