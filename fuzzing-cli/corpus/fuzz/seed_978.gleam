pub const seed_value: Float = 100.0
pub const euler_value: String = "abc"
pub const tag_value: String = "abc"

pub type V0 {
  Cv1(value: List(Int))
  Cv2(Bool)
  Cv3(value: Float)
}

fn f0(z: String) -> List(Int) {
case 100, fn(v4) { 1 }(1) {
    9, _ -> {
      let acc = z <> "b"
      let z = {
        let acc = 7
        "x"
      }
      [1, 4]
    }
    8, _ -> [42, 10]
    4, _ -> case 1.0 {
      0.0 -> [100, 3]
      b -> {
        let prototype = False
        let b = 7
        [2]
      }
    }
    _, _ -> case <<0:16, 0:8>> {
      <<"":utf8, 4:8>> -> [100]
      <<_:little-signed-8, delete:little-signed-8>> if delete % 2 == 0 && delete <= 9 -> []
      <<"b":utf8>> -> [0]
      _ -> {
        let y = True
        let length = False
        []
      }
    }
  }
}

pub fn main() {
  let self_ = {
    0 - 100
  } * {
    5 + 3
  }
  let l = [42]
  echo case 10.0, Cv1([7]) {
    _, Cv3(v5) -> "constructor" != {
      "ab" <> euler_value
    }
    1.5, Cv3(0.1 as whole) as it -> True
    v6, _ -> case fn(v7, v8) { tag_value }(0.5, "a") {
      "ab" -> True
      "bc" -> True || False
      "x" as whole -> True
      v9 -> True
    }
  }
  echo l
}
