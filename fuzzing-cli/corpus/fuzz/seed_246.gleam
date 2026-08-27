fn f0(m: String) -> List(Int) {
case 10 {
    b -> [3, 4]
    item -> case "a", <<"a":utf8>> {
      "constructor" as whole, <<_:utf8>> if whole == "ab" -> fn(v0, v1) { [] }(True, 42)
      "ab", <<"ab":utf8, 10:1>> -> {
        let y = 1.5
        let v = item
        [7, 10]
      }
      "x", _ -> [2]
      v2, _ -> [1]
    }
    9 | 2 -> case "a" {
      new | "" <> new -> fn(v3, v4) { [] }(True, 1)
      _ | "res" <> _ -> []
    }
  }
}

pub fn main() {
  echo fn(v5, v6) { {
    fn(v7, v8) { v8 }(1, True)
  } || {
    False && True
  } }(10, "res")
  echo {
    3.14
  } +. {
    fn(v9) { 0.1 }(False)
  }
}
