pub const seed_value: Int = 2

pub type V0 {
  Cv1
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn delete(l: String, v3: Int) -> Float {
2.0
}

fn f1(x: #(Int, Bool)) -> Int {
4
}

fn f2(v4: Bool, v5: Bool, length: Int) -> List(Int) {
fn(v6) { case "constructor" {
    _ -> [100]
    "x" -> [5]
    "" <> inner -> {
      let inner = 0.1
      let v = v4
      []
    }
  } }(4)
}

pub fn main() {
  echo case "bc" <> "b" {
    "constructor" | "res" -> case #(1.0, "data") {
      #(100.0, "x" <> rest) if rest != "b" -> True || False
      #(3.14, "x") -> True
      #(100.0, "bc" <> rest as whole) -> {
        let rest = 0.1
        let whole = [42, 5]
        False
      }
      v7 -> 4 == 42
    }
    "ab" <> rest | "data" <> rest -> {
      0.5
    } <. {
      0.1
    }
    _ -> {
      let class = seed_value
      fn(v8, v9) { True }(0.5, 0.1)
    }
  }
  echo {
    {
      let value = "data"
      let seed_value = 7 + seed_value
      7
    }
  } + {
    {
      seed_value * seed_value
    } + spin(4, seed_value)
  }
  echo "abc" <> {
    case {
        let delete = 42
        "a"
      }, <<4:4>> {
      "bc" <> _, <<"constructor":utf8>> -> fn(v10, v11) { "ab" }(0.1, 100)
      "ab" <> rest, <<"constructor":utf8>> -> "a"
      "data", _ -> {
        let seed_value = True
        "bc"
      }
      _, v12 -> "b"
    }
  }
}
