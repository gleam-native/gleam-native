pub const seed_value: Float = 0.25
pub const golden_value: Float = 1.5

pub type Number {
  Cv0(value: String, inner: Float)
}

pub type V1 {
  Cv2(Float, value: List(Int))
  Cv3(String, value: Float)
  Cv4
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(pair: Int, acc: Int, constructor: #(Float, String)) -> List(Int) {
[7]
}

pub fn main() {
  let golden_value = case [0, 42], spin(0, 5) {
    [golden_value], _ -> True
    [8, ..rest], 2 -> False
    [seed_value], 6 -> {
      let prototype = "x"
      False
    }
    _, _ -> True
  }
  echo {
    let l = {
      {
        let default = 1.5
        let length = "bc"
        length
      }
    } != {
      {
        let x = []
        let x = 10
        "abc"
      }
    }
    let rest = f0(42, 42, {
      let golden_value = [7, 100]
      let length = "a"
      #(0.1, "b")
    })
    {
      let y = 5 < 5
      let m = True
      m
    }
  }
  echo "x"
  echo {
    case Cv3("a", 0.1) {
      Cv3(item, _) if item == "data" -> item <> "constructor"
      Cv3("abc", 1.0) -> "ab"
      v5 -> "res" <> "bc"
    }
  } <> {
    {
      "bc" <> "res"
    } <> {
      "ab" <> "a"
    }
  }
  echo 100 < {
    case <<4:16, "bc":utf8, "abc":utf8>> {
      <<0:1>> -> {
        let default = [2, 10]
        let class = [7, 2]
        1
      }
      _ -> 4 % 2
    }
  }
}
