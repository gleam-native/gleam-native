pub const euler_value: Int = 10

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Float, z: Bool, y: Bool) -> Bool {
fn(v0, v1) { spin(v1, v1) != spin(v1, v1) }(3.14, 1)
}

fn f1(l: String, v2: Float, arguments: Int) -> Bool {
f0({
    v2 +. {
      0.25
    }
  } +. {
    1.5
  }, True, {
    let v2 = arguments
    let l = True
    5 > v2
  })
}

fn f2(m: Float, prototype: Bool, v: Bool) -> Bool {
case prototype || v, 1 + 7 {
    True, 6 -> False
    False, 7 -> f1("b" <> "bc", 1.0, 10)
    _, _ -> prototype
  }
}

pub fn main() {
  let prototype = case "a" {
    a -> {
      let pair = 0
      True
    }
    "x" <> item | "a" <> item -> fn(v3) { True }("constructor")
    "constructor" <> rest | "abc" <> rest -> True
  }
  let s = case spin(0, 100) {
    constructor -> prototype
    prototype -> True
  }
  echo case euler_value - euler_value {
    7 -> "bc"
    5 -> "res"
    v4 -> case "ab" <> "" {
      "b" -> "x" <> "data"
      "abc" <> constructor if constructor != "a" -> fn(v5) { "bc" }(7)
      "res" <> rest -> "abc"
      _ -> "ab" <> ""
    }
  }
  echo case {
      let euler_value = prototype
      "ab"
    } {
    "ab" <> _ -> []
    _ -> [42, 42]
    "ab" <> rest -> []
  }
  echo True && s
  echo case "a", "b" {
    "b", "b" <> _ -> case euler_value {
      7 -> "ab"
      inner -> "bc" <> "abc"
    }
    "bc" <> _, "res" -> fn(v6, v7) { "abc" }(10.0, 3)
    "b" <> rest as whole, "ab" <> tail -> case [10], {
        let l = []
        [3, 10]
      } {
      [_], [4] -> "a"
      [], [7, ..rest] -> "bc"
      _, _ -> {
        let length = 1.5
        let pair = [0]
        rest
      }
    }
    _, _ -> "b"
  }
}
