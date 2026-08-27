pub type V0 {
  Cv1
  Cv2
  Cv3
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(x: Int) -> Bool {
False
}

pub fn main() {
  let acc = {
    10.0
  } <. {
    fn(v4, v5) { 0.5 }(False, "res")
  }
  let acc = 0.25
  echo case {
      let acc = True
      let acc = 100.0
      0
    }, "data" == "bc" {
    7 as whole, v6 -> 42 + 4
    3, _ -> {
      {
        let acc = "constructor"
        7
      }
    } * {
      1 * 10
    }
    v7, v8 -> case "abc" <> "res" {
      a | "x" <> a -> 2
      "data" -> spin(v7, v7)
    }
  }
  echo {
    fn(v9) { 0.25 }(3.14)
  } *. {
    {
      acc /. {
        0.5
      }
    } -. {
      acc +. {
        0.0
      }
    }
  }
}
