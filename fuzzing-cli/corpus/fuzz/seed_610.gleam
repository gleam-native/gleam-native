pub const tag_value: String = "bc"

pub type V0 {
  Error(value: String, inner: Int)
  None(List(Int), Float)
}

pub type V1 {
  Record
  Cv2(String)
  Number
}

fn f0(prototype: V0, v3: Int) -> List(Int) {
[]
}

fn f1(v4: String) -> Bool {
True
}

fn f2(arguments: #(Float, Int), this_: Int, v5: #(Bool, Int)) -> Bool {
fn(v6, v7) { {
    1.0
  } >=. {
    v6 +. v6
  } }(1.0, 10)
}

pub fn main() {
  let n = "data" <> {
    "" <> "constructor"
  }
  let tag_value = {
    let item = {
      0.5
    } +. {
      2.0
    }
    let n = False
    {
      let y = False
      tag_value
    }
  }
  echo 100
  echo case <<"a":utf8>> {
    <<_:16>> -> 10
    _ -> case "ab" {
      n | "abc" <> n -> 7
      v8 -> fn(v9) { 5 }("a")
      b -> 1
    }
  }
  echo case #(10.0, 7) |> f2(3 + 4, #(False, 2)) {
    v -> case {
        let delete = [10, 4]
        let y = tag_value
        v
      }, 1 {
      _, 9 -> "ab" <> tag_value
      True, 5 -> tag_value
      False, _ -> "bc"
      v10, v11 -> fn(v12) { "res" }(False)
    }
    arguments -> case [7] {
      [n, constructor, ..] if constructor == 0 && n > 1 -> "constructor"
      [arguments, ..rest] -> n <> tag_value
      [x] -> fn(v13, v14) { v14 }("", "data")
      v15 -> fn(v16) { n }(0.5)
    }
  }
  echo case 0 - 0 {
    b -> case f0(None([5], 0.5), b), b {
      [_, h, ..], rest if rest <= 2 -> f0(None([4, 3], 1.5), 1)
      [_], 8 -> [0, 7]
      v17, _ -> {
        let s = v17
        [5, 7]
      }
    }
    tag_value -> fn(v18, v19) { [7, 10] }(True, "a")
    constructor -> case constructor * 5 {
      _ | 4 -> []
      item -> [42, 1]
    }
  }
}
