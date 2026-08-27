pub const pi_value: Float = 0.1
pub const golden_value: Int = 42

fn yield(constructor: Float, v0: Int, new: #(String, Float)) -> List(Int) {
[5, 42]
}

fn f1(v1: Int) -> List(Int) {
case fn(v2) { v1 }(False) {
    item -> []
    b -> case <<0:16>> {
      <<"":utf8>> -> {
        let v1 = v1
        []
      }
      _ -> {
        10.0
      } |> yield(v1 + v1, {
        let m = 7
        let acc = 0.0
        #("abc", 0.1)
      })
    }
  }
}

fn f2(s: List(Int), v3: Int, v4: Float) -> Bool {
case 4 {
    7 -> !True
    item -> True
  }
}

pub fn main() {
  let pi_value = case "a" {
    b | "x" <> b -> 42
    _ -> 7
  }
  echo case fn(v5) { "" }(True) {
    "abc" -> case True {
      False -> 100.0
      b -> 100.0
      inner -> 0.25
    }
    item -> case fn(v6) { False }(0.25), <<"abc":utf8, "bc":utf8>> {
      _, <<"res":utf8>> -> {
        10.0
      } -. {
        10.0
      }
      False as whole, <<"b":utf8>> -> fn(v7, v8) { v7 }(0.5, 0)
      True, _ -> 1.0
      v9, _ -> 1.5
    }
    "res" <> rest -> {
      1.0
    } +. {
      {
        1.5
      } *. {
        10.0
      }
    }
  }
}
