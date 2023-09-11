use core::fmt::Debug;
use std::process;
use std::io::prelude::*;

use console::{Term, Key, style};
use derive_more::{Deref, DerefMut, Display};
use itertools::Itertools;
use rand::{Rng, rngs::ThreadRng};

use jugo::{Puzzle, Piece, NdArrayPuzzle};

#[derive(Deref, DerefMut, Display)]
#[display(fmt = "{}", inner)]
struct PuzzleBox<T: Piece, R: Rng> {
    #[deref]
    #[deref_mut]
    inner: NdArrayPuzzle<T>,
    rng: R,
}
impl<T: Piece + Debug, R: Rng> core::fmt::Debug for PuzzleBox<T, R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PuzzleBox({:?})", self.inner)
    }
}
impl<T: Piece, R: Rng> PuzzleBox<T, R> {
    pub fn new_with_rng(mut rng: R, shape: (usize, usize)) -> Self {
        Self {
            inner: NdArrayPuzzle::random_with_rng(&mut rng, shape).unwrap(),
            rng,
        }
    }
    pub fn reset(&mut self) {
        let shape = self.inner.shape();
        self.inner = NdArrayPuzzle::random_with_rng(&mut self.rng, shape).unwrap();
    }
}
impl<T: Piece> PuzzleBox<T, ThreadRng> {
    pub fn new(size: (usize, usize)) -> Self {
        Self::new_with_rng(rand::thread_rng(), size)
    }
}

fn key_to_index(key: char) -> Option<(usize, usize)> {
    Some(match key {
        '4' => (0, 0),
        '5' => (1, 0),
        '6' => (2, 0),
        '7' => (3, 0),
        'r' => (0, 1),
        't' => (1, 1),
        'y' => (2, 1),
        'u' => (3, 1),
        'f' => (0, 2),
        'g' => (1, 2),
        'h' => (2, 2),
        'j' => (3, 2),
        'v' => (0, 3),
        'b' => (1, 3),
        'n' => (2, 3),
        'm' => (3, 3),
        _ => return None,
    })
}

fn main() {
    let mut terminal = Term::stdout();
    let mut puzzle = PuzzleBox::<u8, _>::new((4, 4));
    let mut history = Vec::with_capacity(80);

    terminal.clear_screen().unwrap();
    writeln!(terminal, "{puzzle}").unwrap();
    terminal.flush().unwrap();

    macro_rules! slide_towards {
        ($direction: ident) => {
            puzzle.slide_towards($direction, 1)
                .or(Some(0))
                .map(|d| (match $direction {
                    Up => '↑',
                    Down => '↓',
                    Left => '←',
                    Right => '→',
                }, d))
        }
    }

    use jugo::Direction::*;
    loop {
        let event = terminal.read_key().unwrap();
        let moved = match event {
            Key::Char('q') => process::exit(0),
            Key::Char(' ') => {
                puzzle.reset();
                history.clear();
                None
            }
            Key::ArrowUp => slide_towards!(Up),
            Key::ArrowDown => slide_towards!(Down),
            Key::ArrowLeft => slide_towards!(Left),
            Key::ArrowRight => slide_towards!(Right),
            Key::Char(c) => key_to_index(c)
                .and_then(|idx| puzzle.slide_from(idx)
                    .or(Some(0))
                    .map(|d| (c, d))
                ),
            _ => None
        };

        if let Some(a) = moved {
            history.push(a);
        }

        terminal.clear_screen().unwrap();
        writeln!(terminal, "{puzzle}").unwrap();

        if history.is_empty() {
            continue;
        }

        let history = history
            .iter()
            .map(|(c, d)| if *d == 0 {
                style(c).bright().black()
            } else {
                style(c)
            })
            .join("");

        println!("{history}");
    }
}
