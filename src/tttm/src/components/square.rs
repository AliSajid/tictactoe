// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2022
// *
// * This project is dual-licensed under the MIT and Apache licenses.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** APACHE 2.0 LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Licensed under the Apache License, Version 2.0 (the "License");
// * you may not use this file except in compliance with the License.
// * You may obtain a copy of the License at
// *
// *     http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** MIT LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Permission is hereby granted, free of charge, to any person obtaining a copy
// * of this software and associated documentation files (the "Software"), to deal
// * in the Software without restriction, including without limitation the rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

use crate::gameplay::Symbol;

use super::square_value::SquareValue;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Square {
    value: SquareValue,
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[allow(dead_code)]
impl Square {
    pub fn new() -> Square {
        Square {
            value: SquareValue::Empty,
        }
    }

    fn is_empty(&self) -> bool {
        self.value == SquareValue::Empty
    }

    fn is_x(&self) -> bool {
        self.value == SquareValue::X
    }

    fn is_o(&self) -> bool {
        self.value == SquareValue::O
    }

    fn get_value(&self) -> SquareValue {
        self.value
    }

    pub fn set_value(&mut self, value: Symbol) {
        match value {
            Symbol::X => self.value = SquareValue::X,
            Symbol::O => self.value = SquareValue::O,
        }
    }

    fn set_x(&mut self) {
        self.value = SquareValue::X;
    }

    fn set_o(&mut self) {
        self.value = SquareValue::O;
    }

    fn set_empty(&mut self) {
        self.value = SquareValue::Empty;
    }
}

impl Default for Square {
    fn default() -> Self {
        Square::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let square = Square::new();
        assert_eq!(square.value, SquareValue::Empty);
    }

    #[test]
    fn test_is_empty() {
        let square = Square::new();
        assert!(square.is_empty());
    }

    #[test]
    fn test_is_x() {
        let mut square = Square::new();
        square.set_x();
        assert!(square.is_x());
    }

    #[test]
    fn test_is_o() {
        let mut square = Square::new();
        square.set_o();
        assert!(square.is_o());
    }

    #[test]
    fn test_get_value() {
        let mut square = Square::new();
        square.set_x();
        assert_eq!(square.get_value(), SquareValue::X);
    }

    #[test]
    fn test_set_value() {
        let mut square = Square::new();
        square.set_value(Symbol::X);
        assert_eq!(square.get_value(), SquareValue::X);
    }

    #[test]
    fn test_set_x() {
        let mut square = Square::new();
        square.set_x();
        assert_eq!(square.get_value(), SquareValue::X);
    }

    #[test]
    fn test_set_o() {
        let mut square = Square::new();
        square.set_o();
        assert_eq!(square.get_value(), SquareValue::O);
    }

    #[test]
    fn test_set_empty() {
        let mut square = Square::new();
        square.set_x();
        square.set_empty();
        assert_eq!(square.get_value(), SquareValue::Empty);
    }

    #[test]
    fn test_display() {
        let mut square = Square::new();
        square.set_x();
        assert_eq!(format!("{}", square), " X ");
    }

    #[test]
    fn test_default() {
        let square = Square::default();
        assert_eq!(square.value, SquareValue::Empty);
    }

    #[test]
    fn test_eq() {
        let square1 = Square::new();
        let square2 = Square::new();
        assert_eq!(square1, square2);
    }
}
