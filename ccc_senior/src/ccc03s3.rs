use ccc_lib::types::question::{Question, QuestionType};

/// CCC '03 S3 - Floor Plan
/// Canadian Computing Competition: 2003 Stage 1, Junior #5, Senior #3
///
/// The floor plan of a house shows rooms separated by walls. This floor plan can
/// be transferred to a grid using the character I for walls and . for room space.
/// Doorways are not shown. Each I or . character occupies one square metre.
///
/// Sample 1:
/// 105
/// 14
/// 16
/// IIIIIIIIIIIIIIII
/// I......I.......I
/// I......III.....I
/// I........I.....I
/// I........IIIIIII
/// IIIIIIIIII.....I
/// I.I......I.....I
/// III..III.I.....I
/// I....I.IIIII...I
/// I....I.....III.I
/// I....I.......I.I
/// I....I.....III.I
/// I....I.....I...I
/// IIIIIIIIIIIIIIII
/// In this diagram, there are six rooms.
///
/// You have been given the floor plan of a house and a supply of hardwood flooring.
/// You are to determine how many rooms will have the flooring installed if you
/// start installing the floor in the largest room first and move to the next largest room,
/// and so on. You may not skip over any room, and you must stop when you do not
/// have enough wood for the next room. Output the number of rooms that can have
/// hardwood installed, and how many square metres of flooring are left over.
/// No room will be larger than square metres.
///
/// The first line contains the number of square metres of flooring you have. The second line contains an integer in range
/// that represents the number of rows in the grid. The third line contains an integer
/// in that represents the number of columns in the grid. The remaining lines contain characters of grid data.

pub fn get_question() -> Question {
    Question {
        year: 2003,
        level: QuestionType::Senior,
        num: 3,
        title: "Floor Plan".to_string(),
        description: " \
The floor plan of a house shows rooms separated by walls. This floor plan can \
be transferred to a grid using the character I for walls and . for room space. \
Doorways are not shown. Each I or . character occupies one square metre. \
 \
Sample 1: \
105 \
14 \
16 \
 IIIIIIIIIIIIIIII\
 I......I.......I\
 I......III.....I\
 I........I.....I\
 I........IIIIIII\
 IIIIIIIIII.....I\
 I.I......I.....I\
 III..III.I.....I\
 I....I.IIIII...I\
 I....I.....III.I\
 I....I.......I.I\
 I....I.....III.I\
 I....I.....I...I\
 IIIIIIIIIIIIIIII\
 In this diagram, there are six rooms.\
\
 You have been given the floor plan of a house and a supply of hardwood flooring.\
 You are to determine how many rooms will have the flooring installed if you\
 start installing the floor in the largest room first and move to the next largest room,\
 and so on. You may not skip over any room, and you must stop when you do not\
 have enough wood for the next room. Output the number of rooms that can have\
 hardwood installed, and how many square metres of flooring are left over.\
 No room will be larger than square metres.\
\
 The first line contains the number of square metres of flooring you have. The second line contains an integer in range\
 that represents the number of rows in the grid. The third line contains an integer\
 in that represents the number of columns in the grid. The remaining lines contain characters of grid data.".to_string(),
    }
}

pub fn run_solver() {
    println!("CCC '03 S3 - Floor Plan");
}
