pub const limit_value: Bool = False
pub const euler_value: String = "x"
pub const tag_value: Bool = False

pub type Object {
  Record
}

pub type V0 {
  Cv1(value: Float)
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(v3: Float) -> Bool {
True
}

pub fn main() {
  echo case {
      let length = 0.1
      let constructor = limit_value
      Record
    }, [10, 100] {
    _, [] -> fn(v4) { {
      let new = [42]
      let euler_value = []
      [5, 10]
    } }(0)
    Record, [constructor] -> [1]
    _, v5 -> case [4, 10] |> walk(100 * 4), {
        let class = v5
        let x = [2]
        v5
      } {
      _, [a] if a <= 2 -> v5
      v, [] -> []
      _, _ -> [5, 4]
    }
  }
  echo 0.0
  echo {
    let limit_value = 100 >= {
      4 * 4
    }
    {
      10.0
    } -. {
      0.1
    }
  }
  echo False
}
