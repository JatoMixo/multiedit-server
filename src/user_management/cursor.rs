/// The position of a cursor in the screen, to show it in the rest of the clients
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Cursor {
    column: u32,
    row: u32,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            column: 0,
            row: 0,
        }
    }

    pub fn move_to(&mut self, cursor: Cursor) {
        self.column = cursor.column;
        self.row = cursor.row;
    }
}
