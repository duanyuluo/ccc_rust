// Table utilites
//
// Table border chars:
//  Legend           TablePos       Table Group            Group Fmt Pattern
// --------------------------------------------------------------------------------
// ┌───┬───┐  <-  LT TT TM TT RT  <- HeadTop     ->  LT TT [TT] TT [TM TT [TT] TT] RT
// │_x_│_x_│  <-  LL    IV    RR  <- HeadLine    ->  LL CH [xx] __ [IV CH [xx] __] RR
// ├───┼───┤  <-  LM IH IC IH RM  <- HeadBottom  ->  LM IH [IH] IH [IC IH [IH] IH] RM
// │█x_│_x_│  <-  LL    IV    RR  <- BodyLine    ->  LL CH [xx] __ [IV CH [xx] __] RR
// └───┴───┘  <-  LB BB BM BB RB  <- BodyBottom  ->  LB BB [BB] BB [BM BB [BB] BB] RB
//  ^ ^  ^                        <- Corner          ██ ██████████  ██ ██████████  ██
//  │ │  │                        <- SepBorder       │  │Cell Fmt│  │  │Cell Fmt│   │
//  │ │  │                        <- Border          │              │               │
//  │ │  │                        <- Inner           First        Middle         Last
//  │ │  └───────── Cell Text
//  │ └──────────── Cell Margin
//  └────────────── CH (InnerCellHighlight)
//
extern crate strfmt;
use std::{collections::HashMap, usize};
use strfmt::Format;

/// Table Border Char List
pub const TABLE_BORDER_CHARS: [&'static str; 17] = [
    "┌", "┐", "└", "┘", // LT, RT, LB, RB <- Corner
    "├", "┤", "┬", "┴", // LM, RM, TM, BM <- Seperator Border
    "│", "│", "─", "─", // LL, RR, TT, BB <- Border
    "│", "─", "┼", "█", // VV, HH, MM, CC <- SepInner
    " ",
];

/// Cell Side Margin Width
const TABLE_CELL_MARGIN: u8 = 1;

/// Table Border Char Position
/// Corner...table 4 corners
/// SepBdr...table 4 borders for seperate column or row
/// Border...table 4 borders
/// Inner ...table column and row seperater, cells cross and cell highlight tip
///
#[derive(Clone, Copy)]
pub enum TablePos {
    CornerLeftTop = 0, // Corner
    CornerRightTop,
    CornerLeftBottom,
    CornerRightBottom,
    SepBdrLeft, // SepBorder
    SepBdrRight,
    SepBdrTop,
    SepBdrBottom,
    BorderLeft, // Border
    BorderRight,
    BorderTop,
    BorderBottom,
    InnerVert, // Inner
    InnerHori,
    InnerCross,
    InnerCellHighlight,
    InnerMargin,
}

const LT: usize = TablePos::CornerLeftTop as usize;
const RT: usize = TablePos::CornerRightTop as usize;
const LB: usize = TablePos::CornerLeftBottom as usize;
const RB: usize = TablePos::CornerRightBottom as usize;
const LM: usize = TablePos::SepBdrLeft as usize;
const RM: usize = TablePos::SepBdrRight as usize;
const TM: usize = TablePos::SepBdrTop as usize;
const BM: usize = TablePos::SepBdrBottom as usize;
const LL: usize = TablePos::BorderLeft as usize;
const RR: usize = TablePos::BorderRight as usize;
const TT: usize = TablePos::BorderTop as usize;
const BB: usize = TablePos::BorderBottom as usize;
const IV: usize = TablePos::InnerVert as usize;
const IH: usize = TablePos::InnerHori as usize;
const IC: usize = TablePos::InnerCross as usize;
const CH: usize = TablePos::InnerCellHighlight as usize;
const CM: usize = TablePos::InnerMargin as usize;

macro_rules! bdr {
    ($item:ident) => {
        TABLE_BORDER_CHARS[$item]
    };
}

