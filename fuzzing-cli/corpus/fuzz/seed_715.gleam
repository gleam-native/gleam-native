pub const euler_value: String = "ab"

pub type V0 {
  Record(value: String, inner: Bool)
  Cv1
  Number(value: Float, inner: Bool)
}

pub type V2 {
  Error(value: Bool, inner: String)
}

fn f0(delete: Bool, class: Int, this_: V2) -> Bool {
case 5 {
    item -> case "b" <> "a", fn(v3, v4) { True }(True, True) {
      "data" <> rest, _ if rest == "ab" -> False
      "data" <> rest, _ -> {
        let class = rest
        let rest = [0, 2]
        True
      }
      v5, v6 -> delete
    }
    a -> case 1 + a, 100.0 {
      4, 10.0 as whole -> delete
      7, _ -> False
      v7, _ -> True
    }
    item -> {
      {
        let item = 42
        let new = 0.1
        2
      }
    } < 100
  }
}

fn export(x: Float) -> String {
"res"
}

pub fn main() {
  let value = {
    let euler_value = f0(False, 0, Error(True, "x"))
    "ab" <> ""
  }
  echo case {
      let self_ = True
      let arguments = euler_value
      "ab"
    } {
    "ab" <> rest | "" <> rest -> {
      let euler_value = 42
      {
        let new = 0.5
        let length = 2.0
        [7, 1]
      }
    }
    value -> case Error(False, "b"), export(100.0) {
      _, v8 -> [0, 100]
      Error(acc, new), _ if new != "a" || !acc -> fn(v9, v10) { [100] }(3, "abc")
      Error(True, "" <> rest), "a" -> []
    }
    b | "res" <> b -> case True {
      constructor -> [2, 1]
      False | False -> [3, 2]
    }
  }
  echo case {
      let constructor = 1
      Error(True, "bc")
    } {
    Error(False, "constructor" <> _) -> 2.0
    v11 -> fn(v12) { 2.0 }("constructor")
    Error(_, inner) -> 0.0
  }
}
