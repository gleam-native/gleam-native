pub const euler_value: Bool = True

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: List(Int), v3: Float) -> String {
{
    {
      let default = [4, 1]
      let v2 = v2
      "abc"
    }
  } <> {
    case <<"a":utf8>> {
      <<"x":utf8>> -> "b"
      <<class:big-signed-8, 3:8, _:big-unsigned-8>> if class <= 4 -> "b" <> "ab"
      _ -> "abc" <> "x"
    }
  }
}

fn delete(n: Int, rest: Int, v4: String) -> Int {
n + n
}

pub fn main() {
  let prototype = "" <> f0([], 0.25)
  let euler_value = [] |> f0(2.0)
  echo {
    case {
        let y = True
        let prototype = [7, 3]
        prototype
      } {
      [_] as whole -> euler_value <> prototype
      [1, ..rest] -> {
        let delete = True
        euler_value
      }
      [x] -> "ab" <> "res"
      v5 -> prototype
    }
  } <> euler_value
}
