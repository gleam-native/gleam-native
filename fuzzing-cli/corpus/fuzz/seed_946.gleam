pub type V0 {
  Error(value: String, inner: Bool)
  Cv1(value: Bool)
  Number(Bool, String)
}

fn default(this_: V0, v2: Int, v3: String) -> Int {
case this_ {
    Cv1(True) as whole -> case 0.25 {
      v4 -> 10
      0.25 -> v2
    }
    Number(_, _) -> {
      fn(v5) { 4 }(True)
    } * {
      v2 % 6
    }
    Error("abc" <> _, v6) -> case "b" {
      "a" <> rest -> v2
      constructor -> v2
    }
    v7 -> v2
  }
}

fn new(v8: #(String, List(Int))) -> Int {
default(case <<"bc":utf8, "a":utf8, "constructor":utf8>>, default(Number(False, "a"), 3, "a") {
    <<2:16>>, 3 -> Number(False, "x")
    <<4:16>>, 0 -> Number(True, "abc")
    _, 7 -> Number(True, "")
    v9, _ -> Number(False, "data")
  }, {
    0 + 4
  } - default(Number(False, "ab"), 2, "constructor"), {
    let v8 = "constructor" == "data"
    "x"
  })
}

fn f2(v10: #(List(Int), Int), z: List(Int)) -> Int {
5
}

pub fn main() {
  let length = 1.0
  let acc = {
    "abc" <> "x"
  } <> {
    fn(v11) { "res" }(10.0)
  }
  echo case fn(v12, v13) { 100.0 }(0.0, True) {
    _ -> fn(v14) { {
      let arguments = 0.25
      v14
    } }("")
    _ -> {
      let item = {
        let m = []
        let n = [7]
        acc
      }
      "a"
    }
  }
}
