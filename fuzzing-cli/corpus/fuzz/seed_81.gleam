pub type V0 {
  Ok(value: String, inner: List(Int))
}

pub type V1 {
  Cv2(value: List(Int), inner: Float)
  Cv3(value: Bool, inner: Int)
  Cv4(value: String, inner: Bool)
}

pub type V5 {
  None(value: Bool)
  Cv6(Bool, value: String)
}

fn f0(default: String) -> List(Int) {
case 3, True {
    _, True -> case <<5:8>> {
      <<"res":utf8>> -> []
      <<_:utf8>> -> []
      v7 -> [3]
    }
    4, True -> case default, "ab" {
      "ab" <> rest, "ab" if rest != "x" -> [1, 2]
      "b", _ -> [4]
      _, v8 -> [2, 1]
    }
    _, _ -> []
  }
}

fn f1(v9: String, arguments: Float, acc: Int) -> Int {
{
    let value = case True || True, acc {
      arguments, 8 -> fn(v10) { v9 }(True)
      False, 4 -> ""
      True, _ -> v9
      v11, v12 -> v9
    }
    let pair = [7, 2]
    0
  }
}

fn f2(v13: String) -> Int {
3
}

pub fn main() {
  let z = True
  echo {
    case Cv2([100, 3], 0.5) {
      Cv4(b, _) -> 100 <= 2
      Cv3(False, 4) | Cv4(_, _) -> False
      Cv3(constructor, _) -> constructor
      v14 -> {
        let this_ = z
        let v14 = False
        True
      }
    }
  } && z
  echo "a"
}
