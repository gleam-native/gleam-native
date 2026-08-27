pub const tag_value: Bool = True
pub const euler_value: Float = 0.1
pub const golden_value: Bool = True

fn f0(m: String) -> Float {
3.14
}

fn extends(item: Bool, v0: Int, m: Int) -> List(Int) {
{
    let length = fn(v1, v2) { item }(3.14, False)
    [2, 1]
  }
}

pub fn main() {
  let euler_value = case 10.0 {
    0.5 -> 1.5
    a -> a
    3.14 -> euler_value
  }
  echo {
    let x = "res"
    "constructor" == "x"
  }
  echo case "x" {
    _ -> fn(v3) { 0 - 100 }(True)
    "" <> rest -> case <<"a":utf8>>, #(7, "x") {
      <<_:utf8>>, #(5, _) -> 5 + 100
      <<constructor:16, 100:1>>, #(2, "res") -> constructor
      _, #(8, "res" <> _) as whole -> 2
      v4, _ -> 42
    }
  }
  echo case "data" {
    "abc" <> rest -> case {
        let acc = tag_value
        let m = [10]
        rest
      } {
      a -> rest <> rest
      b | "res" <> b -> rest <> "res"
    }
    "x" <> _ | "constructor" <> _ -> "constructor" <> {
      fn(v5, v6) { "" }("a", "a")
    }
    _ -> ""
  }
  echo fn(v7) { golden_value }("a")
}
