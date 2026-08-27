pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Int)
  Cv3
}

pub type V4 {
  Cv5
  Cv6(String)
  Cv7
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(pair: String) -> List(Int) {
{
    let pair = spin(1, 3) |> spin(0)
    [1]
  }
}

fn export(item: #(String, Int), l: Int, n: Int) -> List(Int) {
[]
}

pub fn main() {
  echo {
    {
      "ab" == "a"
    } || {
      "ab" == "ab"
    }
  } || {
    case "constructor" <> "bc", <<10:1, 4:16, 100:16>> {
      "x", <<"":utf8>> -> {
        let length = True
        let z = length
        True
      }
      "bc" <> rest, _ -> rest != rest
      _, v8 -> 2 != 42
    }
  }
}
