use alloc::string::String;

use super::{super::DisplayImage, Animate, Frame};

pub struct ScrollingText {
    text: String,
    text_index: usize,
    column_index: Column,
    speed: u32,
    scrolled_on: bool,
}

enum Column {
    C0,
    C1,
    C2,
    C3,
    C4,
    B0,
    B1,
}

impl Column {
    fn columns_passed(&self) -> u32 {
        match self {
            Column::C0 => 0,
            Column::C1 => 1,
            Column::C2 => 2,
            Column::C3 => 3,
            Column::C4 => 4,
            Column::B0 => 5,
            Column::B1 => 6,
        }
    }
}

pub enum Letter {
    Wide([[bool; 5]; 5]),
    Medium([[bool; 4]; 5]),
    Thin([[bool; 3]; 5]),
}

impl Letter {
    fn column_width(&self) -> u8 {
        match self {
            Letter::Wide(_) => 5,
            Letter::Medium(_) => 4,
            Letter::Thin(_) => 3,
        }
    }

    fn get_column(&self, column: usize) -> Option<[bool; 5]> {
        match self {
            Letter::Wide(array) => {
                if column < 5 {
                    Some([
                        array[0][column],
                        array[1][column],
                        array[2][column],
                        array[3][column],
                        array[4][column],
                    ])
                } else {
                    None
                }
            }
            Letter::Medium(array) => {
                if column < 4 {
                    Some([
                        array[0][column],
                        array[1][column],
                        array[2][column],
                        array[3][column],
                        array[4][column],
                    ])
                } else {
                    None
                }
            }
            Letter::Thin(array) => {
                if column < 3 {
                    Some([
                        array[0][column],
                        array[1][column],
                        array[2][column],
                        array[3][column],
                        array[4][column],
                    ])
                } else {
                    None
                }
            }
        }
    }
}

impl From<[[bool; 5]; 5]> for Letter {
    fn from(other: [[bool; 5]; 5]) -> Self {
        Letter::Wide(other)
    }
}

impl From<[[bool; 3]; 5]> for Letter {
    fn from(other: [[bool; 3]; 5]) -> Self {
        Letter::Thin(other)
    }
}

impl From<Letter> for DisplayImage {
    fn from(other: Letter) -> Self {
        match other {
            Letter::Wide(letter) => DisplayImage(letter),
            Letter::Medium(letter) => DisplayImage([
                [
                    letter[0][0],
                    letter[0][1],
                    letter[0][2],
                    letter[0][3],
                    false,
                ],
                [
                    letter[1][0],
                    letter[1][1],
                    letter[1][2],
                    letter[1][3],
                    false,
                ],
                [
                    letter[2][0],
                    letter[2][1],
                    letter[2][2],
                    letter[2][3],
                    false,
                ],
                [
                    letter[3][0],
                    letter[3][1],
                    letter[3][2],
                    letter[3][3],
                    false,
                ],
                [
                    letter[4][0],
                    letter[4][1],
                    letter[4][2],
                    letter[4][3],
                    false,
                ],
            ]),
            Letter::Thin(letter) => DisplayImage([
                [false, letter[0][0], letter[0][1], letter[0][2], false],
                [false, letter[1][0], letter[1][1], letter[1][2], false],
                [false, letter[2][0], letter[2][1], letter[2][2], false],
                [false, letter[3][0], letter[3][1], letter[3][2], false],
                [false, letter[4][0], letter[4][1], letter[4][2], false],
            ]),
        }
    }
}

include!("letters.rs");

impl ScrollingText {
    pub fn new(text: String, speed: u32) -> Self {
        ScrollingText {
            text: text,
            text_index: 0,
            column_index: Column::C0,
            speed: speed,
            scrolled_on: false,
        }
    }

