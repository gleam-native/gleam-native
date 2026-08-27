pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Int)
}

pub type Number {
  Cv3(Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn arguments(rest: String, v4: String) -> Int {
spin({
    10 * 2
  } + {
    fn(v5) { 5 }(1.5)
  }, spin(3, 1) % 3)
}

fn f1(class: #(Bool, Bool), default: #(Bool, String)) -> String {
case <<3:4, "data":utf8>>, [4, 10] {
    <<_:utf8, value:little-signed-8>> as whole, [h] if value > 2 -> "res"
    <<_:utf8>>, [0] -> case Cv2(0) {
      Cv1([5]) -> "constructor"
      _ | Cv2(_) -> "constructor" <> ""
    }
    _, [] -> "a"
    _, _ -> "data"
  }
}

fn default(y: Int, m: List(Int), length: String) -> List(Int) {
case #(0.0, [5]), "bc" {
    #(_, [h, ..rest] as whole), "abc" <> _ -> case {
        let prototype = length
        let whole = False
        length
      }, y |> spin(fn(v6, v7) { v7 }(True, 5)) {
      _, 6 -> m
      "constructor", 0 -> {
        let s = "x"
        let m = y
        [0, 10]
      }
      "a", n -> fn(v8) { [4] }("data")
      v9, _ -> fn(v10, v11) { [] }(False, False)
    }
    #(_, [2, m, ..]), "res" <> rest if rest != "data" -> [3]
    #(1.0, [6] as whole), "data" <> _ as it -> [42]
    v12, v13 -> fn(v14) { [3, 10] }(False)
  }
}

pub fn main() {
  let m = 42 |> spin("a" |> arguments("data"))
  echo case "ab" {
    "a" -> False
    "res" <> item -> "b" == {
      item <> "b"
    }
    _ -> case <<"b":utf8>> {
      <<2:8>> -> 100 == m
      _ -> False
    }
  }
  echo {
    let x = {
      let new = m - m
      ""
    }
    spin(m, 1) - m
  }
  echo {
    case False, m - m {
      True, 0 -> {
        let m = False
        "x"
      }
      length, 1 -> {
        let new = True
        let v = []
        "data"
      }
      _, _ -> "a"
    }
  } <> ""
}
