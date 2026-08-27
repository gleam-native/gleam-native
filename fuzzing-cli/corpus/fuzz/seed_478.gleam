pub const euler_value: String = "abc"
pub const limit_value: String = "ab"

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(v2: Float) -> String {
"ab"
}

fn f1(new: #(Float, Int)) -> List(Int) {
case "x", 2 {
    _, v3 -> [3, 2]
    "x" as whole, v4 if whole != "b" -> [4]
    "data" <> rest, _ -> case 10.0 {
      0.25 as whole -> {
        let item = whole
        let z = 0.5
        []
      }
      inner -> {
        let new = rest
        let length = inner
        []
      }
      b -> {
        let new = False
        let delete = new
        [5]
      }
    }
  }
}

pub fn main() {
  let delete = fn(v5) { v5 }(42)
  let constructor = {
    euler_value <> "a"
  } == {
    {
      let s = [100, 4]
      "constructor"
    }
  }
  echo delete
}
