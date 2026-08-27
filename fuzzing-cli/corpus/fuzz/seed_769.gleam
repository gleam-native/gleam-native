pub const pi_value: String = "bc"

pub type Map {
  Cv0(value: String, inner: Int)
  Cv1(String, Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Map, rest: List(Int), v2: Map) -> List(Int) {
rest
}

pub fn main() {
  let pi_value = pi_value
  let x = {
    {
      let pi_value = pi_value
      let arguments = 2.0
      False
    }
  } || {
    !True
  }
  echo case 7 {
    pi_value -> case Cv1("res", False) {
      Cv0("x" <> rest, 5) -> {
        let n = [4, 1]
        ""
      }
      Cv1(z, _) -> z
      v3 -> "" <> "constructor"
    }
    constructor -> case Cv1("abc", False), {
        let constructor = pi_value
        0
      } {
      x, 1 -> pi_value <> pi_value
      Cv0("res", 8), constructor -> pi_value
      v4, _ -> fn(v5) { "" }(42)
    }
    constructor -> {
      {
        let delete = constructor
        "data"
      }
    } <> {
      {
        let item = 0.5
        let rest = pi_value
        rest
      }
    }
  }
  echo {
    {
      fn(v6, v7) { "bc" }(10.0, 3.14)
    } <> pi_value
  } <> {
    {
      let n = Cv0("bc", 5) |> f0(f0(Cv0("bc", 1), [0, 100], Cv0("b", 0)), {
        let class = False
        let pi_value = 100
        Cv1("a", False)
      })
      "x"
    }
  }
  echo 1.0
  echo case fn(v8) { v8 }(0.5), #("bc", 3) {
    10.0, #("data" <> _ as whole, 2) as it if whole != "a" -> case {
        let l = []
        let item = "bc"
        False
      }, Cv0("data", 5) |> f0([], {
        let constructor = []
        Cv1("ab", True)
      }) {
      v9, [4] if v9 -> True
      True, [] -> 0 == 4
      False, [] -> {
        let new = True
        new
      }
      _, v10 -> {
        let x = False
        let x = v10
        True
      }
    }
    1.5, #("b", rest) if rest <= 1 -> "ab" != pi_value
    class, #("ab", self_) -> case #(True, 1), Cv0("constructor", 5) {
      #(_, 9), _ -> False
      #(False, _), Cv1(x, _) as whole -> False
      v11, v12 -> class >. class
    }
    _, _ -> True || {
      {
        10.0
      } != {
        100.0
      }
    }
  }
}
