pub const golden_value: String = "ab"
pub const euler_value: Int = 1

pub type Number {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(new: Int, acc: String) -> Bool {
case {
      let arguments = False
      let arguments = True
      Record
    }, acc <> "res" {
    Record, "res" as whole if whole == "ab" -> case <<"x":utf8>> {
      <<"bc":utf8, 3:4>> -> False
      <<_:utf8>> -> False
      v0 -> False
    }
    Record, "data" -> False
    Record, "b" -> {
      let acc = {
        let this_ = new
        acc
      }
      True
    }
    _, _ -> case <<100:16, "abc":utf8, "abc":utf8>>, <<2:8, "data":utf8>> {
      <<v:big-unsigned-8, 7:1>>, <<0:16, "x":utf8>> -> True
      <<"ab":utf8>>, <<3:8, 0:8>> -> True && True
      <<constructor:16, _:utf8, _:bytes>>, _ -> True
      _, _ -> True
    }
  }
}

fn f1(v1: Int, v2: Number, v3: Number) -> Bool {
False
}

pub fn main() {
  let value = "constructor" <> golden_value
  echo case #(2.0, False) {
    #(length, False) -> {
      fn(v4, v5) { 2 }(True, 1.0)
    } * euler_value
    constructor -> euler_value + 0
  }
}
