pub const euler_value: Bool = True

pub type Record {
  Cv0(value: String, inner: Bool)
  Cv1(value: Bool)
}

pub type V2 {
  Cv3(value: Bool, inner: Int)
  Cv4(Int)
  Some(Int)
}

pub type V5 {
  Cv6(String, String)
  Cv7
  Cv8
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v9: String, this_: Bool, v10: Bool) -> List(Int) {
case fn(v11) { [] }(42) {
    [constructor, ..rest] if constructor > 9 -> case <<4:16>> {
      <<1:16>> -> fn(v12) { [] }("data")
      <<_:utf8>> as whole -> rest
      _ -> rest
    }
    [] -> fn(v13, v14) { {
      let new = []
      [5]
    } }("bc", False)
    [3] -> []
    v15 -> []
  }
}

fn f1(constructor: String, v16: Int) -> Int {
fn(v17, v18) { case Cv7 {
    v16 -> {
      let z = "res"
      let rest = 2
      rest
    }
    Cv7 -> v16 - v16
  } }(False, False)
}

pub fn main() {
  let euler_value = case "bc" {
    "ab" <> a -> {
      let acc = [42, 7]
      False
    }
    constructor -> {
      let constructor = 10
      let class = constructor
      euler_value
    }
  }
  echo case <<"x":utf8>> {
    <<_:utf8>> -> case #(3.14, []) {
      b -> [2, 2]
      inner -> f0("b", euler_value, euler_value)
    }
    <<_:16, 3:8>> -> f0("", False, euler_value)
    _ -> []
  }
  echo True
}
