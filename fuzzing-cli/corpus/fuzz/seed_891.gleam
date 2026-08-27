pub const pi_value: Int = 3
pub const seed_value: Int = 5

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(item: Bool) -> List(Int) {
[42, 7]
}

fn new(z: String, v0: String) -> Int {
case z <> "b" {
    "ab" -> 10
    a | "" <> a -> case 42 {
      8 as whole -> whole
      7 as whole -> {
        let rest = []
        let v = rest
        whole
      }
      _ -> 0
    }
    item -> 2
  }
}

pub fn main() {
  echo case "bc" {
    "x" <> rest | "" <> rest -> case {
        let this_ = []
        let m = ""
        0.5
      } {
      _ -> [42]
      pi_value -> [7, 5]
      _ -> []
    }
    _ | "x" -> constructor(False)
    "data" -> []
  }
  echo {
    let class = fn(v1) { spin(2, 7) }(True)
    let l = {
      let class = {
        let seed_value = True
        let new = seed_value
        pi_value
      }
      let prototype = [100]
      {
        100.0
      } +. {
        3.14
      }
    }
    {
      "b" <> "bc"
    } == "ab"
  }
  echo "constructor"
}
