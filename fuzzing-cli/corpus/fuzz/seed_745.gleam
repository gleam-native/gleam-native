fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(s: #(List(Int), Bool)) -> List(Int) {
fn(v0) { {
    let s = 0 |> spin(spin(v0, v0))
    let item = 3.14
    [42]
  } }(4)
}

fn arguments(v: String, v1: Int, prototype: String) -> Float {
{
    let s = {
      let v1 = 5 <= v1
      let constructor = "res"
      100 + 0
    }
    0.0
  }
}

pub fn main() {
  let length = 10.0
  echo [2]
  echo {
    10.0
  } *. {
    fn(v2, v3) { arguments("x", v2, "b") }(5, 1.5)
  }
}
