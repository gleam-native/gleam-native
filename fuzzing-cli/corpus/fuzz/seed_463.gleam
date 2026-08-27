pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type Symbol {
  Ok
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn default(acc: Bool, v2: Bool) -> String {
{
    case 5, {
        100.0
      } -. {
        0.25
      } {
      _, _ -> ""
      5, 1.0 -> {
        let y = ""
        let arguments = y
        y
      }
      1, 2.0 -> "a"
    }
  } <> {
    "res" <> {
      fn(v3, v4) { v4 }("res", "x")
    }
  }
}

pub fn main() {
  let prototype = True |> default({
    let prototype = "b"
    let l = 1
    False
  })
  let prototype = case spin(5, 10) {
    _ -> {
      let arguments = "bc"
      "bc"
    }
    8 -> "res"
  }
  echo 2
  echo {
    let m = case 3.14 {
      constructor -> [3]
      2.0 -> []
    }
    {
      let delete = True |> default(True)
      5
    }
  }
  echo case [100, 4], 1.0 {
    [_, ..rest] as whole, 1.5 -> case 10 {
      constructor -> False
      8 -> 3 <= 42
    }
    [_] as whole, 0.25 -> spin(2, 4) == {
      {
        let acc = 0
        let y = True
        10
      }
    }
    [6, _, ..], 1.0 -> True
    v5, v6 -> case v5 {
      [9] -> False
      [1, 6, ..] -> False
      _ -> False
    }
  }
}
