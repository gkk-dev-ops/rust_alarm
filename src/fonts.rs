use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Debug)]
pub struct Font {
    pub name: &'static str,
    glyphs: HashMap<char, Vec<&'static str>>,
    spacing: u16,
}

impl Font {
    pub fn render(&self, text: &str) -> Vec<String> {
        let height = self.glyphs.values().next().map_or(1, Vec::len);
        let mut lines = vec![String::new(); height];
        for (index, character) in text.chars().enumerate() {
            let glyph = self
                .glyphs
                .get(&character)
                .or_else(|| self.glyphs.get(&' '))
                .expect("font includes a space glyph");
            for (line, part) in lines.iter_mut().zip(glyph) {
                if index > 0 {
                    line.push_str(&" ".repeat(self.spacing as usize));
                }
                line.push_str(part);
            }
        }
        lines
    }

    fn fits(&self, text: &str, size: Size) -> bool {
        let rendered = self.render(text);
        rendered.len() <= size.height as usize
            && rendered
                .iter()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0)
                <= size.width as usize
    }
}

#[derive(Clone, Debug)]
pub struct FontCatalog {
    fonts: Vec<Font>,
}

impl Default for FontCatalog {
    fn default() -> Self {
        Self {
            fonts: vec![banner_font(), standard_font(), compact_font()],
        }
    }
}

impl FontCatalog {
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.fonts.iter().map(|font| font.name)
    }

    pub fn by_name(&self, name: &str) -> Option<&Font> {
        self.fonts
            .iter()
            .find(|font| font.name.eq_ignore_ascii_case(name))
    }

    pub fn largest_fit(&self, text: &str, size: Size) -> Option<&Font> {
        self.fonts.iter().find(|font| font.fits(text, size))
    }

    pub fn largest_fit_preferring(&self, preferred: &str, text: &str, size: Size) -> Option<&Font> {
        self.by_name(preferred)
            .filter(|font| font.fits(text, size))
            .or_else(|| self.largest_fit(text, size))
    }
}

fn compact_font() -> Font {
    Font {
        name: "compact",
        glyphs: chars_to_glyphs("0123456789: ", 1),
        spacing: 0,
    }
}

fn standard_font() -> Font {
    let rows = [
        [
            "┌─┐",
            " ┐ ",
            "┌─┐",
            "┌─┐",
            "┐ ┐",
            "┌─┐",
            "┌─┐",
            "┌─┐",
            "┌─┐",
            "┌─┐",
            " ",
            "   ",
        ],
        [
            "│ │",
            " │ ",
            "┌─┘",
            " ─┤",
            "└─┤",
            "└─┐",
            "├─┐",
            "  │",
            "├─┤",
            "└─┤",
            "·",
            "   ",
        ],
        [
            "└─┘",
            " ┴ ",
            "└──",
            "└─┘",
            "  ┴",
            "└─┘",
            "└─┘",
            "  ┴",
            "└─┘",
            "└─┘",
            "·",
            "   ",
        ],
    ];
    Font {
        name: "standard",
        glyphs: rows_to_glyphs(rows),
        spacing: 2,
    }
}

fn banner_font() -> Font {
    let rows = [
        [
            "███",
            " █ ",
            "███",
            "███",
            "█ █",
            "███",
            "███",
            "███",
            "███",
            "███",
            " ",
            "   ",
        ],
        [
            "█ █", "██ ", "  █", "  █", "█ █", "█  ", "█  ", "  █", "█ █", "█ █", "█", "   ",
        ],
        [
            "█ █",
            " █ ",
            "███",
            "███",
            "███",
            "███",
            "███",
            "  █",
            "███",
            "███",
            " ",
            "   ",
        ],
        [
            "█ █", " █ ", "█  ", "  █", "  █", "  █", "█ █", "  █", "█ █", "  █", "█", "   ",
        ],
        [
            "███",
            "███",
            "███",
            "███",
            "  █",
            "███",
            "███",
            "  █",
            "███",
            "███",
            " ",
            "   ",
        ],
    ];
    Font {
        name: "banner",
        glyphs: rows_to_glyphs(rows),
        spacing: 2,
    }
}

fn chars_to_glyphs(characters: &str, height: usize) -> HashMap<char, Vec<&'static str>> {
    characters
        .chars()
        .map(|character| {
            let leaked: &'static str = Box::leak(character.to_string().into_boxed_str());
            (character, vec![leaked; height])
        })
        .collect()
}

fn rows_to_glyphs<const H: usize, const W: usize>(
    rows: [[&'static str; W]; H],
) -> HashMap<char, Vec<&'static str>> {
    let characters: Vec<char> = "0123456789: ".chars().collect();
    characters
        .into_iter()
        .enumerate()
        .map(|(column, character)| {
            (
                character,
                rows.iter().map(|row| row[column]).collect::<Vec<_>>(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{FontCatalog, Size};

    #[test]
    fn chooses_largest_font_that_fits() {
        let catalog = FontCatalog::default();
        assert_eq!(
            catalog
                .largest_fit("01:30", Size::new(120, 30))
                .unwrap()
                .name,
            "banner"
        );
        assert_eq!(
            catalog.largest_fit("01:30", Size::new(20, 5)).unwrap().name,
            "compact"
        );
        assert!(catalog.largest_fit("01:30", Size::new(4, 1)).is_none());
    }
}
