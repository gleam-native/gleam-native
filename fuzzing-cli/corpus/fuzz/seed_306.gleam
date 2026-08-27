pub type Record {
  Cv0(value: String, inner: Float)
  Cv1(value: List(Int), inner: Bool)
}

fn f0(v2: Int, v3: Bool, pair: Bool) -> Bool {
{
    case "" <> "ab", 10 {
      v4, 3 -> {
        10.0
      } +. {
        1.5
      }
      "x" <> _ as whole, 6 as it -> {
        1.0
      } -. {
        0.1
      }
      _, v5 -> fn(v6) { 0.1 }(100.0)
    }
  } >=. {
    {
      let v = []
      let acc = "data"
      fn(v7, v8) { 10.0 }(10, True)
    }
  }
}

fn f1(v9: #(Float, String)) -> String {
"x"
}

fn f2(arguments: List(Int), constructor: #(Bool, String)) -> Float {
case fn(v10) { Cv0("constructor", 3.14) }(42), 4 {
    Cv1([7], True), 6 -> case fn(v11) { arguments }("x") {
      [0, ..rest] -> {
        0.25
      } +. {
        1.0
      }
      [_, ..rest] as whole -> 0.25
      v12 -> 1.0
    }
    Cv0("data" <> rest, 0.5), length if length <= 8 && length > 4 -> case True {
      _ -> 1.5
      False | True -> 2.0
      True | False -> 1.5
    }
    Cv1([], False as whole), _ -> {
      fn(v13) { 0.1 }(True)
    } +. {
      3.14
    }
    v14, _ -> {
      {
        10.0
      } +. {
        0.25
      }
    } +. {
      {
        0.0
      } +. {
        1.5
      }
    }
  }
}

pub fn main() {
  let z = {
    {
      let new = False
      let m = "x"
      0.0
    }
  } == {
    0.1
  }
  echo "b" <> "b"
  echo case 1 + 100 {
    a -> case {
        let pair = a
        1.5
      } {
      2.0 -> a
      b -> a
    }
    a -> a
  }
  echo 2.0
  echo {
    {
      "ab" <> "constructor"
    } <> "data"
  } <> {
    {
      fn(v15) { #(0.0, "data") }(5)
    } |> f1()
  }
}
