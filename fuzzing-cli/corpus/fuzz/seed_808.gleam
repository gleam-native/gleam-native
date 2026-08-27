pub const golden_value: String = "a"

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  None(Int, String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(v2: #(Bool, Bool)) -> Int {
spin(spin(4, 4), 10)
}

fn f1(v3: Float, delete: Int) -> String {
{
    "ab" <> {
      "ab" <> "constructor"
    }
  } <> {
    "ab" <> {
      "abc" <> "res"
    }
  }
}

fn f2(v4: List(Int), length: V0, new: Int) -> Int {
fn(v5, v6) { {
    0 * 4
  } % 1 }(3, False)
}

pub fn main() {
  let class = f1(0.1, 0)
  echo {
    2.0
  } -. {
    case True, [4, 7] {
      False, [] -> {
        100.0
      } *. {
        10.0
      }
      _, [5] -> 0.5
      class, [_] -> 3.14
      v7, _ -> {
        0.25
      } /. {
        10.0
      }
    }
  }
  echo 5
}
