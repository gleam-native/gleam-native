pub const golden_value: Bool = False
pub const tag_value: Bool = False

pub type Promise {
  Cv0(value: String, inner: String)
  Cv1(Float, String)
  Cv2(value: String)
}

fn f0(z: Bool, arguments: Float) -> Float {
{
    {
      {
        100.0
      } -. {
        100.0
      }
    } -. arguments
  } /. {
    10.0
  }
}

fn default(z: String, v3: Promise) -> Float {
0.1
}

fn f2(v4: String, arguments: Int) -> List(Int) {
[3, 2]
}

pub fn main() {
  let l = tag_value
  let pair = "data" <> {
    fn(v5) { "bc" }(0.25)
  }
  echo True
  echo {
    let delete = [7, 10]
    let tag_value = case 3 {
      _ -> tag_value
      a -> pair == pair
    }
    case "data" <> "constructor", "constructor" <> "x" {
      _, _ -> pair |> f2(0 - 10)
      "res" <> _, "a" as whole -> f2("abc", 0)
    }
  }
}
