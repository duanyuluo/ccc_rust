// Table utilites
//
// Table border chars:
//  Legend           TablePos       Table Group            Group Fmt Pattern
// --------------------------------------------------------------------------------
// ┌───┬───┐  <-  LT TT TM TT RT  <- HeadTop     ->  LT TT [TT] TT [TM TT [TT] TT] RT
// │_x_│_x_│  <-  LL    IV    RR  <- HeadLine    ->  LL CH [xx] __ [IV CH [xx] __] RR
// ├───┼───┤  <-  LM IH IC IH RM  <- BodySep  ->  LM IH [IH] IH [IC IH [IH] IH] RM
// │█x_│_x_│  <-  LL    IV    RR  <- BodyLine    ->  LL CH [xx] __ [IV CH [xx] __] RR
// └───┴───┘  <-  LB BB BM BB RB  <- BodyBtm  ->  LB BB [BB] BB [BM BB [BB] BB] RB
//  ^ ^  ^                        <- Corner          ██ ██████████  ██ ██████████  ██
//  │ │  │                        <- SepBorder       │  │Cell Fmt│  │  │Cell Fmt│   │
//  │ │  │                        <- Border          │              │               │
//  │ │  │                        <- Inner           First        Middle         Last
//  │ │  └───────── Cell Text
//  │ └──────────── Cell Margin
//  └────────────── CH (InnerMark)
//

use std::cmp;

