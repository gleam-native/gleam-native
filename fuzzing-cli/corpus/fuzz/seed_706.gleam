pub const pi_value: Bool = False
pub const golden_value: String = "abc"
pub const euler_value: Int = 4

pub type Record {
  Cv0(value: String, inner: Float)
  Error(value: Int, inner: List(Int))
  None
}

fn f0(v1: String, v2: Float) -> Bool {
{
    case fn(v3) { [100] }(False) {
      [6, ..rest] -> 4
      [3] -> 1
      [4, 2, ..] -> 10 - 100
      _ -> 3
    }
  } <= {
    {
      0 - 4
    } % 4
  }
}

fn f1(delete: Int, v4: Int, m: Record) -> Float {
{
    0.1
  } +. {
    {
      1.5
    } -. {
      10.0
    }
  }
}

fn extends(delete: Int) -> String {
{
    let n = "res"
    n
  }
}

pub fn main() {
  echo case euler_value {
    _ | 5 -> 0
    6 -> euler_value * {
      {
        let prototype = [42]
        1
      }
    }
  }
  echo case [], euler_value % 1 {
    [_], 2 -> case euler_value {
      3 -> {
        let x = [42]
        let self_ = "x"
        euler_value
      }
      constructor -> constructor
    }
    [], 6 -> case Cv0("constructor", 0.5) {
      Error(6, [7, 3, ..]) -> 0 - 7
      Cv0("b" <> rest, 0.1) if rest != "bc" -> fn(v5) { euler_value }(42)
      Error(constructor, _) -> constructor - euler_value
      _ -> euler_value - 100
    }
    [3, ..rest], _ -> case {
        let delete = 0.0
        None
      }, euler_value {
      v6, 0 -> 7
      None, v7 if v7 <= 4 -> {
        let pi_value = 1.0
        let class = False
        euler_value
      }
      Cv0("res" as whole, 3.14), _ -> euler_value + 3
      v8, _ -> euler_value
    }
    v9, _ -> 10
  }
  echo 1
}
