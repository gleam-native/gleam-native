pub const euler_value: Int = 100

pub type Symbol {
  Cv0(value: String, inner: String)
  Number
  Cv1
}

fn f0(v2: Int, prototype: Int, v3: Int) -> String {
case {
      let n = prototype
      "abc"
    } {
    "x" | "ab" -> {
      {
        let constructor = "abc"
        constructor
      }
    } <> {
      "data" <> "abc"
    }
    constructor -> case {
        let this_ = []
        let m = 2.0
        this_
      } {
      [9] -> "bc"
      [a, _, ..] if a % 2 == 0 -> "a" <> "a"
      [_, 6, ..] -> constructor
      v4 -> "bc"
    }
    "a" -> "a" <> "res"
  }
}

fn arguments(m: #(Bool, List(Int))) -> Int {
fn(v5, v6) { {
    {
      let v5 = [42, 100]
      7
    }
  } - {
    2 + 4
  } }(0.25, True)
}

fn f2(v7: Int, v8: Symbol) -> Bool {
!{
    case [5, 1], "data" {
      [], v9 if v9 != "res" -> False || False
      [_, a, ..], "ab" as whole -> {
        let rest = []
        let y = 10.0
        True
      }
      v10, v11 -> 10 <= v7
    }
  }
}

pub fn main() {
  let self_ = case [3], Cv0("bc", "ab") {
    [9], Cv0("bc", "constructor") -> {
      let this_ = "ab"
      let arguments = "constructor"
      0
    }
    [], Cv1 -> euler_value * euler_value
    [a, 4, ..], Number -> arguments(#(False, [42]))
    _, v12 -> euler_value - euler_value
  }
  echo {
    case f0(euler_value, 100, self_) {
      v13 -> {
        10.0
      } -. {
        100.0
      }
      "ab" | "res" -> {
        0.5
      } +. {
        1.0
      }
      "data" <> _ -> 1.0
    }
  } +. {
    0.5
  }
  echo "" != {
    "abc" <> "ab"
  }
  echo 0.0
  echo {
    {
      2.0
    } *. {
      10.0
    }
  } *. {
    10.0
  }
}
