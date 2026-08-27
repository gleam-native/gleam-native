fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(value: String) -> Bool {
case <<"bc":utf8, "x":utf8>>, 1 + 3 {
    <<_:8>>, 5 -> {
      {
        100.0
      } >=. {
        1.0
      }
    } || {
      "bc" == "abc"
    }
    <<_:utf8>>, 3 -> True
    _, 0 -> True
    _, _ -> {
      {
        1.5
      } *. {
        0.25
      }
    } <. {
      {
        10.0
      } -. {
        0.0
      }
    }
  }
}

fn f1(v0: Float) -> Int {
case [0, 3] {
    [6] -> 4
    [] -> 100
    [] as whole -> 42
    v1 -> 2
  }
}

fn f2(constructor: Bool, value: Bool) -> List(Int) {
case {
      3.14
    } -. {
      100.0
    } {
    2.0 as whole -> []
    2.0 -> []
    _ -> [3]
  }
}

pub fn main() {
  echo case "" <> "abc" {
    constructor -> case {
        let prototype = [1, 3]
        constructor
      } {
      "constructor" <> rest | "constructor" <> rest -> fn(v2) { "constructor" }(True)
      constructor -> constructor <> "b"
    }
    _ -> {
      {
        let this_ = 1.0
        "a"
      }
    } <> {
      "res" <> "res"
    }
  }
}