/// Table Border Char List
pub const TABLE_BORDER_CHARS: [&'static str; 17] = [
    "┌", "┐", "└", "┘", // LT, RT, LB, RB <- Corner
    "├", "┤", "┬", "┴", // LM, RM, TM, BM <- Seperator Border
    "│", "│", "─", "─", // LL, RR, TT, BB <- Border
    "│", "─", "┼", "█", // VV, HH, MM, CH <- SepInner
    " ", // CM
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
    InnerMark,
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
const CH: usize = TablePos::InnerMark as usize;
const CM: usize = TablePos::InnerMargin as usize;

macro_rules! bdr {
    ($item:ident) => {
        TABLE_BORDER_CHARS[$item]
    };
}

/// Cell format pattern (left border, right border, left margin, right margin)
///                LineNum         First       Middle          Last
/// HeadTop      __ __ CM CM   LT TM TT TT   TM TM TT TT    TM RT TT TT
/// HeadLine     __ __ CM CM   LL IV CM CM   IV IV CM CM    IV RR CM CM  
/// BodySep      __ __ CM CM   LM IC IH IH   IC IC IH IH    IC RM IH IH
/// BodyLine     __ __ CM CM   LL IV CH CM   IV IV CH CM    IV RR CH CM
/// BodyBtm      __ __ CM CM   LB BM BB BB   BM BM BB BB    BM RB BB BB
///
fn cell_patterns(
    pos_group: TblGrp,
    column_type: ColumnType,
) -> (&'static str, &'static str, &'static str, &'static str) {
    let patterns = match (pos_group, column_type) {
        (TblGrp::HeadTop, ColumnType::LineNum) => ("", "", bdr!(CM), bdr!(CM)), // LineNum
        (TblGrp::HeadTop, ColumnType::First) => (bdr!(LT), bdr!(TM), bdr!(TT), bdr!(TT)), // First
        (TblGrp::HeadTop, ColumnType::Middle) => ("", bdr!(TM), bdr!(TT), bdr!(TT)), // Middle
        (TblGrp::HeadTop, ColumnType::Last) => ("", bdr!(RT), bdr!(TT), bdr!(TT)), // Last
        (TblGrp::HeadLine, ColumnType::LineNum) => ("", "", bdr!(CM), bdr!(CM)), // LineNum
        (TblGrp::HeadLine, ColumnType::First) => (bdr!(LL), bdr!(IV), bdr!(CM), bdr!(CM)), // First
        (TblGrp::HeadLine, ColumnType::Middle) => ("", bdr!(IV), bdr!(CM), bdr!(CM)), // Middle
        (TblGrp::HeadLine, ColumnType::Last) => ("", bdr!(RR), bdr!(CM), bdr!(CM)), // Last
        (TblGrp::BodySep, ColumnType::LineNum) => ("", "", bdr!(CM), bdr!(CM)), // LineNum
        (TblGrp::BodySep, ColumnType::First) => (bdr!(LM), bdr!(IC), bdr!(IH), bdr!(IH)), // First
        (TblGrp::BodySep, ColumnType::Middle) => ("", bdr!(IC), bdr!(IH), bdr!(IH)), // Middle
        (TblGrp::BodySep, ColumnType::Last) => ("", bdr!(RM), bdr!(IH), bdr!(IH)), // Last
        (TblGrp::BodyLine, ColumnType::LineNum) => ("", "", bdr!(CM), bdr!(CM)), // LineNum
        (TblGrp::BodyLine, ColumnType::First) => (bdr!(LL), bdr!(IV), bdr!(CH), bdr!(CM)), // First
        (TblGrp::BodyLine, ColumnType::Middle) => ("", bdr!(IV), bdr!(CH), bdr!(CM)), // Middle
        (TblGrp::BodyLine, ColumnType::Last) => ("", bdr!(RR), bdr!(CH), bdr!(CM)), // Last
        (TblGrp::BodyBtm, ColumnType::LineNum) => ("", "", bdr!(CM), bdr!(CM)), // LineNum
        (TblGrp::BodyBtm, ColumnType::First) => (bdr!(LB), bdr!(BM), bdr!(BB), bdr!(BB)), // First
        (TblGrp::BodyBtm, ColumnType::Middle) => ("", bdr!(BM), bdr!(BB), bdr!(BB)), // Middle
        (TblGrp::BodyBtm, ColumnType::Last) => ("", bdr!(RB), bdr!(BB), bdr!(BB)), // Last
        (TblGrp::Summary, ColumnType::LineNum) => ("", "", bdr!(CM), bdr!(CM)), // LineNum
        (TblGrp::Summary, ColumnType::First) => (bdr!(CM), bdr!(CM), bdr!(CM), bdr!(CM)), // First
        (TblGrp::Summary, ColumnType::Middle) => ("", bdr!(CM), bdr!(CM), bdr!(CM)), // Middle
        (TblGrp::Summary, ColumnType::Last) => ("", bdr!(CM), bdr!(CM), bdr!(CM)), // Last
        (_, _) => ("", "", "", ""),
    };
    patterns
}

/// Table Border Char Group
/// Type Group: Corner, SepBorder, Border, Inner
/// Line Group: HeadTop, HeadLine, BodySep, BodyLine, BodyBtm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TblGrp {
    Corner = 0,
    SepBorder,
    Border,
    Inner,
    HeadTop,
    HeadLine,
    BodySep,
    BodyLine,
    BodyBtm,
    Summary,
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
                    title: "No".to_string(),
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
    pub fn fmt_cell(&self, tbl_grp: TblGrp, column_idx: usize, cell_txt: &str) -> String {
        // set column type by column index
        let col_type = match column_idx {
            0 if self.tbl_cfg.has_linenum => ColumnType::LineNum,
            0 if !self.tbl_cfg.has_linenum => ColumnType::First,
            1 if self.tbl_cfg.has_linenum => ColumnType::First,
            idx if idx < self.tbl_cfg.columns.len() - 1 => ColumnType::Middle,
            idx if idx == self.tbl_cfg.columns.len() - 1 => ColumnType::Last,
            _ => {
                assert!(false);
                ColumnType::Middle
            }
        };

        // choice cell format pattern item: left_border, right_border, left_margin, right_margin
        let (left_border, right_border, left_margin, mut right_margin) =
            cell_patterns(tbl_grp, col_type);

        // adjust cell text or border for column width
        let column_cfg = &self.tbl_cfg.columns[column_idx];
        let (column_align, col_width) = (column_cfg.align, column_cfg.width);

        // trim cell_text to column width.
        // if extend out of cell, mark "~" at the end of cell.
        let extend_cell = col_type != ColumnType::LineNum && cell_txt.len() >= col_width;
        let mut rdr_txt: String;
        if extend_cell {
            rdr_txt = cell_txt[0..col_width].to_string();
            right_margin = "~";
        } else {
            rdr_txt = cell_txt.to_string();
        }

        // format cell text
        let need_fill_border =
            tbl_grp == TblGrp::HeadTop || tbl_grp == TblGrp::BodySep || tbl_grp == TblGrp::BodyBtm;
        if col_type == ColumnType::LineNum {
            if need_fill_border {
                rdr_txt = format!("{:width$}", "", width = col_width);
            } else {
                if rdr_txt == "" {
                    rdr_txt = format!("{:>width$}", rdr_txt, width = col_width);
                } else {
                    rdr_txt = format!("{:0>width$}", rdr_txt, width = col_width);
                }
            }
        } else {
            if need_fill_border {
                rdr_txt = left_margin.repeat(col_width); // two margins
            } else {
                rdr_txt = match column_align {
                    ColumnAlign::Left => {
                        format!("{:<width$}", rdr_txt, width = col_width)
                    }
                    ColumnAlign::Center => {
                        format!("{:^width$}", rdr_txt, width = col_width)
                    }
                    ColumnAlign::Right => {
                        format!("{:>width$}", rdr_txt, width = col_width)
                    }
                };
            }
        }

        // format line
        format!("{left_border}{left_margin}{rdr_txt}{right_margin}{right_border}")
    }

    /// Format Table Whole Line
    pub fn fmt_body_line(&self, tbl_grp: TblGrp, all_cells: Vec<&str>) -> String {
        assert!(all_cells.len() == self.tbl_cfg.columns.len());
        assert!(
            tbl_grp == TblGrp::BodyLine
                || tbl_grp == TblGrp::HeadLine
                || tbl_grp == TblGrp::Summary
        );
        let mut line_render = String::new();
        for (idx, _column) in self.tbl_cfg.columns.iter().enumerate() {
            let line = self.fmt_cell(tbl_grp, idx, &all_cells[idx]);
            line_render.push_str(line.as_str());
        }
        line_render
    }

    /// Format Table Multi-Lines
    pub fn fmt_body_mlines(&self, tbl_grp: TblGrp, all_cells: Vec<&str>) -> Vec<String> {
        assert!(all_cells.len() == self.tbl_cfg.columns.len());
        assert!(tbl_grp == TblGrp::BodyLine || tbl_grp == TblGrp::HeadLine);

        // 1) prepare slice line cells to multi-lines if some cell's test are too long.
        //    slice record: col_idx, start, end, guard
        let mut all_slice: Vec<(usize, usize, usize, usize)> = all_cells
            .iter()
            .enumerate()
            .map(|(i, s)| {
                (
                    i,                                                // column index
                    0,                                                // slice start position
                    cmp::min(s.len(), self.tbl_cfg.columns[i].width), // slice end position
                    s.len(),                                          // slice end guard position
                )
            })
            .collect();

        // let slice_cell = move |col_idx| {
        // let (start, guard) = all_slice[col_idx];
        // let col_width = self.tbl_cfg.columns[col_idx].width;
        // cell_txt[start..cmp::min(start + col_width, guard)]
        // };

        // 2) loop slice cell text until all of them text are rendered.
        let mut rendered_lines: Vec<String> = Vec::new();
        loop {
            // 3) from last slice position, slice every cell's text into current line
            let cur_line_cells: Vec<&str> = all_cells
                .iter()
                .enumerate()
                .map(|(i, s)| &s[all_slice[i].1..all_slice[i].2]) // slice cell text[start, end]
                .collect();

            // 4) render current line and then push to multi-lines result vector.
            let mut cur_rendered_line: String = String::new();
            cur_rendered_line.push_str(self.fmt_body_line(tbl_grp, cur_line_cells).as_str());
            rendered_lines.push(cur_rendered_line);

            // 5) update slice records
            all_slice.iter_mut().for_each(|r| {
                if r.2 == r.3 {
                    // if this cell has total rendered
                    (r.1, r.2) = (r.3, r.3);
                } else {
                    // render rest cell test, slice to next text window
                    r.1 = r.2;
                    r.2 += cmp::min(self.tbl_cfg.columns[r.0].width, r.3 - r.2);
                }
            });

            // 6) break render loop if all of cell have rendered.
            if all_slice.iter().all(|r| r.1 == r.2 && r.2 == r.3) {
                break;
            }
        }
        rendered_lines
    }

    /// Format Table Buildin Line.
    /// builtin line only in one line.
    pub fn fmt_buildin_line(&self, tbl_grp: TblGrp) -> String {
        assert!(
            tbl_grp == TblGrp::HeadTop
                || tbl_grp == TblGrp::HeadLine
                || tbl_grp == TblGrp::BodySep
                || tbl_grp == TblGrp::BodyBtm
        );

        let cell_text_list: Vec<&str> = match tbl_grp {
            TblGrp::HeadLine => self
                .tbl_cfg
                .columns
                .iter()
                .map(|i| i.title.as_str())
                .collect(),
            _ => self.tbl_cfg.columns.iter().map(|_| "").collect(),
        };
        let mut line_render = String::new();
        for (idx, _column) in self.tbl_cfg.columns.iter().enumerate() {
            let line = self.fmt_cell(tbl_grp, idx, &cell_text_list[idx]);
            line_render.push_str(line.as_str());
        }
        line_render
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn table_border_char() {
        assert_eq!(TableTool::char_at_tblpos(TablePos::InnerMark), "█");
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

        let tb = TableTool::attach(tbl_cfg);
        assert_eq!(tb.table_width(), 29)
    }

    #[test]
    fn table_tool_fmt_cell() {
        let mut tbl_cfg = TableConfig::start_build();
        tbl_cfg.new_column(10, "First Col", ColumnAlign::Left);
        tbl_cfg.auto_column("Left Column ");
        tbl_cfg.auto_column(" Middle Column ");
        tbl_cfg.auto_column(" Right Column");
        tbl_cfg.build_done(true);

        let tb = TableTool::attach(tbl_cfg);
        assert_eq!(tb.tbl_cfg.columns[0].width, 2);
        assert_eq!(tb.tbl_cfg.columns[1].width, 10);
        assert_eq!(tb.tbl_cfg.columns[2].width, 12);
        assert_eq!(tb.tbl_cfg.columns[3].width, 15);
        assert_eq!(tb.tbl_cfg.columns[4].width, 13);

        assert_eq!(tb.fmt_cell(TblGrp::HeadTop, 0, "xx"), "    ");
        assert_eq!(tb.fmt_cell(TblGrp::HeadTop, 1, "xx"), "┌────────────┬");
        assert_eq!(tb.fmt_cell(TblGrp::HeadTop, 2, "xx"), "──────────────┬");
        assert_eq!(tb.fmt_cell(TblGrp::HeadTop, 3, "xx"), "─────────────────┬");
        assert_eq!(tb.fmt_cell(TblGrp::HeadTop, 4, "xx"), "───────────────┐");

        assert_eq!(tb.fmt_cell(TblGrp::HeadLine, 0, "1"), " 01 ");
        assert_eq!(tb.fmt_cell(TblGrp::HeadLine, 1, "x"), "│ x          │");
        assert_eq!(tb.fmt_cell(TblGrp::HeadLine, 2, "x"), " x            │");
        assert_eq!(tb.fmt_cell(TblGrp::HeadLine, 3, "x"), "        x        │");
        assert_eq!(tb.fmt_cell(TblGrp::HeadLine, 4, "x"), "             x │");

        assert_eq!(tb.fmt_cell(TblGrp::BodySep, 0, "xx"), "    ");
        assert_eq!(tb.fmt_cell(TblGrp::BodySep, 1, "xx"), "├────────────┼");
        assert_eq!(tb.fmt_cell(TblGrp::BodySep, 2, "xx"), "──────────────┼");
        assert_eq!(tb.fmt_cell(TblGrp::BodySep, 3, "xx"), "─────────────────┼");
        assert_eq!(tb.fmt_cell(TblGrp::BodySep, 4, "xx"), "───────────────┤");

        assert_eq!(tb.fmt_cell(TblGrp::BodyLine, 0, "1"), " 01 ");
        assert_eq!(tb.fmt_cell(TblGrp::BodyLine, 1, "x"), "│█x          │");
        assert_eq!(
            tb.fmt_cell(
                TblGrp::BodyLine,
                2,
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
            ),
            "█xxxxxxxxxxxx~│"
        );
        assert_eq!(tb.fmt_cell(TblGrp::BodyLine, 3, "x"), "█       x        │");
        assert_eq!(tb.fmt_cell(TblGrp::BodyLine, 4, "x"), "█            x │");

        assert_eq!(tb.fmt_cell(TblGrp::BodyBtm, 0, ""), "    ");
        assert_eq!(tb.fmt_cell(TblGrp::BodyBtm, 1, "xx"), "└────────────┴");
        assert_eq!(tb.fmt_cell(TblGrp::BodyBtm, 2, "xx"), "──────────────┴");
        assert_eq!(tb.fmt_cell(TblGrp::BodyBtm, 3, "xx"), "─────────────────┴");
        assert_eq!(tb.fmt_cell(TblGrp::BodyBtm, 4, "xx"), "───────────────┘");
    }

    #[test]
    fn table_tool_fmt_line() {
        let mut tbl_cfg = TableConfig::start_build();
        tbl_cfg.new_column(10, "First", ColumnAlign::Left);
        tbl_cfg.auto_column("Left        ");
        tbl_cfg.auto_column(" Middle        ");
        tbl_cfg.auto_column("        Right");
        tbl_cfg.build_done(true);

        let tb = TableTool::attach(tbl_cfg);
        assert_eq!(
            tb.fmt_buildin_line(TblGrp::HeadTop),
            "    ┌────────────┬──────────────┬─────────────────┬───────────────┐"
        );
        assert_eq!(
            tb.fmt_buildin_line(TblGrp::HeadLine),
            " No │ First      │ Left         │     Middle      │         Right │"
        );
        assert_eq!(
            tb.fmt_buildin_line(TblGrp::BodySep),
            "    ├────────────┼──────────────┼─────────────────┼───────────────┤"
        );
        assert_eq!(
            tb.fmt_body_line(
                TblGrp::BodyLine,
                vec![
                    "1",
                    "ooooooooooooooooooooooooooooooooooooooooo",
                    "x",
                    "y",
                    "z",
                ]
            ),
            " 01 │█oooooooooo~│█x            │█       y        │█            z │"
        );
        let tri = tb.fmt_body_mlines(
            TblGrp::BodyLine,
            vec![
                "2",
                "1234567890123456789012345",
                "X",
                "YYYYYYYYYYYYYYYYYYY",
                "Z",
            ],
        );
        assert_eq!(
            tri[0],
            " 02 │█1234567890~│█X            │█YYYYYYYYYYYYYYY~│█            Z │"
        );
        assert_eq!(
            tri[1],
            "    │█1234567890~│█             │█     YYYY       │█              │"
        );
        assert_eq!(
            tri[2],
            "    │█12345      │█             │█                │█              │"
        );
        assert_eq!(
            tb.fmt_buildin_line(TblGrp::BodyBtm),
            "    └────────────┴──────────────┴─────────────────┴───────────────┘"
        );
        assert_eq!(
            tb.fmt_body_line(
                TblGrp::Summary,
                vec!["--", "1,243.0", "-23,432.3", "0.00", "TOTAL",]
            ),
            " --   1,243.0      -23,432.3           0.00                 TOTAL  "
        );
    }
}
