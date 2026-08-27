pub const golden_value: Float = 2.0
pub const tag_value: String = "res"
pub const pi_value: Int = 42

pub type V0 {
  None(value: String, inner: Bool)
  Cv1
}

fn f0(v2: Float, v3: Int, n: String) -> Float {
{
    case v3 {
      item -> v2
      4 -> 2.0
      a -> v2 *. {
        0.1
      }
    }
  } +. {
    3.14
  }
}

fn f1(v4: Bool, v5: Int, z: Int) -> Bool {
v4
}

fn f2(v6: List(Int), class: Int, v7: Bool) -> Float {
{
    case f0(1.0, 2, ""), None("x", True) {
      0.0, v8 -> 0.5
      _, v9 -> 100.0
      1.5 as whole, None("abc" <> _, False) as it -> {
        0.5
      } |> f0(class + class, "bc")
    }
  } |> f0(class, "b")
}

pub fn main() {
  let l = tag_value
  let this_ = {
    let l = True
    let class = fn(v10, v11) { l }("constructor", 42)
    4
  }
  echo []
  echo case 4 + this_ {
    l -> case "res", [] {
      tag_value, [] -> pi_value
      "data", [1, 8, ..] -> fn(v12) { 5 }(1.5)
      _, [5, ..rest] -> fn(v13, v14) { l }("constructor", "")
      _, _ -> l - pi_value
    }
    item -> case 100, fn(v15, v16) { Cv1 }(3.14, 10.0) {
      7, None("a", True) -> 5
      2, Cv1 -> this_ + 2
      v17, v18 -> fn(v19) { 1 }("x")
    }
    item -> item
  }
}
