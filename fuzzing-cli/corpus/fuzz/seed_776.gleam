pub type V0 {
  Cv1(value: List(Int))
  Cv2(String, value: String)
}

pub type V3 {
  Cv4
  Cv5
  Cv6(String, Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v7: Bool, v8: String) -> String {
{
    let z = case Cv1([7, 7]) {
      Cv2("b" <> rest, "x" as whole) if rest == "constructor" && rest != "" -> v8 <> rest
      a -> "" <> "a"
    }
    case <<"constructor":utf8, 7:8, 2:1>> {
      <<_:utf8, _:little-unsigned-4, 0:4>> -> v8 <> "bc"
      _ -> "res" <> "x"
    }
  }
}

pub fn main() {
  let constructor = False
  echo {
    {
      let s = {
        0.0
      } *. {
        2.0
      }
      fn(v9) { 3 }(False)
    }
  } % 3
  echo 0.0
}
