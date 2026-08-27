pub const euler_value: Bool = True
pub const limit_value: Int = 10

pub type V0 {
  Error(value: String, inner: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(arguments: Bool, v1: Int, self_: V0) -> Int {
v1
}

fn new(v2: Float, v3: Bool) -> Float {
v2 -. {
    {
      2.0
    } +. {
      0.5
    }
  }
}

fn f2(v4: V0, acc: Int) -> Float {
case "", 100 {
    "" <> _, v -> {
      {
        0.1
      } +. {
        2.0
      }
    } /. {
      1.0
    }
    arguments, _ -> case #(2, "data") {
      constructor -> 2.0
      #(9, "a") -> new(3.14, False)
      a -> 0.1
    }
  }
}

pub fn main() {
  let euler_value = {
    {
      let euler_value = []
      "x"
    }
  } <> {
    {
      let self_ = "constructor"
      "res"
    }
  }
  let limit_value = case {
      0.5
    } +. {
      0.0
    } {
    0.0 as whole -> [3]
    b -> []
    0.0 -> [7]
  }
  echo case #(0.5, 2.0), 42 != 3 {
    #(100.0, 100.0), False as whole -> case {
        let m = 10
        "data"
      }, Error("data", "x") {
      _, Error(_, _) -> limit_value
      v5, Error("x", "data" <> rest) -> {
        let n = 2
        limit_value
      }
      v6, _ -> limit_value
    }
    #(_, _), True -> {
      let euler_value = {
        let euler_value = 10
        let n = 0
        "b"
      }
      {
        let v = 5
        [2, 100]
      }
    }
    _, _ -> {
      let limit_value = {
        let limit_value = 42
        [1]
      }
      let limit_value = {
        let pair = 5
        True
      }
      [3]
    }
  }
  echo case fn(v7, v8) { 2 }("data", "res"), 4 {
    _, _ -> fn(v9, v10) { limit_value }("res", False)
    3, _ -> fn(v11, v12) { limit_value }("a", 0.5)
    6, 9 -> {
      let constructor = 0 - 3
      let constructor = [3, 7]
      constructor
    }
  }
  echo False
}
