pub const limit_value: String = "res"
pub const golden_value: Bool = False
pub const euler_value: Bool = False

pub type Map {
  Cv0(value: String, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(v1: Int, v2: Map, v3: Int) -> Int {
case v3 {
    7 -> case fn(v4) { v2 }(3.14) {
      Cv0("a", v5) -> {
        let s = [4, 0]
        v3
      }
      Cv0("x", 1.0) | Cv0(_, _) -> v3 + v1
      _ -> 0
    }
    inner -> v3
  }
}

fn f1(v6: Int) -> Float {
10.0
}

fn f2(value: List(Int), v7: Int, class: List(Int)) -> List(Int) {
[]
}

pub fn main() {
  let golden_value = case 2, fn(v8) { "ab" }(2.0) {
    7, "res" -> {
      1.5
    } *. {
      0.0
    }
    1, "bc" <> _ -> f1(4)
    v9, "bc" <> rest -> 100.0
    _, _ -> {
      let default = limit_value
      let arguments = "res"
      3.14
    }
  }
  echo 1.5
  echo []
  echo case [42] |> f2(3, [0]) {
    [constructor, ..rest] if constructor <= 7 -> euler_value
    [_, ..rest] -> {
      True && euler_value
    } && True
    v10 -> case limit_value <> limit_value {
      b -> 5 < 3
      "b" <> constructor -> {
        let constructor = golden_value
        euler_value
      }
      "a" | "" <> _ -> "ab" == limit_value
    }
  }
}
