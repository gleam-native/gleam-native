pub const seed_value: Float = 0.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(class: Int, prototype: List(Int)) -> Int {
{
    let prototype = False || {
      fn(v2) { False }(1.0)
    }
    fn(v3, v4) { v3 }(2, "constructor")
  }
}

pub fn main() {
  echo [2]
  echo 7
  echo {
    case 5 + 1, fn(v5, v6) { 5 }(True, "ab") {
      2, l -> l
      _, _ -> 10
    }
  } - {
    {
      let y = "x"
      10
    }
  }
  echo {
    case "constructor" {
      "" <> a -> {
        100.0
      } +. seed_value
      "abc" -> seed_value
      constructor -> 1.5
    }
  } >. {
    case fn(v7, v8) { "bc" }(1.0, 3) {
      inner | "a" <> inner -> {
        let inner = True
        let length = inner
        seed_value
      }
      inner -> seed_value
    }
  }
}
