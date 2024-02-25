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

use super::square::Square;

#[derive(Debug, PartialEq, Eq)]

pub struct Board {
    squares: [[Square; 3]; 3],
}

#[allow(dead_code)]
impl Board {
    pub fn new() -> Board {
        Board {
            squares: [[Square::new(); 3]; 3],
        }
    }

    fn get_row(&self, row: usize) -> [Square; 3] {
        [
            self.get_square(row, 0),
            self.get_square(row, 1),
            self.get_square(row, 2),
        ]
    }

    fn get_column(&self, column: usize) -> [Square; 3] {
        [
            self.get_square(0, column),
            self.get_square(1, column),
            self.get_square(2, column),
        ]
    }

    fn get_diagonal(&self, diagonal: usize) -> [Square; 3] {
        match diagonal {
            0 => [
                self.get_square(0, 0),
                self.get_square(1, 1),
                self.get_square(2, 2),
            ],
            1 => [
                self.get_square(0, 2),
                self.get_square(1, 1),
                self.get_square(2, 0),
            ],
            _ => panic!("Invalid diagonal"),
        }
    }

    pub fn get_square(&self, row: usize, column: usize) -> Square {
        self.squares[row][column]
    }

    pub fn print(&self) {
        for row in 0..3 {
            println!(
                "{}|{}|{}",
                self.get_square(row, 0),
                self.get_square(row, 1),
                self.get_square(row, 2)
            );
            if row < 2 {
                println!("-----------");
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        let board = Board::new();
        assert_eq!(
            board.get_row(0),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_row(1),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_row(2),
            [Square::new(), Square::new(), Square::new()]
        );
    }

    #[test]
    fn test_get_column() {
        let board = Board::new();
        assert_eq!(
            board.get_column(0),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_column(1),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_column(2),
            [Square::new(), Square::new(), Square::new()]
        );
    }

    #[test]
    fn test_get_diagonal() {
        let board = Board::new();
        assert_eq!(
            board.get_diagonal(0),
            [Square::new(), Square::new(), Square::new()]
        );
        assert_eq!(
            board.get_diagonal(1),
            [Square::new(), Square::new(), Square::new()]
        );
    }
}
