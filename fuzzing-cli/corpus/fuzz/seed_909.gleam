fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v0: #(String, Bool), v1: Int) -> Int {
fn(v2) { 100 }("data")
}

pub fn main() {
  echo {
    let l = "bc"
    case {
        let default = 0
        l
      } {
      new -> [7, 1]
      "res" <> rest if rest == "b" -> {
        let z = 3.14
        let l = 0.25
        []
      }
      constructor -> []
    }
  }
  echo 2
  echo {
    let acc = case False {
      True -> {
        0.1
      } == {
        0.5
      }
      False | True -> 42 < 4
      True -> True
    }
    case "abc" {
      inner -> {
        3.14
      } +. {
        100.0
      }
      v3 -> {
        1.0
      } *. {
        100.0
      }
      "abc" <> rest -> 10.0
    }
  }
}
