pub type Promise {
  Cv0(value: String, inner: String)
}

pub type V1 {
  Cv2
  Cv3(List(Int))
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(s: String, prototype: V1, v: Int) -> Bool {
case #([42], []), <<"x":utf8>> {
    #([s], [_]), <<_:8>> as whole if s == 7 -> case Cv3([42]) {
      whole -> True || False
      b -> 4 >= 42
    }
    #([9, 0, ..], [_, ..rest]), _ -> case fn(v4) { "b" }("res"), "" {
      "x", "data" <> rest if rest == "a" || rest == "abc" -> True
      constructor, "bc" if constructor != "a" || constructor == "constructor" -> True
      "res" <> rest, "data" -> False
      _, _ -> True
    }
    v5, _ -> spin(42, v) < {
      10 * v
    }
  }
}

pub fn main() {
  let n = {
    let v = "abc"
    [100]
  }
  let n = "constructor"
  echo case n <> "x" {
    "ab" -> 1.0
    "bc" -> {
      fn(v6) { v6 }(1.5)
    } /. {
      10.0
    }
    _ -> {
      {
        0.25
      } +. {
        0.1
      }
    } +. {
      0.1
    }
  }
  echo [4]
}