    fn char_to_image(letter: char) -> Letter {
        match letter {
            'A' => A_UPPER,
            'B' => B_UPPER,
            'C' => C_UPPER,
            'D' => D_UPPER,
            'E' => E_UPPER,
            'F' => F_UPPER,
            'G' => G_UPPER,
            'H' => H_UPPER,
            'I' => I_UPPER,
            'J' => J_UPPER,
            'K' => K_UPPER,
            'L' => L_UPPER,
            'M' => M_UPPER,
            'N' => N_UPPER,
            'O' => O_UPPER,
            'P' => P_UPPER,
            'Q' => Q_UPPER,
            'R' => R_UPPER,
            'S' => S_UPPER,
            'T' => T_UPPER,
            'U' => U_UPPER,
            'V' => V_UPPER,
            'W' => W_UPPER,
            'X' => X_UPPER,
            'Y' => Y_UPPER,
            'Z' => Z_UPPER,
            'a' => A_LOWER,
            'b' => B_LOWER,
            'c' => C_LOWER,
            'd' => D_LOWER,
            'e' => E_LOWER,
            'f' => F_LOWER,
            'g' => G_LOWER,
            'h' => H_LOWER,
            'i' => I_LOWER,
            'j' => J_LOWER,
            'k' => K_LOWER,
            'l' => L_LOWER,
            'm' => M_LOWER,
            'n' => N_LOWER,
            'o' => O_LOWER,
            'p' => P_LOWER,
            'q' => Q_LOWER,
            'r' => R_LOWER,
            's' => S_LOWER,
            't' => T_LOWER,
            'u' => U_LOWER,
            'v' => V_LOWER,
            'w' => W_LOWER,
            'x' => X_LOWER,
            'y' => Y_LOWER,
            'z' => Z_LOWER,
            '0' => O_UPPER,
            '1' => NUMBER1,
            '2' => NUMBER2,
            '3' => NUMBER3,
            '4' => NUMBER4,
            '5' => NUMBER5,
            '6' => NUMBER6,
            '7' => NUMBER7,
            '8' => NUMBER8,
            '9' => NUMBER9,
            '!' => EXCLAMATION,
            ',' => COMMA,
            '?' => QUESTION,
            ' ' => SPACE,
            '.' => PERIOD,
            _ => QUESTION,
        }
    }
}

