fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: String, v0: Int, v1: Int) -> Bool {
case fn(v2, v3) { v1 }(1.5, True) {
    0 | 4 -> False
    inner -> fn(v4) { False }(2)
    b -> {
      let new = fn(v5) { [3] }(3)
      let constructor = {
        let new = constructor
        let new = 10.0
        new
      }
      True
    }
  }
}

fn f1(new: Bool) -> Int {
5
}

pub fn main() {
  echo spin({
    let pair = fn(v6, v7) { "data" }(2, "abc")
    True |> f1()
  }, 5)
  echo "a"
  echo case [1, 42], 2 {
    [b, _, ..], _ -> "b" <> "bc"
    [_], v8 -> case [2, 10] {
      [0] -> "b" <> "ab"
      [] -> "abc"
      _ -> "res" <> "a"
    }
    [a, ..rest], 6 -> case "x" {
      inner -> fn(v9) { inner }(True)
      "" <> a -> "constructor"
    }
    v10, v11 -> case v10 {
      [v10, 8, ..] -> fn(v12) { "a" }(1.0)
      [_] -> "" <> "a"
      v13 -> "ab" <> "a"
    }
  }
  echo case #(3, "abc"), fn(v14, v15) { #([], 0.25) }(True, True) {
    #(6, "constructor"), #([6, ..rest], 2.0) -> {
      {
        let x = 1
        1.0
      }
    } -. {
      {
        0.0
      } +. {
        0.25
      }
    }
    #(delete, "res"), #([4, ..rest], _) if delete <= 3 && delete > 7 -> {
      fn(v16) { 2.0 }("abc")
    } +. {
      1.0
    }
    #(3, x), #([constructor, 5, ..], 2.0) -> 0.1
    _, _ -> {
      {
        2.0
      } *. {
        2.0
      }
    } +. {
      {
        1.0
      } +. {
        0.5
      }
    }
  }
}
