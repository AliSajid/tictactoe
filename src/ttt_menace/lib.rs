// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2022
// *
// *
// * This project is dual-licensed under the MIT and Apache licenses.
// *
// *
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
// *
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
// *
// *

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(dead_code)]
enum SquareValue {
    X,
    O,
    Empty,
}

impl std::fmt::Display for SquareValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SquareValue::X => write!(f, " X "),
            SquareValue::O => write!(f, " O "),
            SquareValue::Empty => write!(f, "   "),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Square {
    value: SquareValue,
}

pub struct Board {
    board: [Square; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [
                Square {
                    value: SquareValue::Empty,
                },
                Square {
                    value: SquareValue::Empty,
                },
                Square {
                    value: SquareValue::Empty,
                },
                Square {
                    value: SquareValue::Empty,
                },
                Square {
                    value: SquareValue::Empty,
                },
                Square {
                    value: SquareValue::Empty,
                },
                Square {
                    value: SquareValue::Empty,
                },
                Square {
                    value: SquareValue::Empty,
                },
                Square {
                    value: SquareValue::Empty,
                },
            ],
        }
    }

    pub fn print(&self) {
        println!(
            "{}|{}|{}",
            self.board[0].value, self.board[1].value, self.board[2].value
        );
        println!("-----------");
        println!(
            "{}|{}|{}",
            self.board[3].value, self.board[4].value, self.board[5].value
        );
        println!("-----------");
        println!(
            "{}|{}|{}",
            self.board[6].value, self.board[7].value, self.board[8].value
        );
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
