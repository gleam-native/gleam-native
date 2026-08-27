pub const euler_value: Int = 5

pub type V0 {
  Number(value: String, inner: String)
  Cv1(value: Bool)
}

fn f0(v2: #(Float, Int)) -> Int {
{
    let v2 = {
      {
        let value = "bc"
        let value = True
        value
      }
    } && {
      42 >= 5
    }
    let rest = 42 - 5
    case rest {
      _ -> 3
      this_ -> rest + rest
    }
  }
}

fn default(item: Bool, delete: Int) -> Bool {
case [0, 4] {
    [3, 3, ..] -> {
      {
        10.0
      } != {
        2.0
      }
    } || {
      {
        let item = 42
        let delete = 3
        True
      }
    }
    [_, b, ..] -> fn(v3, v4) { v4 != v4 }(1.5, "bc")
    v5 -> fn(v6, v7) { False }(0.1, 2.0)
  }
}

pub fn main() {
  let this_ = case #(100.0, False), fn(v8) { "a" }(10.0) {
    #(100.0, _), "x" -> {
      0.25
    } -. {
      0.25
    }
    #(_, _), "bc" -> fn(v9) { 0.5 }("ab")
    _, _ -> {
      100.0
    } -. {
      1.0
    }
  }
  let euler_value = {
    {
      let arguments = 1.5
      let this_ = 10
      10
    }
  } * {
    #(10.0, 10) |> f0()
  }
  echo case #(2.0, 5) |> f0() {
    8 -> case {
        0.0
      } == {
        1.0
      } {
      False | True -> []
      True -> [42, 10]
    }
    inner -> [1, 2]
  }
  echo fn(v10) { "a" }(False)
  echo case "b" {
    _ -> [4, 1]
    _ | "" <> _ -> [1]
    inner -> case [] {
      [3] -> [3]
      [] -> []
      [4, ..rest] -> rest
      _ -> [100]
    }
  }
}
