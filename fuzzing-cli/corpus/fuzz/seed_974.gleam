pub const seed_value: String = "b"

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(new: List(Int), this_: #(Float, Int), prototype: Bool) -> Float {
fn(v2, v3) { fn(v4, v5) { 2.0 }(0.0, 7) }(2, False)
}

fn f1(v6: Int, v7: Bool, v8: String) -> Bool {
{
    let arguments = v8
    {
      {
        let self_ = v7
        let v7 = self_
        False
      }
    } && v7
  }
}

fn extends(z: #(String, Bool), constructor: Int, prototype: Float) -> List(Int) {
[3, 1]
}

pub fn main() {
  let this_ = {
    3.14
  } -. {
    {
      0.5
    } +. {
      1.5
    }
  }
  let delete = 10
  echo case [10], extends(#("data", True), delete, this_) {
    [9, ..rest] as whole, [_, _, ..] -> fn(v9) { 10 }(5)
    [] as whole, [] -> 5
    _, v10 -> 5
  }
  echo case 2 {
    b -> b < {
      b + 42
    }
    item -> True
  }
  echo True
  echo case {
      let this_ = [4]
      this_
    } {
    [9, 6, ..] -> {
      seed_value <> "x"
    } <> ""
    [6, ..rest] -> seed_value <> {
      seed_value <> "abc"
    }
    [6, ..rest] -> case spin(10, delete), {
        let this_ = False
        let z = True
        delete
      } {
      7, _ -> {
        let m = rest
        let this_ = "a"
        "b"
      }
      0, 3 -> "a"
      _, v11 -> seed_value <> seed_value
    }
    v12 -> seed_value
  }
}
