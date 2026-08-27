pub const tag_value: String = "constructor"
pub const pi_value: Float = 2.0
pub const euler_value: Bool = True

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(n: Int) -> List(Int) {
[0]
}

fn f1(v0: String) -> Bool {
fn(v1, v2) { !{
    fn(v3, v4) { True }(4, "constructor")
  } }(5, True)
}

pub fn main() {
  echo {
    {
      {
        0.1
      } -. pi_value
    } *. {
      {
        let tag_value = []
        let pi_value = euler_value
        0.1
      }
    }
  } == {
    pi_value -. pi_value
  }
  echo tag_value
  echo {
    let pair = case fn(v5, v6) { "a" }("x", True) {
      _ -> [0, 1]
      a -> fn(v7) { [] }(2.0)
    }
    let l = {
      {
        let pair = tag_value
        4
      }
    } + 3
    pair
  }
  echo {
    let constructor = 10
    100.0
  }
}
