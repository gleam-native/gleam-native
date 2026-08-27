pub const golden_value: String = "data"
pub const limit_value: Float = 3.14

fn f0(m: String, v0: Bool, v1: Int) -> Bool {
fn(v2, v3) { case v1 + v3 {
    item -> False
    b -> v1 == 42
  } }(5, 5)
}

fn f1(l: Float, v4: String) -> String {
{
    case <<"x":utf8>> {
      <<_:utf8, _:utf8>> -> v4
      <<"data":utf8, "bc":utf8>> -> "b" <> v4
      _ -> "data" <> "constructor"
    }
  } <> {
    {
      let l = True
      let prototype = v4
      "constructor" <> ""
    }
  }
}

pub fn main() {
  let v = 4
  let s = {
    let default = [7, 7]
    let self_ = {
      let default = default
      let default = False
      True
    }
    default
  }
  echo case {
      let default = s
      let pair = limit_value
      golden_value
    } {
    "" <> rest -> limit_value +. {
      {
        let golden_value = rest
        limit_value
      }
    }
    "x" <> rest -> fn(v5) { 0.25 }(0)
    b -> limit_value
  }
  echo v >= {
    v % 6
  }
}
