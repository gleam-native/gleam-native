pub type V0 {
  Cv1(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn delete(v2: Int) -> String {
"a"
}

fn yield(v3: #(Float, Int), v4: Int, constructor: Int) -> Float {
{
    case 0 {
      item -> 0.25
      2 -> fn(v5, v6) { 2.0 }(False, True)
      a -> {
        let item = a
        1.0
      }
    }
  } /. {
    2.0
  }
}

pub fn main() {
  let z = {
    {
      1.0
    } +. {
      0.5
    }
  } +. {
    {
      100.0
    } *. {
      3.14
    }
  }
  let new = [10]
  echo case Cv1([]), "ab" {
    _, "bc" as whole if whole == "res" || whole == "ab" -> case {
        let item = True
        Cv1([])
      }, {
        let m = True
        let new = 0
        new
      } {
      Cv1([9, ..rest]), _ -> 5 + 1
      _, 5 -> fn(v7) { 4 }(2)
      _, _ -> 1
    }
    Cv1([4, ..rest]), "bc" <> tail -> 5
    v8, _ -> {
      let pair = new
      let pair = {
        let m = "abc"
        let n = new
        m
      }
      10 + 4
    }
  }
}
