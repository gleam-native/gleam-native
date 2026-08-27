pub const euler_value: String = "x"
pub const golden_value: String = "a"
pub const limit_value: Bool = False

pub type Number {
  Cv0(value: String, inner: Float)
  None
}

pub type Record {
  Cv1(Float, value: String)
  Ok
}

pub type V2 {
  Cv3(Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(s: Int, y: Float, x: Int) -> Int {
x
}

pub fn main() {
  let value = case {
      let limit_value = 0.25
      let m = 10.0
      #([10, 4], "constructor")
    } {
    #([_], value) if value == "abc" -> {
      1.5
    } <. {
      3.14
    }
    #([5, ..rest], "abc") -> {
      0.25
    } != {
      0.0
    }
    _ -> {
      0.1
    } >=. {
      10.0
    }
  }
  let n = 10 + 2
  echo case euler_value, n |> f0({
      let length = False
      let s = 100.0
      s
    }, {
      let acc = n
      let n = 3.14
      42
    }) {
    _, 9 -> case euler_value <> "abc" {
      "b" -> limit_value
      "abc" <> rest | "x" <> rest -> fn(v4) { value }(False)
      constructor | "x" <> constructor -> n < n
    }
    "bc", _ -> value
    "x" <> rest, 7 -> False
    _, v5 -> {
      let n = n
      {
        let limit_value = 0.5
        value
      }
    }
  }
  echo {
    case Ok, "b" {
      pair, "a" <> rest if rest != "a" || rest == "a" -> euler_value
      Ok as whole, "a" -> "bc" <> golden_value
      Ok, v6 -> euler_value <> golden_value
      _, _ -> "b"
    }
  } <> "bc"
  echo euler_value <> {
    case <<"a":utf8>>, "x" {
      <<"data":utf8>>, _ -> "abc"
      <<3:16>>, "data" -> golden_value <> golden_value
      <<_:utf8>>, "bc" -> golden_value
      v7, _ -> fn(v8, v9) { golden_value }(7, False)
    }
  }
}
