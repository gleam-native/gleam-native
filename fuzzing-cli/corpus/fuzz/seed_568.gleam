pub const euler_value: Bool = True
pub const seed_value: Float = 0.5
pub const tag_value: Bool = True

pub type Number {
  Cv0(value: String, inner: String)
  Cv1(value: String, inner: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn delete(acc: Int, x: Float, delete: List(Int)) -> String {
{
    case 5, #("b", "abc") {
      _, #(_, "bc") -> "data" <> "bc"
      1, #(_, delete) -> delete
      default, #("data", "" <> _) as whole -> "constructor"
      _, v2 -> "constructor" <> "res"
    }
  } <> {
    case "res" <> "res" {
      "a" <> rest -> rest <> rest
      "bc" as whole -> fn(v3, v4) { "res" }(True, 1)
      b -> {
        let n = 10
        b
      }
    }
  }
}

pub fn main() {
  echo []
  echo 3.14
  echo "res" == "ab"
}
