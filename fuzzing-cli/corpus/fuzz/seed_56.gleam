pub const seed_value: Float = 2.0

pub type Number {
  Record
  Cv0(value: List(Int))
  Cv1(List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(l: #(String, List(Int)), delete: String) -> String {
delete
}

pub fn main() {
  echo [4, 42]
  echo "a"
  echo 3
  echo {
    {
      1.5
    } +. {
      10.0
    }
  } +. {
    10.0
  }
}
