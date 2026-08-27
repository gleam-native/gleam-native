pub const golden_value: Float = 10.0
pub const euler_value: Bool = True

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, v0: Int, v1: Int) -> List(Int) {
case "x" {
    b -> case b {
      "res" -> {
        let l = True
        [10, 3]
      }
      "abc" -> [1]
      _ -> []
    }
    inner -> fn(v2, v3) { [] }(0, True)
    constructor -> []
  }
}

fn f1(v4: Int, n: String, arguments: Int) -> Int {
1 + {
    case v4 % 5 {
      constructor -> 1
      2 -> 1
      _ -> 5
    }
  }
}

fn extends(self_: Bool) -> Int {
walk({
    let self_ = "a" == "res"
    let new = {
      0.0
    } == {
      10.0
    }
    [42]
  }, case "" {
    x -> fn(v5) { 3 }(True)
    "data" <> _ -> {
      let self_ = [1, 3]
      let pair = 0.5
      4
    }
    "bc" <> rest | "a" <> rest -> 2 - 10
  })
}

pub fn main() {
  echo case <<"b":utf8>> {
    <<4:16, _:little-unsigned-4, "":utf8>> -> {
      4 + 100
    } + {
      3 + 1
    }
    <<3:16, 2:1>> -> case "res" {
      item | "res" <> item -> 3 - 1
      v6 -> [4, 42] |> walk(1 % 2)
      "bc" as whole -> extends(euler_value)
    }
    _ -> 42
  }
}
