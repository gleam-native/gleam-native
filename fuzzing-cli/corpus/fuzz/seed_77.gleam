pub const seed_value: Float = 1.5

pub type V0 {
  Cv1
  Cv2
  Cv3
}

pub type V4 {
  Some(String)
  Cv5(value: List(Int), inner: Bool)
}

fn f0(acc: String) -> List(Int) {
case False {
    item -> case 4 {
      1 | 7 -> fn(v6, v7) { [3, 7] }(True, 1.5)
      item -> {
        let n = [5]
        [42]
      }
    }
    True -> fn(v8) { {
      let acc = True
      [7]
    } }(False)
  }
}

fn yield(pair: #(List(Int), Bool), v9: Float) -> Float {
1.5
}

pub fn main() {
  let seed_value = seed_value +. {
    fn(v10, v11) { v10 }(0.1, 0)
  }
  let class = case <<7:16, "ab":utf8, "constructor":utf8>> {
    <<_:utf8, _:utf8>> -> [10, 4]
    _ -> f0("a")
  }
  echo "data"
  echo {
    "constructor" <> "x"
  } <> {
    "res" <> "x"
  }
  echo {
    let this_ = case "constructor" <> "bc" {
      a -> "data"
      b -> b <> b
      a | "x" <> a -> a
    }
    case True {
      True as whole -> f0("data")
      True -> []
      _ -> class
    }
  }
  echo False
}
