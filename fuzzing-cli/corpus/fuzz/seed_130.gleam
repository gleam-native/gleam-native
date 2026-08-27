pub type V0 {
  Cv1(value: List(Int))
  Cv2
}

pub type Number {
  Some(Bool)
}

fn f0(pair: List(Int), v3: String, length: List(Int)) -> Bool {
case 1 * 10, {
      let x = False
      Cv2
    } {
    3, Cv2 -> case False {
      False -> 1 >= 5
      b -> b
      prototype -> prototype && prototype
    }
    _, Cv2 -> False || True
    0, v4 -> True
    _, v5 -> {
      let self_ = v3 <> v3
      {
        let v5 = False
        v5
      }
    }
  }
}

fn class(v6: Bool, s: Int, constructor: String) -> String {
case {
      let constructor = []
      "constructor"
    } {
    b -> {
      b <> "abc"
    } <> "res"
    _ | "abc" -> case Cv1([100, 2]) {
      b -> "data" <> constructor
      Cv2 as whole -> constructor <> "a"
      Cv1([constructor, ..rest]) -> ""
    }
  }
}

fn default(v7: Int, v8: Float, v9: String) -> String {
case Cv2, 5 {
    v9, _ -> "abc" <> {
      True |> class(3, {
        let m = v8
        "abc"
      })
    }
    Cv2 as whole, v10 -> "data"
    _, 4 -> v9
  }
}

pub fn main() {
  let this_ = 100
  let this_ = "x"
  echo case <<"a":utf8>>, [] {
    <<_:utf8>>, [this_] if this_ == 8 && this_ == 4 -> "constructor"
    <<3:8, _:utf8, 5:16>>, [] -> this_
    _, [] -> case [10] {
      [2] -> "b" <> "abc"
      [8, 2, ..] -> this_
      [4] as whole -> "abc"
      v11 -> "bc"
    }
    _, _ -> this_
  }
}
