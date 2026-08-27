pub const seed_value: Int = 1

pub type V0 {
  None(value: String, inner: List(Int))
}

pub type V1 {
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn arguments(v3: String, pair: Int, v4: Int) -> List(Int) {
case fn(v5) { 0.1 }(0.25) {
    0.1 | 100.0 -> case {
        let this_ = 2.0
        []
      } {
      [0, ..rest] -> [7]
      [constructor, 1, ..] -> {
        let rest = constructor
        [42, 2]
      }
      v6 -> {
        let class = True
        let pair = pair
        v6
      }
    }
    v7 -> case {
        let constructor = [3]
        let item = v3
        None("abc", [2, 1])
      } {
      item -> [2, 5]
      None("bc" as whole, [_, ..rest] as it) -> rest
    }
  }
}

fn default(value: V0) -> Bool {
4 > 42
}

fn f2(arguments: String, rest: Int) -> String {
{
    case rest - 7 {
      8 -> {
        let new = True
        let arguments = "x"
        arguments
      }
      inner -> fn(v8, v9) { arguments }(False, True)
      inner -> arguments
    }
  } <> "bc"
}

pub fn main() {
  let seed_value = 10 > seed_value
  echo []
  echo "data"
  echo 100 - 7
  echo case #([10, 42], 2) {
    #([8], 2) -> False
    #([1], _) -> case {
        let constructor = [0]
        let pair = 0.1
        1
      } {
      inner -> None("b", []) |> default()
      inner -> inner <= inner
    }
    _ -> case <<"data":utf8, "data":utf8>> {
      <<42:8>> -> 42 == 1
      _ -> seed_value
    }
  }
}
