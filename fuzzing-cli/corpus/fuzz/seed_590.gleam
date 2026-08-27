pub const euler_value: Int = 100

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, v0: List(Int), this_: String) -> List(Int) {
{
    let pair = []
    case 3.14, "constructor" <> "b" {
      v1, acc -> {
        let self_ = "constructor"
        v0
      }
      v2, "x" -> [10]
    }
  }
}

pub fn main() {
  let euler_value = {
    "data" <> ""
  } <> {
    fn(v3) { "bc" }(4)
  }
  let euler_value = case 10 {
    b -> {
      0.5
    } *. {
      3.14
    }
    _ -> {
      100.0
    } +. {
      3.14
    }
  }
  echo [42, 3]
  echo case fn(v4, v5) { 42 }(True, False) {
    constructor -> spin({
      let this_ = euler_value
      constructor
    }, 1 - constructor)
    constructor -> case spin(constructor, constructor), !True {
      9, _ -> spin(2, constructor)
      v6, _ -> {
        let l = [1, 7]
        constructor
      }
    }
    arguments -> 0 + {
      1 - arguments
    }
  }
}
