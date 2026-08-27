pub const tag_value: String = ""

fn constructor(constructor: Float, v0: Int, delete: Bool) -> Int {
1
}

pub fn main() {
  let z = case tag_value, fn(v1) { #(True, 3.14) }(3) {
    "res", #(_, 0.25 as whole) -> tag_value <> tag_value
    "data", #(tag_value, 2.0) as whole -> "b"
    _, _ -> "ab" <> tag_value
  }
  echo z <> {
    tag_value <> "res"
  }
}