impl Animate for ScrollingText {
    fn next_screen(&mut self) -> Option<Frame> {
        let (current, next) = if self.text_index == 0 && !self.scrolled_on {
            (
                SPACE,
                ScrollingText::char_to_image(self.text.as_bytes()[0] as char),
            )
        } else {
            if self.text_index + 1 >= self.text.len() {
                (
                    ScrollingText::char_to_image(self.text.as_bytes()[self.text.len() - 1] as char),
                    SPACE,
                )
            } else {
                (
                    ScrollingText::char_to_image(self.text.as_bytes()[self.text_index] as char),
                    ScrollingText::char_to_image(self.text.as_bytes()[self.text_index + 1] as char),
                )
            }
        };

        Some(match self.column_index {
            Column::C0 => {
                // first led column is first img column
                self.column_index = Column::C1;

                match current {
                    Letter::Wide(array) => Frame {
                        image: array.into(),
                        length: self.speed,
                    },
                    Letter::Medium(array) => Frame {
                        image: DisplayImage([
                            [array[0][0], array[0][1], array[0][2], array[0][3], false],
                            [array[1][0], array[1][1], array[1][2], array[1][3], false],
                            [array[2][0], array[2][1], array[2][2], array[2][3], false],
                            [array[3][0], array[3][1], array[3][2], array[3][3], false],
                            [array[4][0], array[4][1], array[4][2], array[4][3], false],
                        ]),
                        length: self.speed,
                    },
                    Letter::Thin(array) => Frame {
                        image: DisplayImage([
                            [array[0][0], array[0][1], array[0][2], false, false],
                            [array[1][0], array[1][1], array[1][2], false, false],
                            [array[2][0], array[2][1], array[2][2], false, false],
                            [array[3][0], array[3][1], array[3][2], false, false],
                            [array[4][0], array[4][1], array[4][2], false, false],
                        ]),
                        length: self.speed,
                    },
                }
            }
            Column::C1 => {
                // first led column is second img column
                self.column_index = Column::C2;

                match current {
                    Letter::Wide(array) => Frame {
                        image: DisplayImage([
                            [array[0][1], array[0][2], array[0][3], array[0][4], false],
                            [array[1][1], array[1][2], array[1][3], array[1][4], false],
                            [array[2][1], array[2][2], array[2][3], array[2][4], false],
                            [array[3][1], array[3][2], array[3][3], array[3][4], false],
                            [array[4][1], array[4][2], array[4][3], array[4][4], false],
                        ]),
                        length: self.speed,
                    },
                    Letter::Medium(array) => Frame {
                        image: DisplayImage([
                            [array[0][1], array[0][2], array[0][3], false, false],
                            [array[1][1], array[1][2], array[1][3], false, false],
                            [array[2][1], array[2][2], array[2][3], false, false],
                            [array[3][1], array[3][2], array[3][3], false, false],
                            [array[4][1], array[4][2], array[4][3], false, false],
                        ]),
                        length: self.speed,
                    },
                    Letter::Thin(array) => {
                        let last_column = match next.get_column(0) {
                            Some(column) => column,
                            None => panic!("Bad index on get_column()"),
                        };
                        Frame {
                            image: DisplayImage([
                                [array[0][1], array[0][2], false, false, last_column[0]],
                                [array[1][1], array[1][2], false, false, last_column[1]],
                                [array[2][1], array[2][2], false, false, last_column[2]],
                                [array[3][1], array[3][2], false, false, last_column[3]],
                                [array[4][1], array[4][2], false, false, last_column[4]],
                            ]),
                            length: self.speed,
                        }
                    }
                }
            }
            Column::C2 => {
                // first led column is third img column
                match current {
                    Letter::Wide(array) => {
                        self.column_index = Column::C3;
                        Frame {
                            image: DisplayImage([
                                [array[0][2], array[0][3], array[0][4], false, false],
                                [array[1][2], array[1][3], array[1][4], false, false],
                                [array[2][2], array[2][3], array[2][4], false, false],
                                [array[3][2], array[3][3], array[3][4], false, false],
                                [array[4][2], array[4][3], array[4][4], false, false],
                            ]),
                            length: self.speed,
                        }
                    }
                    Letter::Medium(array) => {
                        self.column_index = Column::C3;
                        let last_column = match next.get_column(0) {
                            Some(column) => column,
                            None => panic!("Bad index on get_column()"),
                        };
                        Frame {
                            image: DisplayImage([
                                [array[0][2], array[0][3], false, false, last_column[0]],
                                [array[1][2], array[1][3], false, false, last_column[1]],
                                [array[2][2], array[2][3], false, false, last_column[2]],
                                [array[3][2], array[3][3], false, false, last_column[3]],
                                [array[4][2], array[4][3], false, false, last_column[4]],
                            ]),
                            length: self.speed,
                        }
                    }
                    Letter::Thin(array) => {
                        self.column_index = Column::B0;
                        let second_last_column = match next.get_column(0) {
                            Some(column) => column,
                            None => panic!("Bad index on get_column()"),
                        };
                        let last_column = match next.get_column(1) {
                            Some(column) => column,
                            None => panic!("Bad index on get_column()"),
                        };
                        Frame {
                            image: DisplayImage([
                                [
                                    array[0][2],
                                    false,
                                    false,
                                    second_last_column[0],
                                    last_column[0],
                                ],
                                [
                                    array[1][2],
                                    false,
                                    false,
                                    second_last_column[1],
                                    last_column[1],
                                ],
                                [
                                    array[2][2],
                                    false,
                                    false,
                                    second_last_column[2],
                                    last_column[2],
                                ],
                                [
                                    array[3][2],
                                    false,
                                    false,
                                    second_last_column[3],
                                    last_column[3],
                                ],
                                [
                                    array[4][2],
                                    false,
                                    false,
                                    second_last_column[4],
                                    last_column[4],
                                ],
                            ]),
                            length: self.speed,
                        }
                    }
                }
            }
            Column::C3 => {
                // first led column is fourth img column
                match current {
                    Letter::Wide(array) => {
                        self.column_index = Column::C4;
                        let last_column = match next.get_column(0) {
                            Some(column) => column,
                            None => panic!("Bad index on get_column()"),
                        };
                        Frame {
                            image: DisplayImage([
                                [array[0][3], array[0][4], false, false, last_column[0]],
                                [array[1][3], array[1][4], false, false, last_column[1]],
                                [array[2][3], array[2][4], false, false, last_column[2]],
                                [array[3][3], array[3][4], false, false, last_column[3]],
                                [array[4][3], array[4][4], false, false, last_column[4]],
                            ]),
                            length: self.speed,
                        }
                    }
                    Letter::Medium(array) => {
                        self.column_index = Column::B0;
                        let second_last_column = match next.get_column(0) {
                            Some(column) => column,
                            None => panic!("Bad index on get_column()"),
                        };
                        let last_column = match next.get_column(1) {
                            Some(column) => column,
                            None => panic!("Bad index on get_column()"),
                        };
                        Frame {
                            image: DisplayImage([
                                [
                                    array[0][3],
                                    false,
                                    false,
                                    second_last_column[0],
                                    last_column[0],
                                ],
                                [
                                    array[1][3],
                                    false,
                                    false,
                                    second_last_column[1],
                                    last_column[1],
                                ],
                                [
                                    array[2][3],
                                    false,
                                    false,
                                    second_last_column[2],
                                    last_column[2],
                                ],
                                [
                                    array[3][3],
                                    false,
                                    false,
                                    second_last_column[3],
                                    last_column[3],
                                ],
                                [
                                    array[4][3],
                                    false,
                                    false,
                                    second_last_column[4],
                                    last_column[4],
                                ],
                            ]),
                            length: self.speed,
                        }
                    }
                    _ => panic!("Out of bounds column"),
                }
            }
            Column::C4 => {
                // first led column is fifth img column
                match current {
                    Letter::Wide(array) => {
                        self.column_index = Column::B0;
                        let second_last_column = match next.get_column(0) {
                            Some(column) => column,
                            None => panic!("Bad index on get_column()"),
                        };
                        let last_column = match next.get_column(1) {
                            Some(column) => column,
                            None => panic!("Bad index on get_column()"),
                        };
                        Frame {
                            image: DisplayImage([
                                [
                                    array[0][4],
                                    false,
                                    false,
                                    second_last_column[0],
                                    last_column[0],
                                ],
                                [
                                    array[1][4],
                                    false,
                                    false,
                                    second_last_column[1],
                                    last_column[1],
                                ],
                                [
                                    array[2][4],
                                    false,
                                    false,
                                    second_last_column[2],
                                    last_column[2],
                                ],
                                [
                                    array[3][4],
                                    false,
                                    false,
                                    second_last_column[3],
                                    last_column[3],
                                ],
                                [
                                    array[4][4],
                                    false,
                                    false,
                                    second_last_column[4],
                                    last_column[4],
                                ],
                            ]),
                            length: self.speed,
                        }
                    }
                    _ => panic!("Out of bounds column"),
                }
            }
            Column::B0 => {
                // currently in buffer 1
                self.column_index = Column::B1;
                let third_last_column = match next.get_column(0) {
                    Some(column) => column,
                    None => panic!("Bad index on get_column()"),
                };
                let second_last_column = match next.get_column(1) {
                    Some(column) => column,
                    None => panic!("Bad index on get_column()"),
                };
                let last_column = match next.get_column(2) {
                    Some(column) => column,
                    None => panic!("Bad index on get_column()"),
                };

                Frame {
                    image: DisplayImage([
                        [
                            false,
                            false,
                            third_last_column[0],
                            second_last_column[0],
                            last_column[0],
                        ],
                        [
                            false,
                            false,
                            third_last_column[1],
                            second_last_column[1],
                            last_column[1],
                        ],
                        [
                            false,
                            false,
                            third_last_column[2],
                            second_last_column[2],
                            last_column[2],
                        ],
                        [
                            false,
                            false,
                            third_last_column[3],
                            second_last_column[3],
                            last_column[3],
                        ],
                        [
                            false,
                            false,
                            third_last_column[4],
                            second_last_column[4],
                            last_column[4],
                        ],
                    ]),
                    length: self.speed,
                }
            }
            Column::B1 => {
                // currently in buffer 2
                self.column_index = Column::C0;
                if !self.scrolled_on {
                    self.scrolled_on = true;
                } else {
                    self.text_index = self.text_index + 1;
                }
                if self.text_index >= self.text.len() {
                    return None;
                }
                let fourth__last_column = match next.get_column(0) {
                    Some(column) => column,
                    None => panic!("Bad index on get_column()"),
                };
                let third_last_column = match next.get_column(1) {
                    Some(column) => column,
                    None => panic!("Bad index on get_column()"),
                };
                let second_last_column = match next.get_column(2) {
                    Some(column) => column,
                    None => panic!("Bad index on get_column()"),
                };
                let last_column = match next.get_column(3) {
                    Some(column) => column,
                    None => [false, false, false, false, false],
                };

                Frame {
                    image: DisplayImage([
                        [
                            false,
                            fourth__last_column[0],
                            third_last_column[0],
                            second_last_column[0],
                            last_column[0],
                        ],
                        [
                            false,
                            fourth__last_column[1],
                            third_last_column[1],
                            second_last_column[1],
                            last_column[1],
                        ],
                        [
                            false,
                            fourth__last_column[2],
                            third_last_column[2],
                            second_last_column[2],
                            last_column[2],
                        ],
                        [
                            false,
                            fourth__last_column[3],
                            third_last_column[3],
                            second_last_column[3],
                            last_column[3],
                        ],
                        [
                            false,
                            fourth__last_column[4],
                            third_last_column[4],
                            second_last_column[4],
                            last_column[4],
                        ],
                    ]),
                    length: self.speed,
                }
            }
        })
    }

    fn frames(&self) -> u32 {
        self.text
            .chars()
            .map(|c| ScrollingText::char_to_image(c))
            .map(|img| (img.column_width() as u32) + 2)
            .sum::<u32>() + 5
    }
}
