pub const seed_value: Bool = True
pub const euler_value: Float = 0.5
pub const tag_value: String = "bc"

pub type Object {
  Cv0(value: String, inner: String)
  Cv1
}

pub type Symbol {
  Some(List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: #(Int, List(Int))) -> String {
case !False {
    b -> "a" <> "x"
    _ -> "b"
  }
}

fn f1(l: List(Int), self_: String, v3: Int) -> Int {
{
    {
      let l = True
      v3 - v3
    }
  } + walk([], {
    let y = False
    v3
  })
}

pub fn main() {
  echo []
}
