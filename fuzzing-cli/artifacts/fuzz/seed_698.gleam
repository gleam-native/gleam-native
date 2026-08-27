pub type Promise {
  Cv0(value: String, inner: Float)
  Cv1
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(m: #(Float, String), v2: Bool) -> List(Int) {
case {
      let x = True
      let m = v2
      "ab"
    } {
    b | "data" <> b -> [42, 10]
    "abc" <> _ | "x" -> []
    "data" -> case Cv0("a", 1.0), <<"a":utf8>> {
      Cv1, <<_:16>> -> [7, 0]
      Cv0("abc", prototype), _ -> {
        let s = "ab"
        let s = s
        [100]
      }
      _, v3 -> []
    }
  }
}

fn f1(v4: Bool, v5: Promise) -> String {
case 1 |> spin(spin(100, 1)), [10, 10] {
    5, [] -> case v5 {
      Cv0("" <> rest, v4) if rest != "" || v4 <=. 0.0 -> "bc" <> rest
      self_ -> {
        let s = "b"
        let s = s
        s
      }
    }
    _, [_] -> "bc"
    9, [] -> {
      {
        let self_ = [5, 100]
        "res"
      }
    } <> {
      {
        let item = [2, 5]
        let v4 = v4
        "a"
      }
    }
    v6, _ -> "b"
  }
}

pub fn main() {
  let acc = ""
  let acc = {
    {
      0.1
    } +. {
      0.5
    }
  } -. {
    {
      let rest = 2.0
      let acc = True
      1.5
    }
  }
  echo case {
      let acc = True
      let length = acc
      100.0
    }, <<"x":utf8>> {
    100.0 as whole, <<"res":utf8, "x":utf8>> -> whole
    _, <<10:8, v:big-unsigned-4>> -> acc
    100.0, _ -> {
      fn(v7, v8) { acc }("bc", True)
    } +. {
      {
        10.0
      } +. acc
    }
    v9, _ -> case "bc" <> "constructor" {
      b | "res" <> b -> v9 -. {
        0.1
      }
      a -> {
        let s = 4
        v9
      }
    }
  }
}
