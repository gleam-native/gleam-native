pub const euler_value: Int = 5
pub const seed_value: Int = 100

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, s: String, n: List(Int)) -> List(Int) {
{
    let value = [42, 3] |> walk(5 + 1)
    let rest = case <<"b":utf8>>, "data" {
      <<4:8>>, _ -> s
      <<_:utf8, _:big-signed-1>>, value -> s <> constructor
      <<0:4, _:utf8>>, _ -> s <> constructor
      _, v0 -> "abc"
    }
    [100]
  }
}

pub fn main() {
  let s = []
  let seed_value = case {
      let seed_value = seed_value
      let prototype = euler_value
      s
    }, fn(v1) { seed_value }(1.5) {
    [6, ..rest], 7 -> euler_value * seed_value
    [a], 5 -> 10
    [euler_value], _ -> 42
    v2, v3 -> walk([1, 100], v3)
  }
  echo euler_value
  echo 0.25
}
