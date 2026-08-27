pub const limit_value: Int = 2
pub const pi_value: String = "b"
pub const golden_value: Int = 42

pub type V0 {
  Cv1(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn class(v2: Int) -> Float {
0.5
}

fn f1(v3: #(Bool, Float), v4: String) -> Bool {
{
    {
      fn(v5, v6) { 2.0 }(True, 5)
    } == {
      {
        let z = v4
        let v3 = 3.14
        v3
      }
    }
  } && {
    {
      {
        let v4 = False
        let length = 3
        length
      }
    } <= {
      {
        let delete = [1]
        7
      }
    }
  }
}

fn f2(v7: List(Int)) -> Float {
10.0
}

pub fn main() {
  let arguments = case <<"x":utf8>> {
    <<2:8>> -> True
    <<_:little-unsigned-8, 5:16>> -> True
    _ -> {
      let v = []
      True
    }
  }
  let golden_value = #(False, 3.14) |> f1("b")
  echo case pi_value {
    b -> case Cv1([]) {
      Cv1([constructor, ..rest]) -> b
      a -> "data"
      Cv1([_]) -> pi_value
    }
    item -> "bc" <> {
      item <> pi_value
    }
  }
  echo limit_value <= {
    case {
        let arguments = pi_value
        arguments
      } {
      "x" <> rest -> {
        let z = arguments
        limit_value
      }
      constructor -> limit_value
    }
  }
}