/// Cell format pattern (l-r border l-r margin)
///                LineNum         First       Middle          Last
/// HeadTop      __ __ __ __   LT TM TT TT   TM TM TT TT    TM RT TT TT
/// HeadLine     __ __ __ __   LL IV __ __   IV IV __ __    IV RR __ __  
/// HeadBottom   __ __ __ __   LM IC IH IH   IC IC IH IH    IC RM IH IH
/// BodyLine     __ __ __ __   LL IV CH CH   IV IV CH CH    IV RR CH CH
/// BodyBottom   __ __ __ __   LB BM BB BB   BM BM BB BB    BM RB BB BB
///
/// CELL_PATTERNS[PosGroup][ColumnType]: Vec<[(&str, &str, &str, &str); 4]>
// const CELL_PATTERNS: Vec<[(&str, &str, &str, &str); 4]> = vec![
fn cell_patterns(
    pos_group: TablePosGroup,
    column_type: ColumnType,
) -> (&'static str, &'static str, &'static str, &'static str) {
    let patterns = match (pos_group, column_type) {
        (TablePosGroup::HeadTop, ColumnType::LineNum) => ("", "", "", ""), // LineNum
        (TablePosGroup::HeadTop, ColumnType::First) => (bdr!(LT), bdr!(TM), bdr!(TT), bdr!(TT)), // First
        (TablePosGroup::HeadTop, ColumnType::Middle) => (bdr!(TM), bdr!(TM), bdr!(TT), bdr!(TT)), // Middle
        (TablePosGroup::HeadTop, ColumnType::Last) => (bdr!(TM), bdr!(RT), bdr!(TT), bdr!(TT)), // Last
        (TablePosGroup::HeadLine, ColumnType::LineNum) => ("", "", "", ""), // LineNum
        (TablePosGroup::HeadLine, ColumnType::First) => (bdr!(LL), bdr!(IV), bdr!(CM), bdr!(CM)), // First
        (TablePosGroup::HeadLine, ColumnType::Middle) => (bdr!(IV), bdr!(IV), bdr!(CM), bdr!(CM)), // Middle
        (TablePosGroup::HeadLine, ColumnType::Last) => (bdr!(IV), bdr!(RR), bdr!(CM), bdr!(CM)), // Last
        (TablePosGroup::HeadBottom, ColumnType::LineNum) => ("", "", "", ""), // LineNum
        (TablePosGroup::HeadBottom, ColumnType::First) => (bdr!(LM), bdr!(IC), bdr!(IH), bdr!(IH)), // First
        (TablePosGroup::HeadBottom, ColumnType::Middle) => (bdr!(IC), bdr!(IC), bdr!(IH), bdr!(IH)), // Middle
        (TablePosGroup::HeadBottom, ColumnType::Last) => (bdr!(IC), bdr!(RM), bdr!(IH), bdr!(IH)), // Last
        (TablePosGroup::BodyLine, ColumnType::LineNum) => ("", "", "", ""), // LineNum
        (TablePosGroup::BodyLine, ColumnType::First) => (bdr!(LL), bdr!(IV), bdr!(CH), bdr!(CH)), // First
        (TablePosGroup::BodyLine, ColumnType::Middle) => (bdr!(IV), bdr!(IV), bdr!(CH), bdr!(CH)), // Middle
        (TablePosGroup::BodyLine, ColumnType::Last) => (bdr!(IV), bdr!(RR), bdr!(CH), bdr!(CH)), // Last
        (TablePosGroup::BodyBottom, ColumnType::LineNum) => ("", "", "", ""), // LineNum
        (TablePosGroup::BodyBottom, ColumnType::First) => (bdr!(LB), bdr!(BM), bdr!(BB), bdr!(BB)), // First
        (TablePosGroup::BodyBottom, ColumnType::Middle) => (bdr!(BM), bdr!(BM), bdr!(BB), bdr!(BB)), // Middle
        (TablePosGroup::BodyBottom, ColumnType::Last) => (bdr!(BM), bdr!(RB), bdr!(BB), bdr!(BB)), // Last
        (_, _) => ("", "", "", ""),
    };
    patterns
}

/// Table Border Char Group
/// Type Group: Corner, SepBorder, Border, Inner
/// Line Group: HeadTop, HeadLine, HeadBottom, BodyLine, BodyBottom
#[derive(Clone, Copy)]
pub enum TablePosGroup {
    Corner = 0,
    SepBorder,
    Border,
    Inner,
    HeadTop,
    HeadLine,
    HeadBottom,
    BodyLine,
    BodyBottom,
}

/// Table Column Alignment
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColumnAlign {
    #[default]
    Left = 0,
    Center,
    Right,
}

