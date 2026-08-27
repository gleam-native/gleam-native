pub const limit_value: Bool = True
pub const tag_value: Bool = False
pub const golden_value: String = "constructor"

pub type V0 {
  Cv1
  Cv2
  Cv3(value: Int, inner: Int)
}

pub type V4 {
  Number
  None(Int)
}

pub type Number {
  Ok(List(Int))
  Cv5(Float, value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(arguments: String, delete: Int) -> Bool {
{
    case {
        let z = "a"
        let m = [0]
        5
      } {
      0 | 9 -> !False
      item -> True
    }
  } || {
    {
      let delete = 7 * delete
      let arguments = True
      arguments
    }
  }
}

fn f1(rest: Float, v6: Bool, v7: List(Int)) -> Int {
1 - {
    fn(v8) { walk(v7, 42) }(0.25)
  }
}

fn f2(length: String, v9: String, new: Int) -> Bool {
{
    case <<5:1, 7:16, "ab":utf8>> {
      <<_:utf8>> -> "res" <> "abc"
      <<10:16>> -> v9
      _ -> {
        let length = True
        let length = [10]
        v9
      }
    }
  } |> export(10)
}

pub fn main() {
  let prototype = case 7 {
    _ | 0 -> [42]
    5 | 7 -> [10, 2]
  }
  echo prototype
}
