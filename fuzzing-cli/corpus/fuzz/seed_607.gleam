pub const limit_value: String = "abc"

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Record
}

fn default(x: Float, v2: Int) -> List(Int) {
[4]
}

fn f1(v3: Int) -> List(Int) {
case Record {
    Cv1([b, ..rest], 9) -> case "ab" {
      "bc" <> _ -> []
      "a" <> constructor | "" <> constructor -> default(0.25, b)
      constructor -> []
    }
    _ -> {
      0.0
    } |> default(4)
  }
}

pub fn main() {
  echo "a" <> {
    {
      let prototype = {
        let acc = limit_value
        True
      }
      limit_value <> limit_value
    }
  }
  echo "constructor"
  echo case Record {
    constructor -> {
      1.5
    } *. {
      fn(v4) { 0.5 }(0.1)
    }
    item -> {
      100.0
    } *. {
      {
        let limit_value = []
        0.1
      }
    }
    _ -> 1.0
  }
  echo case {
      let class = False
      Cv1([], 4)
    }, {
      let length = False
      limit_value
    } {
    _, "constructor" -> "x"
    Cv1([limit_value, ..rest] as whole, v5), _ -> "abc"
    limit_value, "x" <> _ as whole -> case <<"ab":utf8, "abc":utf8>>, 0 {
      <<1:4>>, _ -> "constructor"
      _, 6 -> whole
      v6, v7 -> "data" <> whole
    }
    _, v8 -> v8
  }
}
