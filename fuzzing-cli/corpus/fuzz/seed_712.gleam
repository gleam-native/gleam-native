pub const golden_value: String = "b"
pub const limit_value: Int = 4

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type V2 {
  Some
  Ok
}

pub type V3 {
  Cv4(value: Float, inner: Bool)
  Cv5
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn yield(v6: Int, s: #(Float, Bool)) -> String {
"x"
}

fn delete(this_: Bool, v7: Float) -> Int {
3
}

fn f2(n: Int) -> Bool {
case <<"res":utf8, "":utf8>> {
    <<100:1>> -> {
      fn(v8) { True }(100)
    } && False
    _ -> False || True
  }
}

pub fn main() {
  echo "ab"
  echo case fn(v9, v10) { Ok }("constructor", 2) {
    inner -> {
      1.5
    } +. {
      fn(v11) { 1.5 }("a")
    }
    item -> {
      {
        100.0
      } *. {
        0.5
      }
    } -. {
      0.25
    }
  }
  echo []
}
