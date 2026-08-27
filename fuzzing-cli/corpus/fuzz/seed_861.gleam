pub const pi_value: String = "constructor"

pub type V0 {
  Cv1
}

pub type V2 {
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(v4: String) -> Float {
{
    case <<0:4, 3:16>> {
      <<"ab":utf8, length:8>> if length <= 4 -> {
        let arguments = []
        let this_ = 10
        1.0
      }
      <<rest:1, _:big-unsigned-1>> as whole if rest <= 9 && rest > 9 -> {
        let pair = 10
        1.0
      }
      _ -> 0.25
    }
  } -. {
    {
      {
        let v = 3.14
        let m = v
        100.0
      }
    } -. {
      {
        let v4 = v4
        2.0
      }
    }
  }
}

pub fn main() {
  echo {
    let arguments = {
      pi_value <> "data"
    } |> export()
    let prototype = 0.5
    {
      "b" <> pi_value
    } <> {
      "abc" <> pi_value
    }
  }
  echo {
    {
      5 - 3
    } + {
      3 - 100
    }
  } % 2
  echo "ab" <> {
    fn(v5) { pi_value <> pi_value }(3)
  }
  echo {
    case fn(v6) { pi_value }(2) {
      "abc" <> rest | "ab" <> rest -> "x"
      item -> fn(v7, v8) { item }(0.5, "bc")
      "data" <> rest -> {
        let rest = pi_value
        pi_value
      }
    }
  } <> {
    fn(v9) { pi_value }(7)
  }
}
