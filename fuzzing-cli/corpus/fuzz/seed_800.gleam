pub const limit_value: Bool = True
pub const tag_value: String = "b"

pub type V0 {
  Cv1(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(item: Bool, acc: #(List(Int), Bool)) -> List(Int) {
[3]
}

pub fn main() {
  let z = f0({
    0.1
  } != {
    0.5
  }, {
    let n = [42]
    #([], False)
  })
  let item = 10.0
  echo "b" <> "abc"
  echo fn(v2) { case <<0:16>>, 1 * 42 {
    <<"data":utf8>>, prototype if prototype > 9 -> tag_value
    <<"ab":utf8>>, _ -> tag_value
    <<10:16, "abc":utf8>>, 8 -> tag_value <> tag_value
    v3, _ -> tag_value <> tag_value
  } }(False)
  echo case Cv1([3]), fn(v4, v5) { 7 }("abc", 0.25) {
    Cv1([5, x, ..] as whole), tag_value -> True
    Cv1([]), _ -> limit_value
    _, _ -> False
  }
}
