pub const euler_value: Bool = False

pub type V0 {
  Cv1(value: List(Int))
}

pub type V2 {
  None
  Number(value: Int, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(v3: String, x: String) -> List(Int) {
[4]
}

fn f1(v4: Int, item: V2) -> Int {
v4
}

fn f2(acc: Float, delete: String, m: Float) -> String {
case fn(v5) { 10 }(0.25), [] |> walk(walk([], 5)) {
    4, _ -> "data"
    5, 1 as whole -> {
      let v = 1.5
      let item = delete
      {
        let v = [7]
        item
      }
    }
    v6, _ -> delete
  }
}

pub fn main() {
  let n = 3.14
  let self_ = "a" <> "data"
  echo 0.25
  echo case "ab", 7 + 5 {
    "data", 0 as whole if whole <= 5 -> [2]
    "data" <> rest, 8 -> []
    v7, v8 -> {
      v7 <> "b"
    } |> arguments(v7)
  }
  echo 100
}
