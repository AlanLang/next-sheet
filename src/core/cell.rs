/**
 * 单元格
 */
#[derive(Debug)]
pub struct Cell {
    pub value: String,
    pub cell_type: CellType,
    pub style: CellStyle,
}

#[derive(Debug)]
pub enum CellType {
    Text,
}

#[derive(Debug)]
pub struct CellStyle {
    pub color: String,
}