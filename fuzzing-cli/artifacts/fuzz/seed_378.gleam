pub type Promise {
  Cv0(value: String, inner: String)
  Ok
}

pub type Map {
  Number
}

pub type V1 {
  None
  Cv2
  Cv3(List(Int), Int)
}

fn f0(x: Float, v4: Int, v5: Float) -> Int {
case "a" <> "x" {
    "res" <> rest -> case v4 * v4 {
      l -> 1
      v6 -> {
        let m = True
        v4
      }
    }
    _ -> v4
  }
}

pub fn main() {
  let delete = case !False {
    _ | False -> []
    _ -> [3]
  }
  let m = case None {
    None -> 2 * 42
    v -> 2
  }
  echo "bc"
  echo {
    {
      1.5
    } |> f0(m, 3.14)
  } + {
    case "ab", {
        let m = 2.0
        Number
      } {
      _, v7 -> m
      "data", _ -> fn(v8) { m }("constructor")
    }
  }
}
