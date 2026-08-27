pub const golden_value: Bool = True

pub type Object {
  Record
  Cv0(value: Bool)
}

pub type V1 {
  Cv2(Int)
  Error
}

pub type V3 {
  Cv4
  Number(value: Int, inner: Int)
  Cv5(value: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v: String) -> String {
{
    {
      {
        let y = 3
        let x = "b"
        v
      }
    } <> v
  } <> "abc"
}

pub fn main() {
  let golden_value = 100 + {
    fn(v6, v7) { v7 }("b", 10)
  }
  let y = golden_value
  echo True
  echo {
    let y = golden_value
    let arguments = {
      let v = 0 + y
      {
        0.5
      } *. {
        1.5
      }
    }
    y
  }
}
