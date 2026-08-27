pub const tag_value: Int = 42
pub const pi_value: String = "res"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v0: Int, v1: Bool) -> Bool {
case <<2:1, "":utf8>> {
    <<_:utf8>> -> True
    _ -> v1
  }
}

fn f1(n: List(Int), l: #(String, Float)) -> Int {
100
}

fn yield(v2: #(Int, List(Int)), class: Int) -> Float {
3.14
}

pub fn main() {
  echo {
    {
      {
        let s = "ab"
        let arguments = tag_value
        "constructor"
      }
    } <> {
      pi_value <> ""
    }
  } <> {
    {
      pi_value <> pi_value
    } <> {
      "ab" <> pi_value
    }
  }
}
