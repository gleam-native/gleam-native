pub const tag_value: Int = 10

pub type Record {
  Cv0(value: String, inner: List(Int))
  Cv1(Float, Bool)
}

fn f0(new: List(Int)) -> List(Int) {
fn(v2) { new }(0)
}

pub fn main() {
  let tag_value = 2.0
  let new = True
  echo case f0([0, 3]) {
    [6, ..rest] -> case [1, 5] {
      [_] as whole -> {
        let tag_value = new
        let new = []
        new
      }
      [5, ..rest] -> rest
      [1] as whole -> fn(v3, v4) { [4] }(False, "x")
      _ -> rest
    }
    [] -> case 0, {
        let new = new
        let item = tag_value
        "bc"
      } {
      1 as whole, "a" -> []
      _, _ -> [10]
    }
    _ -> case "res", False {
      "" <> rest, False if rest == "" -> [2, 7]
      "data" <> rest, True -> f0([])
      v5, v6 -> [4]
    }
  }
  echo case #(1, False) {
    inner -> fn(v7) { "x" }(1.0)
    item -> "data"
  }
}
