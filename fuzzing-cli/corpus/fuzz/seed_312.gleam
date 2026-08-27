pub const tag_value: Float = 1.0
pub const seed_value: Float = 0.1

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Error(Int, Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn export(class: Int, x: List(Int)) -> String {
{
    case "b" {
      _ | "x" <> _ -> "a" <> "b"
      a | "ab" <> a -> "res"
      b -> b <> ""
    }
  } <> {
    case True {
      False -> "abc"
      False -> "x" <> "res"
      v2 -> "ab" <> "res"
    }
  }
}

pub fn main() {
  let class = fn(v3) { [] }("")
  echo tag_value
  echo 3
  echo case class, fn(v4) { "bc" }(0) {
    [_, ..rest], "a" <> tail -> export(10, class)
    [7], "bc" -> {
      let tag_value = "a" <> ""
      let m = False
      "constructor"
    }
    v5, _ -> case 5, {
        let arguments = "res"
        4
      } {
      0, _ -> "b" <> "bc"
      5, 9 -> export(0, class)
      3, _ -> "abc"
      v6, _ -> "ab" <> "res"
    }
  }
}
