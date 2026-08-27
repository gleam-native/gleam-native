pub type V0 {
  Some(value: String, inner: String)
}

pub type V1 {
  Cv2
  Cv3
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn class(self_: String, prototype: #(Bool, Float)) -> Int {
5
}

fn default(pair: String, v4: Int, v5: #(Bool, Bool)) -> Bool {
False
}

pub fn main() {
  echo case 100 {
    4 -> case 5, {
        0.25
      } -. {
        0.1
      } {
      v6, 3.14 -> [1]
      _, 0.5 -> [3]
      _, v7 -> []
    }
    6 -> [7]
    _ -> case "bc" {
      inner -> []
      b -> [5, 10]
      inner | "x" <> inner -> [42, 5]
    }
  }
  echo [0, 10]
}
