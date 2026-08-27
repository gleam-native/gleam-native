pub const seed_value: String = "x"

pub type Promise {
  Cv0(value: String, inner: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(v1: Int, v2: String) -> Float {
case v2 {
    "a" <> a if a != "b" -> 2.0
    b | "x" <> b -> case v1 {
      8 -> {
        0.1
      } +. {
        3.14
      }
      a -> {
        let new = v1
        let x = 1
        1.0
      }
    }
  }
}

fn class(class: Float, v: #(String, String)) -> Bool {
case <<42:8>>, 100 % 6 {
    <<"data":utf8, "ab":utf8>>, item if item > 3 -> {
      {
        let delete = 1
        let class = [10]
        "x"
      }
    } == "res"
    _, 7 -> 10 != {
      {
        let class = []
        let delete = class
        1
      }
    }
    _, v3 -> fn(v4) { fn(v5, v6) { True }(100.0, True) }(42)
  }
}

pub fn main() {
  echo case {
      let m = False
      m
    }, {
      let y = "constructor"
      let l = []
      l
    } {
    False as whole, [_, ..rest] as it -> case seed_value {
      "ab" <> rest | "ab" <> rest -> []
      inner | "a" <> inner -> {
        let m = inner
        []
      }
      "res" -> [5, 42]
    }
    True, [seed_value] if seed_value <= 8 -> case 42 {
      _ -> {
        let default = True
        let n = [1, 1]
        [7, 10]
      }
      5 -> {
        let self_ = [1, 7]
        let new = self_
        new
      }
      7 | 1 -> fn(v7, v8) { [4] }(3.14, False)
    }
    False, [1, 0, ..] -> case seed_value {
      _ -> [1]
      "bc" <> inner -> {
        let seed_value = inner
        [5]
      }
    }
    _, v9 -> [3]
  }
}
