use super::image::DisplayImage;

pub const CHECKERBOARD: DisplayImage = DisplayImage([
    [true, false, true, false, true],
    [false, true, false, true, false],
    [true, false, true, false, true],
    [false, true, false, true, false],
    [true, false, true, false, true],
]);

pub const SMILEY: DisplayImage = DisplayImage([
    [false, true, false, true, false],
    [false, false, false, false, false],
    [true, false, false, false, true],
    [false, true, true, true, false],
    [false, false, false, false, false],
]);

pub const FROWNY: DisplayImage = DisplayImage([
    [false, true, false, true, false],
    [false, false, false, false, false],
    [false, true, true, true, false],
    [true, false, false, false, true],
    [false, false, false, false, false],
]);

pub const BLANK: DisplayImage = DisplayImage([
    [false, false, false, false, false],
    [false, false, false, false, false],
    [false, false, false, false, false],
    [false, false, false, false, false],
    [false, false, false, false, false],
]);
