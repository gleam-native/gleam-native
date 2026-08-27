pub const pi_value: Int = 2

pub type Object {
  Cv0(value: String, inner: Float)
  Some(value: String)
}

pub type Symbol {
  Cv1(value: List(Int))
}

fn f0(class: Bool, z: #(String, Bool), default: Bool) -> String {
"data" <> {
    case <<4:8>> {
      <<rest:16, "x":utf8, _:big-signed-1>> if rest > 4 && rest > 9 -> "x" <> ""
      <<_:utf8, 5:8>> -> "bc"
      _ -> {
        let prototype = 2
        ""
      }
    }
  }
}

pub fn main() {
  echo "a"
  echo {
    let constructor = False
    "bc"
  }
  echo {
    {
      fn(v2, v3) { 0.5 }(True, False)
    } +. {
      {
        100.0
      } +. {
        0.0
      }
    }
  } /. {
    1.0
  }
}
