pub const golden_value: Int = 10

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(acc: Int) -> List(Int) {
case "ab" {
    "x" <> a -> case "a" {
      "x" -> [2]
      "bc" -> [100]
      v0 -> [7]
    }
    "b" | "constructor" <> _ -> case acc + acc {
      _ -> [100, 0]
      1 -> [5, 2]
    }
    "b" <> item -> {
      let acc = 0.0
      let value = {
        let arguments = False
        let rest = [100, 3]
        "bc"
      }
      [42, 1]
    }
    v1 -> [10]
  }
}

fn f1(new: Int, rest: Int, v2: Int) -> Float {
{
    {
      {
        0.25
      } *. {
        0.5
      }
    } +. {
      0.25
    }
  } -. {
    {
      0.5
    } /. {
      10.0
    }
  }
}

pub fn main() {
  echo {
    {
      let item = {
        let constructor = 3.14
        let l = True
        [3]
      }
      "bc" <> "b"
    }
  } <> {
    "b" <> {
      "x" <> "data"
    }
  }
  echo case "data" {
    b -> True
    v3 -> False
    "ab" <> item -> True
  }
}
