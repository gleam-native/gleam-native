pub type Map {
  Cv0(value: String, inner: List(Int))
  Cv1(value: String, inner: Bool)
}

pub type Promise {
  Some(value: Int)
  Cv2
}

pub type V3 {
  Cv4
  Cv5
}

fn f0(v6: String, rest: Int, v7: #(List(Int), List(Int))) -> Int {
rest
}

fn f1(v8: V3, acc: List(Int)) -> Int {
{
    let acc = {
      let default = False
      "data"
    }
    {
      let z = [1]
      100
    }
  }
}

pub fn main() {
  let x = case <<"data":utf8, "data":utf8>>, "bc" <> "a" {
    <<10:16>>, "ab" <> _ -> [10, 3]
    <<"a":utf8>>, "bc" as whole -> {
      let this_ = True
      let whole = [10]
      whole
    }
    _, "a" -> [3, 7]
    _, v9 -> {
      let default = "abc"
      []
    }
  }
  let new = 3
  echo f1(case new {
    6 -> Cv5
    a -> Cv4
  }, x)
}
