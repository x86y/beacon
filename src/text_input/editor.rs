use phf::phf_map;

use crate::text_input::{Cursor, Value};
static KS: phf::Map<char, char> = phf_map! {
  '`'=>'˜',
  '1'=>'˘',
  '2'=>'¨',
  '3'=>'⁼',
  '4'=>'⌜',
  '5'=>'´',
  '6'=>'˝',
  '7'=>'7',
  '8'=>'∞',
  '9'=>'¯',
  '0'=>'•',
  '-'=>'÷',
  '='=>'×',
  '~'=>'¬',
  '!'=>'⎉',
  '@'=>'⚇',
  '#'=>'⍟',
  '$'=>'◶',
  '%'=>'⊘',
  '^'=>'⎊',
  '&'=>'⍎',
  '*'=>'⍕',
  '('=>'⟨',
  ')'=>'⟩',
  '_'=>'√',
  '+'=>'⋆',
  'q'=>'⌽',
  'w'=>'𝕨',
  'e'=>'∊',
  'r'=>'↑',
  't'=>'∧',
  'y'=>'y',
  'u'=>'⊔',
  'i'=>'⊏',
  'o'=>'⊐',
  'p'=>'π',
  '['=>'←',
  ']'=>'→',
  'Q'=>'↙',
  'W'=>'𝕎',
  'E'=>'⍷',
  'R'=>'𝕣',
  'T'=>'⍋',
  'Y'=>'Y',
  'U'=>'U',
  'I'=>'⊑',
  'O'=>'⊒',
  'P'=>'⍳',
  '{'=>'⊣',
  '}'=>'⊢',
  'a'=>'⍉',
  's'=>'𝕤',
  'd'=>'↕',
  'f'=>'𝕗',
  'g'=>'𝕘',
  'h'=>'⊸',
  'j'=>'∘',
  'k'=>'○',
  'l'=>'⟜',
  ';'=>'⋄',
  '\''=>'↩',
  'A'=>'↖',
  'S'=>'𝕊',
  'D'=>'D',
  'F'=>'𝔽',
  'G'=>'𝔾',
  'H'=>'«',
  'J'=>'J',
  'K'=>'⌾',
  'L'=>'»',
  ':'=>'·',
  '|'=>'|',
  'z'=>'⥊',
  'x'=>'𝕩',
  'c'=>'↓',
  'v'=>'∨',
  'b'=>'⌊',
  'n'=>'n',
  'm'=>'≡',
  ','=>'∾',
  '.'=>'≍',
  '/'=>'≠',
  'Z'=>'⋈',
  'X'=>'𝕏',
  'C'=>'C',
  'V'=>'⍒',
  'B'=>'⌈',
  'N'=>'N',
  'M'=>'≢',
  '<'=>'≤',
  '>'=>'≥',
  '?'=>'⇐',
  ' '=>'‿'
};

pub struct Editor<'a> {
    value: &'a mut Value,
    cursor: &'a mut Cursor,
}

impl<'a> Editor<'a> {
    pub fn new(value: &'a mut Value, cursor: &'a mut Cursor) -> Editor<'a> {
        Editor { value, cursor }
    }

    pub fn contents(&self) -> String {
        self.value.to_string()
    }

    pub fn insert(&mut self, character: char, locked: bool) {
        if let Some((left, right)) = self.cursor.selection(self.value) {
            self.cursor.move_left(self.value);
            self.value.remove_many(left, right);
        }

        if locked && character != '`' {
            self.value
                .insert(self.cursor.end(self.value), KS[&character]);
        } else {
            self.value.insert(self.cursor.end(self.value), character);
        }
        self.cursor.move_right(self.value);
    }

    pub fn paste(&mut self, content: Value) {
        let length = content.len();
        if let Some((left, right)) = self.cursor.selection(self.value) {
            self.cursor.move_left(self.value);
            self.value.remove_many(left, right);
        }

        self.value.insert_many(self.cursor.end(self.value), content);

        self.cursor.move_right_by_amount(self.value, length);
    }

    pub fn backspace(&mut self) {
        match self.cursor.selection(self.value) {
            Some((start, end)) => {
                self.cursor.move_left(self.value);
                self.value.remove_many(start, end);
            }
            None => {
                let start = self.cursor.start(self.value);

                if start > 0 {
                    self.cursor.move_left(self.value);
                    self.value.remove(start - 1);
                }
            }
        }
    }

    pub fn delete(&mut self) {
        match self.cursor.selection(self.value) {
            Some(_) => {
                self.backspace();
            }
            None => {
                let end = self.cursor.end(self.value);

                if end < self.value.len() {
                    self.value.remove(end);
                }
            }
        }
    }
}
