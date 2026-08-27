pub const golden_value: Float = 100.0
pub const euler_value: Bool = True

fn f0(m: String) -> List(Int) {
case [2] {
    [m, _, ..] -> case {
        let m = 1.5
        let item = [2]
        42
      } {
      inner -> fn(v0, v1) { [2, 7] }(5, 10)
      9 | 5 -> [4, 5]
      v2 -> [1]
    }
    [1, 2, ..] -> case m <> m {
      inner | "x" <> inner -> [3, 1]
      "ab" <> _ -> []
    }
    v3 -> case "abc" <> m {
      "x" -> [0, 7]
      "a" | "bc" <> _ -> v3
      _ -> [3]
    }
  }
}

fn f1(length: Int, y: String) -> Float {
{
    case length * 42 {
      y -> 100.0
      a -> {
        let length = 2
        0.5
      }
    }
  } -. {
    fn(v4) { v4 }(1.5)
  }
}

pub fn main() {
  let default = {
    fn(v5) { "ab" }(True)
  } <> {
    fn(v6) { "data" }(10.0)
  }
  let constructor = 3
  echo {
    case "abc" <> default, {
        let y = 100
        let m = golden_value
        "abc"
      } {
      "a" <> rest, new if rest == "a" -> "data"
      "b", golden_value -> {
        let x = 0
        "x"
      }
      v7, _ -> {
        let euler_value = True
        let self_ = constructor
        v7
      }
    }
  } <> {
    case {
        let l = 10.0
        default
      } {
      "bc" | "res" -> default <> "a"
      _ -> {
        let n = ""
        default
      }
      _ -> "bc"
    }
  }
  echo []
  echo case "b" {
    "" <> inner | "data" <> inner -> "res"
    "b" <> item | "b" <> item -> {
      item <> "b"
    } <> {
      default <> "ab"
    }
    default -> default
  }
}
