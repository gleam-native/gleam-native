pub const seed_value: Float = 3.14
pub const golden_value: Bool = False

pub type V0 {
  Cv1
}

pub type V2 {
  Cv3(Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn export(value: List(Int), v4: String, m: Float) -> Bool {
{
    let x = value
    let item = case "x" {
      b -> "abc" != "res"
      "res" | "data" -> True
    }
    case v4 <> "data" {
      "ab" <> inner if inner == "ab" || inner == "res" -> item
      "a" -> {
        3.14
      } != {
        0.0
      }
      _ -> False
    }
  }
}

fn f1(y: Int, z: Float) -> Bool {
True
}

fn delete(s: Bool, constructor: Int, arguments: Int) -> Int {
{
    fn(v5) { 10 }(7)
  } + {
    {
      let s = {
        let n = 1.5
        [5]
      }
      let m = False
      constructor - arguments
    }
  }
}

pub fn main() {
  echo {
    "b" <> "abc"
  } <> {
    {
      fn(v6, v7) { "bc" }(100.0, "b")
    } <> "abc"
  }
}
