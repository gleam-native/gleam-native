pub const seed_value: Bool = False

pub type Object {
  Record
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(new: Float, prototype: String) -> List(Int) {
[]
}

fn f1(delete: Float) -> List(Int) {
f0({
    let pair = 7
    let delete = walk([0], pair)
    2.0
  }, "data")
}

pub fn main() {
  echo case 1, "ab" <> "ab" {
    0, "b" <> rest -> fn(v0) { [2] }(False)
    5, _ -> fn(v1, v2) { [10] }(1.0, 0)
    _, _ -> {
      {
        2.0
      } +. {
        0.25
      }
    } |> f0("data" <> "b")
  }
  echo case "ab" <> "b" {
    "bc" -> case Record {
      a -> 2
      item -> 100
    }
    "data" <> rest -> 100
    _ -> 0
  }
}
