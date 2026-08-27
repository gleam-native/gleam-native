pub const seed_value: Bool = True

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(l: #(String, Bool), v: Int, length: Float) -> Float {
0.5
}

fn default(v2: Bool, v3: Int, x: Bool) -> String {
"abc"
}

pub fn main() {
  echo 42
  echo True
  echo {
    {
      let seed_value = [1, 100]
      100
    }
  } + {
    case "res" <> "bc" {
      a -> 7
      "bc" -> fn(v4, v5) { 5 }(3.14, 0.5)
    }
  }
  echo 0.25
}
