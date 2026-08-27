pub const seed_value: Float = 0.25

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: Int, default: List(Int)) -> List(Int) {
case True, Cv1([]) {
    _, _ -> case "res" <> "ab", Cv1([10, 2]) {
      "abc", Cv1([8, ..rest]) as whole -> fn(v3) { [1] }(0.25)
      "b" <> _, v4 -> {
        let y = "a"
        default
      }
      v5, _ -> {
        let value = False
        default
      }
    }
    v6, Cv1([_, ..rest]) -> case fn(v7, v8) { default }("res", True) {
      [3] -> {
        let default = v2
        rest
      }
      [4, _, ..] -> [1, 1]
      _ -> [0]
    }
  }
}

fn f1(acc: Int, item: Int) -> List(Int) {
[2]
}

pub fn main() {
  let m = True
  let rest = 3.14
  echo 5
}
