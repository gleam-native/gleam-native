pub type V0 {
  Cv1(value: List(Int))
  Cv2
  Ok(Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v3: Int, item: Int, length: String) -> String {
"res" <> {
    case "bc" {
      v4 -> length
      a -> "abc" <> length
    }
  }
}

pub fn main() {
  let this_ = case Cv2, "ab" {
    Cv1([4]), "a" -> [10, 10]
    Cv2, _ -> []
    Ok(rest), "ab" <> tail -> fn(v5) { [5, 5] }(False)
    _, v6 -> []
  }
  echo case 2 {
    _ -> case this_, {
        let acc = True
        this_
      } {
      [a, ..rest], [9, _, ..] -> [42]
      [a] as whole, [8] if a <= 0 || a <= 7 -> {
        let s = 0.5
        let y = a
        [4]
      }
      [], [6, constructor, ..] as whole -> fn(v7, v8) { whole }(3, "abc")
      v9, _ -> {
        let l = this_
        let l = 0
        v9
      }
    }
    _ | 8 -> [0]
  }
  echo False
}
