// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

pub type Shape {
  Circle(radius: Float)
  Rect(width: Float, height: Float)
  Point
}

pub type Person {
  Person(name: String, age: Int)
}

fn area(shape: Shape) -> Float {
  case shape {
    Circle(radius) -> 3.0 *. radius *. radius
    Rect(width, height) -> width *. height
    Point -> 0.0
  }
}

fn birthday(person: Person) -> Person {
  Person(..person, age: person.age + 1)
}

fn safe_div(a: Int, b: Int) -> Result(Int, String) {
  case b == 0 {
    True -> Error("division by zero")
    False -> Ok(a / b)
  }
}

pub fn main() -> Nil {
  echo area(Circle(1.5))
  echo area(Rect(2.5, 1.5))
  echo area(Point) == 0.0
  let alice = Person("Alice", 30)
  let older = birthday(alice)
  echo older.name
  echo older.age
  // Records are immutable: the original is untouched.
  echo alice.age
  echo safe_div(84, 2)
  echo safe_div(1, 0)
  // Deep structural equality.
  echo older == Person("Alice", 31)
  echo alice == older
  Nil
}
