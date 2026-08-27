pub type V0 {
  Number(value: String, inner: List(Int))
}

fn f0(v1: V0) -> String {
{
    let acc = case 0.1 {
      0.0 -> 7
      v1 -> {
        let n = ""
        let v1 = [42, 42]
        0
      }
      constructor -> 7 - 100
    }
    "x"
  }
}

pub fn main() {
  echo 4
  echo {
    0.5
  } -. {
    case 0 - 4 {
      self_ -> 3.14
      3 -> 3.14
    }
  }
  echo case {
      let y = 100.0
      2
    } {
    b -> {
      let delete = b * 2
      let length = {
        let b = 1.5
        delete
      }
      {
        let length = 7
        let length = 0.5
        7
      }
    }
    a -> a
  }
  echo 2
}