/// Table Column Type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ColumnType {
    LineNum = 0, // todo: out of table
    First,
    #[default]
    Middle,
    Last,
}

/// Table column configeration
/// width: column content width
/// column-width: width + 2 * TABLE_CELL_MARGIN
/// table-width: sum(column-width) * count(columns) + 1
pub struct TableColumnConfig {
    pub width: usize,
    pub title: String,
    pub align: ColumnAlign,
    pub ty: ColumnType,
}

/// Table Whole Config
pub struct TableConfig {
    pub columns: Vec<TableColumnConfig>,
    pub has_linenum: bool,
}

impl TableConfig {
    /// start build table config and reset all columns.
    pub fn start_build() -> TableConfig {
        TableConfig {
            columns: Vec::new(),
            has_linenum: false,
        }
    }

    /// new column config
    pub fn new_column(&mut self, width: usize, title: &str, align: ColumnAlign) {
        let column_cfg = TableColumnConfig {
            width,
            title: title.to_string(),
            align,
            ty: {
                match self.columns.len() {
                    0 => ColumnType::First,
                    _ => ColumnType::Middle, // set default and then modify Last at build_done()
                }
            },
        };
        self.columns.push(column_cfg);
    }

    /// auto new column config with column_autocfg spec:
    ///   width: column_autocfg.length
    ///   title: column_autocfg.trimleft and trimright
    ///   align: "  title" -> ColumnAlign::Left
    ///          "title  " -> ColumnAlign::Right
    ///          " title " -> ColumnAlign::Center
    pub fn auto_column(&mut self, column_autocfg: &str) {
        let align = {
            if column_autocfg.starts_with(" ") && column_autocfg.ends_with(" ") {
                ColumnAlign::Center
            } else {
                if column_autocfg.starts_with(" ") {
                    ColumnAlign::Right
                } else {
                    ColumnAlign::Left
                }
            }
        };
        let widht = column_autocfg.len();
        let title = column_autocfg.trim();
        self.new_column(widht, title, align)
    }

    /// adjust full of table config
    /// 1) modify last column type
    /// 2) append line number column if needed
    pub fn build_done(&mut self, with_line_num: bool) {
        // adjust last column type
        let mut last_column = self.columns.pop().unwrap();
        last_column.ty = ColumnType::Last;
        self.columns.push(last_column);

        // append line number column
        self.has_linenum = with_line_num;
        if with_line_num {
            self.columns.insert(
                0,
                TableColumnConfig {
                    width: 2,
                    title: "#".to_string(),
                    align: ColumnAlign::Left,
                    ty: ColumnType::LineNum,
                },
            );
        }
    }
}

/// Table Format Tools
pub struct TableTool {
    pub tbl_cfg: TableConfig,
    pub rending_column: usize,
    pub rending_row: usize, // start at body line from 1
}

impl TableTool {
    /// attach table config for formatting
    pub fn attach(tbl_cfg: TableConfig) -> TableTool {
        TableTool {
            tbl_cfg,
            rending_column: 0,
            rending_row: 0, // set 1 when first render body line
        }
    }

    /// calculate table width
    pub fn table_width(&self) -> usize {
        let mut width = 1; // Table Left border
        for column in &self.tbl_cfg.columns {
            if column.ty == ColumnType::LineNum {
                // Line number column without border and margin
                width += column.width;
            } else {
                // Cell column with margin of both sides
                width += column.width + TABLE_CELL_MARGIN as usize * 2;
                // add right border or inner seperater
                width += 1;
            }
        }
        width
    }

