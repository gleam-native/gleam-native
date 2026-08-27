pub const limit_value: String = ""
pub const tag_value: String = "res"

pub type Number {
  Record
  Cv0(value: List(Int), inner: String)
}

pub type V1 {
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(class: Int, constructor: Int, v3: String) -> List(Int) {
[]
}

fn f1(prototype: Bool) -> Int {
1
}

fn yield(v4: Int) -> Bool {
{
    let l = "a" <> "b"
    False
  }
}

pub fn main() {
  let delete = 42
  let delete = True
  echo 4 |> export(f1(False), "a")
  echo limit_value
  echo {
    let z = 0.0
    fn(v5) { [7, 4] }(0.1)
  }
  echo 3
}
