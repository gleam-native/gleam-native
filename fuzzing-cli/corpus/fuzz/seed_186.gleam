pub const pi_value: Float = 1.5

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Number(value: Int, inner: Float)
  Cv2(Int)
}

fn f0(this_: Bool, v3: Int, y: Int) -> Float {
case {
      let this_ = 0.0
      let z = y
      #(3, True)
    } {
    #(3, False) -> case Cv1([5], 5) {
      Number(y, 0.1) -> {
        let new = this_
        let s = new
        0.25
      }
      Number(_, item) -> item /. {
        3.14
      }
      b -> {
        let m = 0.5
        let m = 3
        1.5
      }
    }
    #(_, True) | #(5, False) -> case Cv2(1), 1 {
      _, 2 -> {
        3.14
      } *. {
        2.0
      }
      Cv2(_), 3 -> {
        0.25
      } +. {
        1.5
      }
      item, v3 -> {
        0.0
      } *. {
        100.0
      }
    }
    _ -> case Cv2(10) {
      inner -> {
        2.0
      } +. {
        0.25
      }
      b -> {
        2.0
      } -. {
        10.0
      }
      Number(item, _) -> {
        1.0
      } +. {
        0.25
      }
    }
  }
}

fn f1(v4: #(List(Int), List(Int)), v5: Float) -> Float {
f0(False, 2, 1 - {
    3 - 0
  })
}

fn f2(item: Int, this_: Int) -> Bool {
False && {
    {
      1.5
    } != {
      fn(v6, v7) { 2.0 }(True, 100.0)
    }
  }
}

pub fn main() {
  let pi_value = fn(v8) { True }("x")
  let arguments = {
    let pi_value = pi_value
    False
  }
  echo "b"
}
