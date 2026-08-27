pub const tag_value: String = "x"
pub const golden_value: String = "x"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Float, new: Int, v0: List(Int)) -> Int {
4
}

fn f1(arguments: String) -> String {
{
    {
      arguments <> "x"
    } <> {
      arguments <> arguments
    }
  } <> {
    arguments <> ""
  }
}

pub fn main() {
  echo {
    case False {
      golden_value -> fn(v1) { v1 }(0.1)
      True -> {
        let default = golden_value
        1.5
      }
      True -> {
        0.5
      } -. {
        2.0
      }
    }
  } +. {
    fn(v2, v3) { {
      let pair = golden_value
      let l = golden_value
      0.1
    } }(1.5, 3)
  }
  echo [100, 0]
  echo case <<"":utf8, 0:8>> {
    <<1:16>> -> [3]
    <<5:16>> as whole -> [3, 4]
    _ -> [0, 10]
  }
  echo case spin(100, 42), #(2.0, [5, 1]) {
    _, #(3.14, []) -> {
      let item = 1.5
      1
    }
    3, #(1.5, [0]) as whole -> 100 + {
      1 + 100
    }
    _, _ -> {
      {
        let n = golden_value
        let n = False
        2
      }
    } + 3
  }
}
