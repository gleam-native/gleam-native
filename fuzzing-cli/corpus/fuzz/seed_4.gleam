pub const pi_value: Bool = True
pub const euler_value: String = "bc"

pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: List(Int))
  Cv3(List(Int), value: Float)
}

pub type Promise {
  Cv4(value: List(Int))
  Cv5(Bool)
}

fn constructor(constructor: Bool, v6: #(List(Int), String)) -> String {
"res"
}

fn yield(v7: String, default: Float) -> List(Int) {
{
    let new = 100 - {
      {
        let n = 0.5
        let v7 = 2
        v7
      }
    }
    [3]
  }
}

fn default(new: List(Int), self_: List(Int), arguments: Bool) -> Bool {
{
    case fn(v8, v9) { 1.5 }(True, "data"), constructor(False, #([], "b")) {
      _, _ -> fn(v10, v11) { 0.1 }(2, 3.14)
      item, "ab" -> {
        1.0
      } *. {
        0.1
      }
      _, "abc" <> rest as whole -> {
        0.25
      } -. {
        2.0
      }
    }
  } != {
    0.5
  }
}

pub fn main() {
  echo ""
  echo case "" <> "res" {
    _ | "abc" <> _ -> fn(v12, v13) { v12 +. v12 }(1.5, "b")
    "constructor" | "abc" <> _ -> 0.0
    b -> {
      fn(v14) { v14 }(0.1)
    } -. {
      0.5
    }
  }
  echo 7
  echo case "a" {
    item | "data" <> item -> {
      False |> constructor(#([], ""))
    } <> ""
    "ab" -> {
      "res" <> euler_value
    } <> "b"
  }
}
