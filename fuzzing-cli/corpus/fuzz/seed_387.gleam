pub const pi_value: Float = 0.0
pub const tag_value: Float = 0.1

pub type Record {
  Cv0(value: String, inner: String)
}

fn new(prototype: String, length: Float) -> Bool {
False
}

fn default(v1: List(Int), v2: Record, v3: List(Int)) -> Int {
4
}

fn arguments(m: Record, v4: Int, v5: Int) -> Float {
{
    case [] {
      [x] if x > 4 || x <= 2 -> {
        1.0
      } *. {
        100.0
      }
      [a] as whole -> fn(v6) { 1.5 }("a")
      _ -> fn(v7, v8) { 0.0 }(0.0, True)
    }
  } +. {
    1.0
  }
}

pub fn main() {
  let pi_value = 0 - {
    3 + 100
  }
  echo default(case <<2:1>> {
    <<_:big-unsigned-8, 10:4>> -> fn(v9) { [4, 10] }("abc")
    <<"constructor":utf8>> as whole -> fn(v10, v11) { [] }("data", 7)
    _ -> [7, 1]
  }, Cv0("", "abc"), case {
      let v = 100
      let tag_value = v
      Cv0("res", "a")
    } {
    Cv0(_, new) if new == "abc" || new != "bc" -> []
    Cv0(_, "x") | Cv0(_, _) -> fn(v12) { [3] }(False)
    b -> [1]
  })
}
