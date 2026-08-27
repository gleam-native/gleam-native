pub type V0 {
  Cv1
  Cv2
  Cv3(value: Float)
}

fn f0(v4: Float, s: List(Int), n: Bool) -> List(Int) {
case 0, "a" <> "constructor" {
    1 as whole, "abc" if whole == 0 -> s
    0, "b" <> rest -> case rest {
      inner | "constructor" <> inner -> [4]
      "bc" <> _ -> {
        let m = [1, 7]
        let prototype = 3.14
        [7]
      }
      "b" | "abc" -> s
    }
    _, _ -> {
      let new = {
        100.0
      } /. {
        2.0
      }
      let y = n
      []
    }
  }
}

pub fn main() {
  echo 4
  echo {
    1.5
  } <=. {
    {
      {
        let length = [2]
        let z = 0.5
        2.0
      }
    } +. {
      1.0
    }
  }
  echo case "abc" {
    "res" <> item | "abc" <> item -> [0, 100]
    "" <> b -> {
      let b = {
        3.14
      } /. {
        1.0
      }
      [0, 3]
    }
    _ -> case 7 {
      2 -> {
        0.25
      } |> f0(fn(v5, v6) { [4, 3] }(True, True), {
        0.5
      } == {
        3.14
      })
      6 -> [1]
      5 -> f0(0.25, [5], True)
      _ -> [42]
    }
  }
  echo case Cv2 {
    Cv2 -> case "bc" <> "b", True {
      "res", True as whole -> "res" <> "abc"
      _, _ -> "constructor" <> ""
    }
    Cv3(a) -> {
      let acc = True
      let y = False || False
      "abc"
    }
    _ -> case 100.0 {
      v7 -> ""
      2.0 -> "ab" <> ""
      3.14 as whole -> {
        let acc = 42
        let v = "data"
        ""
      }
    }
  }
}
