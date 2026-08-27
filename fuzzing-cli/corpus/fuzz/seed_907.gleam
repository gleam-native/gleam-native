pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
  Ok
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(arguments: Int) -> Bool {
True
}

fn delete(v3: List(Int), v4: V0) -> Bool {
{
    case "" <> "bc" {
      _ -> {
        let v3 = [7]
        let l = 100
        l
      }
      item -> 4
      "a" <> _ -> 10 + 7
    }
  } |> f0()
}

fn f2(v5: Int) -> String {
"ab"
}

pub fn main() {
  let n = spin(3 % 3, 1)
  let delete = True
  echo 1.5
  echo 1
  echo "constructor"
  echo False
}
