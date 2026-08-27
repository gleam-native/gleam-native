pub const limit_value: String = "bc"
pub const seed_value: Bool = False

pub type V0 {
  Ok(value: String, inner: Bool)
}

pub type V1 {
  Cv2
}

pub type V3 {
  Cv4
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(value: V0, v5: Int, v6: Int) -> Int {
walk([], v5 - {
    4 - 5
  })
}

pub fn main() {
  echo case "b", <<"a":utf8>> {
    "ab" <> rest, <<"x":utf8, 10:1>> -> case 100 {
      _ -> [7]
      _ | 5 -> fn(v7, v8) { [3] }("a", "abc")
      _ -> [2, 4]
    }
    _, _ -> {
      let class = {
        let value = [2]
        2.0
      }
      let m = "data"
      {
        let value = 1.5
        [10]
      }
    }
  }
  echo 3
  echo case {
      let seed_value = seed_value
      let rest = [1, 4]
      Cv4
    } {
    Cv4 | Cv4 -> case [0, 1], {
        let seed_value = limit_value
        100.0
      } {
      [h, _, ..], v9 if h <= 9 && h > 5 -> [10, 7]
      [_, _, ..], 0.25 -> [0]
      [x], 1.5 -> [10]
      _, _ -> [0, 2]
    }
    acc -> fn(v10) { [] }(0.5)
  }
  echo case 100, limit_value {
    1, "res" -> case 2, [10, 5] {
      2 as whole, [3] -> False
      3, [_] -> fn(v11, v12) { True }("data", 100.0)
      v13, _ -> seed_value
    }
    _, "res" <> rest -> False
    _, _ -> True
  }
}
