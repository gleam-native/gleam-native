pub const pi_value: Int = 4
pub const seed_value: Float = 2.0

pub type V0 {
  Record(value: String, inner: Int)
  Cv1
  Cv2(value: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn delete(v3: Int, pair: Bool, v4: Int) -> Bool {
pair
}

fn f1(class: List(Int), v5: Float, v6: String) -> List(Int) {
class
}

pub fn main() {
  let acc = pi_value
  echo f1(case Cv1 {
    Record(seed_value, 5) if seed_value == "bc" && seed_value != "abc" -> [100, 42]
    Cv2("ab" <> rest) -> []
    v7 -> fn(v8) { [10, 0] }(100)
  }, case acc |> delete("data" != "", walk([], 100)) {
    False as whole -> seed_value +. seed_value
    _ -> 0.1
    v9 -> {
      100.0
    } -. seed_value
  }, case pi_value {
    a -> "ab"
    default -> fn(v10) { v10 }("bc")
    v11 -> "bc"
  })
  echo case 0 - pi_value {
    5 | 3 -> True
    b -> True
    _ -> True
  }
  echo False
}
