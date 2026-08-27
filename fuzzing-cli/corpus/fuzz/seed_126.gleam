pub type V0 {
  None(value: String, inner: Bool)
}

pub type V1 {
  Cv2(value: String, inner: String)
  Ok(Float, String)
  Cv3(value: Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn class(v4: String) -> String {
v4
}

pub fn main() {
  let acc = 2
  let constructor = case Ok(1.5, "bc") {
    Ok(3.14 as whole, "bc") -> [42]
    inner -> fn(v5, v6) { [4] }(True, "abc")
    acc -> []
  }
  echo constructor
  echo False
  echo case acc {
    b -> True || False
    constructor -> True && True
    inner -> case {
        0.25
      } /. {
        0.5
      } {
      a -> True
      v7 -> False
      inner -> fn(v8, v9) { True }("a", 2)
    }
  }
  echo case class("") {
    item -> item
    a | "x" <> a -> a
    item | "" <> item -> {
      let y = 0.5
      let x = acc % 2
      item
    }
  }
}
