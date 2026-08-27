pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
  Ok
}

pub type V3 {
  Cv4(String, value: Bool)
}

pub type V5 {
  Cv6(value: String)
  Cv7(Int)
}

fn f0(value: #(Float, String)) -> Bool {
False
}

fn default(acc: Int, delete: Int) -> Int {
2
}

fn f2(self_: Bool, v8: V5) -> Bool {
{
    case fn(v9) { Ok }("res") {
      constructor -> 0
      Cv2 -> {
        let acc = 4
        42
      }
      _ | Cv2 -> 10 % 3
    }
  } > {
    case <<"a":utf8>> {
      <<5:8, _:8, _:16>> as whole -> {
        let self_ = 10.0
        5
      }
      <<"":utf8, 1:16>> as whole -> 5
      _ -> default(3, 42)
    }
  }
}

pub fn main() {
  let constructor = "data" <> {
    "" <> "ab"
  }
  echo case default(10, 0) {
    inner -> constructor <> constructor
    _ -> fn(v10, v11) { {
      let v10 = [0, 3]
      "res"
    } }(True, 0)
  }
  echo fn(v12) { {
    let l = constructor
    2.0
  } }(3)
  echo fn(v13, v14) { case #(0.0, "bc") |> f0() {
    v13 -> v14
    False -> fn(v15, v16) { "abc" }(True, 1.5)
  } }(True, "a")
  echo 0.0
}
