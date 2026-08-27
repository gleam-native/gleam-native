pub const limit_value: Int = 3
pub const tag_value: Float = 100.0
pub const seed_value: Float = 0.0

pub type V0 {
  Cv1
  Cv2
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(prototype: Int, this_: Int) -> String {
"constructor" <> {
    {
      {
        let delete = []
        let y = "data"
        "res"
      }
    } <> "bc"
  }
}

fn f1(rest: Bool, v4: Bool) -> Float {
0.25
}

fn f2(v5: Int, v6: Float, v7: Int) -> Int {
{
    let v5 = v5
    [10, 4] |> walk(fn(v8, v9) { 3 }(0.1, 0))
  }
}

pub fn main() {
  let self_ = {
    let tag_value = seed_value
    True |> f1(seed_value == tag_value)
  }
  echo case limit_value * limit_value, <<"b":utf8>> {
    _, <<"constructor":utf8, "":utf8>> -> f0(limit_value, limit_value + 7)
    5, <<42:8>> -> "res"
    7, _ -> {
      let limit_value = {
        3.14
      } +. self_
      let limit_value = {
        let this_ = "b"
        0.0
      }
      f0(1, 7)
    }
    v10, v11 -> fn(v12) { "x" <> "ab" }(True)
  }
  echo 10
}
