pub const golden_value: Int = 4
pub const tag_value: String = "b"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(this_: Float) -> List(Int) {
[100, 4]
}

fn class(v0: String, n: List(Int), class: List(Int)) -> Bool {
case fn(v1) { n }("data") {
    [_] -> True
    [h] -> case False, v0 <> "res" {
      _, "ab" -> False
      v2, "res" -> {
        let h = n
        False
      }
      default, "res" <> rest -> True
      _, _ -> False
    }
    v3 -> True
  }
}

pub fn main() {
  let golden_value = case {
      let tag_value = 0.25
      0.0
    }, <<"x":utf8>> {
    1.0, <<"ab":utf8>> as whole -> {
      let new = 100.0
      let v = []
      golden_value
    }
    _, _ -> golden_value
  }
  echo golden_value
  echo [10]
  echo {
    {
      10.0
    } +. {
      {
        0.1
      } +. {
        3.14
      }
    }
  } -. {
    case {
        100.0
      } == {
        0.1
      } {
      False -> {
        let length = tag_value
        let golden_value = golden_value
        2.0
      }
      item -> {
        2.0
      } -. {
        2.0
      }
    }
  }
}
