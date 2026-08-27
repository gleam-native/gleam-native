pub const pi_value: Int = 100
pub const limit_value: Bool = False
pub const euler_value: Int = 10

fn f0(m: String, this_: Float, v0: Int) -> List(Int) {
[4]
}

fn f1(v1: Int, v2: Bool, length: #(Float, Float)) -> Float {
{
    case <<100:8>> {
      <<0:16>> -> {
        10.0
      } *. {
        1.0
      }
      _ -> 3.14
    }
  } +. {
    1.0
  }
}

fn f2(class: Bool, arguments: Float, v3: Float) -> Int {
2 - {
    2 - 5
  }
}

pub fn main() {
  let euler_value = False
  let arguments = euler_value
  echo f0(case "constructor" <> "abc" {
    "a" -> "res"
    "a" <> inner | "ab" <> inner -> "b" <> "b"
    "b" <> rest | "abc" <> rest -> "abc"
    v4 -> "bc"
  }, f1(fn(v5) { pi_value }("data"), {
    let pi_value = True
    let v = "a"
    arguments
  }, #(2.0, 3.14)), 0)
  echo case 10.0 {
    a -> euler_value || {
      limit_value && euler_value
    }
    limit_value -> case [] {
      [euler_value] -> limit_value != {
        0.25
      }
      [] -> "" == "data"
      v6 -> pi_value > 4
    }
  }
  echo 5
}
