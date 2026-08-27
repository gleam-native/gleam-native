fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, v0: Float, this_: Bool) -> Int {
case spin(1, 4) {
    inner -> {
      1 + inner
    } |> spin(1 * inner)
    b -> 7
  }
}

pub fn main() {
  echo {
    case "res" == "x", {
        10.0
      } *. {
        0.25
      } {
      v1, 0.1 if v1 -> {
        let default = [2]
        let length = 100.0
        length
      }
      True as whole, _ -> 100.0
      _, _ -> 0.5
    }
  } -. {
    case "x" {
      "res" -> 0.0
      item -> {
        0.5
      } -. {
        1.0
      }
      "bc" | "abc" -> 1.5
    }
  }
  echo {
    100.0
  } *. {
    case 3 {
      v2 -> 0.0
      2 -> 100.0
      a -> 0.1
    }
  }
  echo {
    {
      {
        100.0
      } +. {
        0.5
      }
    } /. {
      10.0
    }
  } -. {
    {
      let x = {
        let arguments = 3
        let z = [0]
        0.1
      }
      let length = 1 * 0
      10.0
    }
  }
}
