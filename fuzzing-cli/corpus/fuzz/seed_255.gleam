pub const seed_value: Float = 0.1
pub const euler_value: Int = 5

pub type V0 {
  Error(value: String, inner: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(item: Int, v1: String) -> Bool {
case fn(v2) { #(100, "abc") }(False) {
    b -> True
    #(6, _) -> True
  }
}

fn export(v3: Int, arguments: V0, v4: String) -> String {
"res"
}

fn f2(constructor: #(Int, List(Int)), n: Int, v5: Int) -> List(Int) {
case v5 * n, [100] {
    0, [_, 6, ..] -> [7, 3]
    _, [_] -> [1]
    _, _ -> []
  }
}

pub fn main() {
  let euler_value = {
    "" <> "constructor"
  } <> ""
  let self_ = 100 * 2
  echo {
    0.5
  } /. {
    2.0
  }
  echo #(1, [1, 2]) |> f2(self_, 4)
  echo False
  echo walk(fn(v6) { fn(v7, v8) { [7] }(False, 100.0) }(100.0), 0 * 4)
}
