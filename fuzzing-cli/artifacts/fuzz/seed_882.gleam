pub type V0 {
  Cv1(value: List(Int))
}

pub type V2 {
  Cv3(Float)
}

pub type V4 {
  Number(Float, Bool)
  Error
  Cv5
}

fn f0(v6: Float, v7: Int) -> Bool {
False
}

pub fn main() {
  let v = case fn(v8, v9) { Cv5 }(1.0, 3), Number(0.5, True) {
    Number(2.0, True as whole), Error -> "constructor"
    Error, Cv5 -> {
      let constructor = []
      let constructor = 2.0
      "ab"
    }
    v10, v11 -> fn(v12, v13) { "constructor" }("res", 100.0)
  }
  let prototype = {
    3 % 2
  } + {
    7 + 2
  }
  echo case <<"a":utf8, 2:4, 7:8>>, {
      let prototype = 0.1
      let this_ = []
      []
    } {
    <<3:8>>, [0, 9, ..] as whole -> v <> v
    <<"abc":utf8>>, [] -> "abc"
    _, [] -> case Cv1([1]) {
      v -> "bc"
      a -> "ab" <> "bc"
    }
    v14, _ -> v
  }
  echo {
    {
      5 * prototype
    } + prototype
  } - {
    {
      prototype - 5
    } + 5
  }
}
