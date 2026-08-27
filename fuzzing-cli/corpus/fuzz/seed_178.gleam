pub const seed_value: Int = 0
pub const pi_value: Int = 0

pub type V0 {
  Cv1(value: List(Int))
  Cv2
  Cv3(value: String, inner: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v4: String) -> Float {
100.0
}

fn f1(v5: Int, new: String, class: Int) -> String {
{
    {
      let pair = class
      "constructor" <> new
    }
  } <> {
    new <> {
      new <> new
    }
  }
}

fn class(this_: String, v6: Bool, v7: Int) -> String {
{
    let v = {
      2.0
    } >. {
      10.0
    }
    let y = [7, 1]
    case this_, y {
      "res" as whole, [1, ..rest] as it if whole != "constructor" && whole == "x" -> whole
      _, [3] -> f1(0, this_, 42)
      "data" <> rest, [_] -> "constructor"
      v8, _ -> f1(v7, v8, v7)
    }
  }
}

pub fn main() {
  let pi_value = case "", "res" {
    "b" <> rest, "abc" -> {
      let constructor = [100]
      "data"
    }
    "bc" <> rest, "bc" -> rest
    _, v9 -> v9
  }
  echo case <<"bc":utf8, "data":utf8>> {
    <<3:8, _:utf8, "constructor":utf8>> -> [0, 1]
    _ -> {
      let y = ""
      {
        let v = 5
        let constructor = []
        [3, 7]
      }
    }
  }
  echo True
  echo seed_value
  echo 10.0
}
