pub const euler_value: Float = 1.0
pub const limit_value: Int = 2

pub type Record {
  Cv0(value: String, inner: List(Int))
  Cv1(value: List(Int))
}

fn f0(m: Int) -> Bool {
case "ab" <> "a" {
    "constructor" <> inner if inner != "b" && inner != "ab" -> case fn(v2) { v2 }(1) {
      9 -> {
        let inner = inner
        True
      }
      5 -> True
      v3 -> True
    }
    "constructor" <> rest -> False
    v4 -> {
      1.5
    } >. {
      fn(v5) { 0.0 }(2)
    }
  }
}

fn f1(v6: #(String, String)) -> Int {
case "a" {
    "constructor" -> {
      1 - 1
    } + {
      1 + 7
    }
    _ -> case [] {
      [3] as whole -> 42
      [b] -> 1 + b
      [] -> 7 + 1
      _ -> 7
    }
  }
}

pub fn main() {
  let rest = case limit_value, limit_value + limit_value {
    limit_value, 5 if limit_value <= 1 -> True
    _, 6 -> False
    _, v7 -> fn(v8, v9) { v9 }(True, True)
  }
  echo case <<10:4>> {
    <<_:4>> -> case 1 {
      constructor -> {
        let rest = [4, 1]
        let prototype = euler_value
        "data"
      }
      0 -> fn(v10) { "a" }(100.0)
    }
    <<_:utf8>> -> case "" {
      "abc" -> "x"
      limit_value -> limit_value
    }
    v11 -> case "x" {
      _ -> "data"
      b -> b
    }
  }
}