    /// Get Table Border String.
    pub fn char_at_tblpos(pos: TablePos) -> &'static str {
        return TABLE_BORDER_CHARS[pos as usize];
    }

    /// Make cell format string for formatting
    /// note: format one row only, so cell_text must shorter than column width.
    ///       use fmt_tbl_line to format too long cell_text cross multi-rows.
    pub fn fmt_cell(&self, pos_grp: TablePosGroup, column_idx: usize, cell_txt: &str) -> String {
        // set column type by column index
        let col_pos = match column_idx {
            0 if self.tbl_cfg.has_linenum => ColumnType::LineNum,
            0 if !self.tbl_cfg.has_linenum => ColumnType::First,
            idx if idx < self.tbl_cfg.columns.len() - 1 => ColumnType::Middle,
            idx if idx == self.tbl_cfg.columns.len() - 1 => ColumnType::Last,
            _ => {
                assert!(false);
                ColumnType::Middle
            }
        };

        // choice cell format pattern item: left_border, right_border, left_margin, right_margin
        let (left_border, right_border, left_margin, right_margin) =
            cell_patterns(pos_grp, col_pos);

        // adjust cell text or border for column width
        // let column_cfg = &self.tbl_cfg.columns[column_idx];
        // let column_align = column_cfg.align;
        // let column_width = column_cfg.width;

        // format line
        format!("{left_border}{left_margin}{cell_txt}{right_margin}{right_border}")
    }

    pub fn fmt_line(&self, _grp: TablePosGroup) -> String {
        todo!()
    }

    pub fn fmt_by_group(_grp: TablePosGroup, columns: Vec<TableColumnConfig>) -> String {
        for (_col_idx, _column) in columns.iter().enumerate() {
            // let col_width = column.width + 2 * TABLE_CELL_MARGIN as usize;
        }
        let line_fmt_pattern = format!(
            "{}[v1]{}[v2]{}",
            TableTool::char_at_tblpos(TablePos::InnerCellHighlight),
            TableTool::char_at_tblpos(TablePos::BorderLeft),
            TableTool::char_at_tblpos(TablePos::InnerCross)
        )
        .replace("[", "{")
        .replace("]", "}");
        let mut vars: HashMap<String, &str> = HashMap::new();
        vars.insert("v1".to_string(), "a");
        vars.insert("v2".to_string(), "b");
        line_fmt_pattern.format(&vars).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn table_border_char() {
        assert_eq!(TableTool::char_at_tblpos(TablePos::InnerCellHighlight), "█");
        assert_eq!(TableTool::char_at_tblpos(TablePos::BorderLeft), "│");
        assert_eq!(TableTool::char_at_tblpos(TablePos::InnerCross), "┼");
    }

    #[test]
    fn table_config_build() {
        let mut tbl_cfg = TableConfig::start_build();
        tbl_cfg.new_column(10, "First Column", ColumnAlign::Left);
        tbl_cfg.auto_column("Left Column ");
        tbl_cfg.auto_column(" Middle Column ");
        tbl_cfg.auto_column("    Right Column");
        tbl_cfg.build_done(false);

        assert_eq!(tbl_cfg.columns.len(), 4);

        let col = tbl_cfg.columns.get(0).unwrap();
        assert_eq!(col.width, 10);
        assert_eq!(col.title, "First Column");
        assert_eq!(col.align, ColumnAlign::Left);
        assert_eq!(col.ty, ColumnType::First);

        let col = tbl_cfg.columns.get(1).unwrap();
        assert_eq!(col.width, 12);
        assert_eq!(col.title, "Left Column");
        assert_eq!(col.align, ColumnAlign::Left);
        assert_eq!(col.ty, ColumnType::Middle);

        let col = tbl_cfg.columns.get(2).unwrap();
        assert_eq!(col.width, 15);
        assert_eq!(col.title, "Middle Column");
        assert_eq!(col.align, ColumnAlign::Center);
        assert_eq!(col.ty, ColumnType::Middle);

        let col = tbl_cfg.columns.get(3).unwrap();
        assert_eq!(col.width, 16);
        assert_eq!(col.title, "Right Column");
        assert_eq!(col.align, ColumnAlign::Right);
        assert_eq!(col.ty, ColumnType::Last);
    }

    #[test]
    fn table_width() {
        let mut tbl_cfg = TableConfig::start_build();
        tbl_cfg.new_column(10, "First Column", ColumnAlign::Left);
        tbl_cfg.auto_column("Left Column ");
        tbl_cfg.build_done(false);

        let tbl_tool = TableTool::attach(tbl_cfg);
        assert_eq!(tbl_tool.table_width(), 29)
    }

    #[test]
    fn table_fmt_tool() {
        let mut tbl_cfg = TableConfig::start_build();
        tbl_cfg.new_column(10, "First Column", ColumnAlign::Left);
        tbl_cfg.auto_column("Left Column ");
        tbl_cfg.auto_column(" Middle Column ");
        tbl_cfg.auto_column("    Right Column");
        tbl_cfg.build_done(false);

        let tbl_tool = TableTool::attach(tbl_cfg);
    }
}
