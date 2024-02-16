use crate::geometry::Point;

/// Shape's skeletons   
/// The first four are dedicated to the explosion.   
///
/// Coordinates from 0 and the top left   
/// 0 1 2 3 4 5  
/// 1 . . . . .   
/// 2 . . . . .   
/// 3 . . . . .   
/// 4 . . . . .   
pub fn skeleton(nb: usize) -> Vec<Point> {
    match nb {
        0 => vec![
            //  #
            // #.#
            //  #
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_001 },
            Point { x: 1_001, y: 1_002 },
            Point { x: 1_000, y: 1_001 },
            // --
            Point { x: 1_001, y: 1_001 },
        ],

        1 => vec![
            // ###
            // #.#
            // ###
            Point { x: 1_000, y: 1_000 },
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_002, y: 1_001 },
            Point { x: 1_002, y: 1_002 },
            Point { x: 1_001, y: 1_002 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_000, y: 1_001 },
            // --
            Point { x: 1_001, y: 1_001 },
        ],

        2 => vec![
            //  ###
            // #   #
            // # . #
            // #   #
            //  ###
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_003, y: 1_000 },
            // --
            Point { x: 1_001, y: 1_004 },
            Point { x: 1_002, y: 1_004 },
            Point { x: 1_003, y: 1_004 },
            // --
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_000, y: 1_003 },
            // --
            Point { x: 1_004, y: 1_001 },
            Point { x: 1_004, y: 1_002 },
            Point { x: 1_004, y: 1_003 },
            // --
            Point { x: 1_002, y: 1_002 },
        ],

        3 => vec![
            //   ###
            // #     #
            // #  .  #
            // #     #
            //   ###
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_003, y: 1_000 },
            Point { x: 1_004, y: 1_000 },
            // --
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_006, y: 1_001 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_006, y: 1_002 },
            Point { x: 1_000, y: 1_003 },
            Point { x: 1_006, y: 1_003 },
            // --
            Point { x: 1_002, y: 1_004 },
            Point { x: 1_003, y: 1_004 },
            Point { x: 1_004, y: 1_004 },
            // --
            Point { x: 1_003, y: 1_002 },
        ],

        4 => vec![
            //   #####
            // #       #
            // #  ###  #
            // #  #.#  #
            // #  ###  #
            // #       #
            //   #####
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_003, y: 1_000 },
            Point { x: 1_004, y: 1_000 },
            Point { x: 1_005, y: 1_000 },
            Point { x: 1_006, y: 1_000 },
            // --
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_008, y: 1_001 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_008, y: 1_002 },
            Point { x: 1_000, y: 1_003 },
            Point { x: 1_008, y: 1_003 },
            Point { x: 1_000, y: 1_004 },
            Point { x: 1_008, y: 1_004 },
            Point { x: 1_000, y: 1_005 },
            Point { x: 1_008, y: 1_005 },
            // --
            Point { x: 1_002, y: 1_006 },
            Point { x: 1_003, y: 1_006 },
            Point { x: 1_004, y: 1_006 },
            Point { x: 1_005, y: 1_006 },
            Point { x: 1_006, y: 1_006 },
            // --
            Point { x: 1_003, y: 1_002 },
            Point { x: 1_004, y: 1_002 },
            Point { x: 1_005, y: 1_002 },
            // --
            Point { x: 1_003, y: 1_003 },
            Point { x: 1_005, y: 1_003 },
            // --
            Point { x: 1_003, y: 1_004 },
            Point { x: 1_004, y: 1_004 },
            Point { x: 1_005, y: 1_004 },
            // --
            Point { x: 1_004, y: 1_003 },
        ],

        5 => vec![
            // #####
            // #   #
            // ##.##
            // #   #
            // #   #
            Point { x: 1_000, y: 1_000 },
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_000, y: 1_003 },
            Point { x: 1_000, y: 1_004 },
            // --
            Point { x: 1_004, y: 1_000 },
            Point { x: 1_004, y: 1_001 },
            Point { x: 1_004, y: 1_002 },
            Point { x: 1_004, y: 1_003 },
            Point { x: 1_004, y: 1_004 },
            // --
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_003, y: 1_000 },
            // --
            Point { x: 1_001, y: 1_002 },
            Point { x: 1_002, y: 1_002 },
            Point { x: 1_003, y: 1_002 },
            // --
            Point { x: 1_002, y: 1_002 },
        ],

        6 => vec![
            // #####
            // #
            // # .
            // #
            // #####
            Point { x: 1_000, y: 1_000 },
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_003, y: 1_000 },
            Point { x: 1_004, y: 1_000 },
            // --
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_000, y: 1_003 },
            // --
            Point { x: 1_000, y: 1_004 },
            Point { x: 1_001, y: 1_004 },
            Point { x: 1_002, y: 1_004 },
            Point { x: 1_003, y: 1_004 },
            Point { x: 1_004, y: 1_004 },
            // --
            Point { x: 1_002, y: 1_002 },
        ],

        7 => vec![
            // #####
            // #
            // ##.
            // #
            // #####
            Point { x: 1_000, y: 1_000 },
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_003, y: 1_000 },
            Point { x: 1_004, y: 1_000 },
            // --
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_001, y: 1_002 },
            Point { x: 1_002, y: 1_002 },
            Point { x: 1_000, y: 1_003 },
            // --
            Point { x: 1_000, y: 1_004 },
            Point { x: 1_001, y: 1_004 },
            Point { x: 1_002, y: 1_004 },
            Point { x: 1_003, y: 1_004 },
            Point { x: 1_004, y: 1_004 },
            // --
            Point { x: 1_002, y: 1_002 },
        ],

        8 => vec![
            // #####
            // #
            // # .##
            // #   #
            // #####
            Point { x: 1_000, y: 1_000 },
            Point { x: 1_001, y: 1_000 },
            Point { x: 1_002, y: 1_000 },
            Point { x: 1_003, y: 1_000 },
            Point { x: 1_004, y: 1_000 },
            // --
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_000, y: 1_003 },
            // --
            Point { x: 1_000, y: 1_004 },
            Point { x: 1_001, y: 1_004 },
            Point { x: 1_002, y: 1_004 },
            Point { x: 1_003, y: 1_004 },
            Point { x: 1_004, y: 1_004 },
            // --
            Point { x: 1_004, y: 1_003 },
            Point { x: 1_004, y: 1_002 },
            Point { x: 1_003, y: 1_002 },
            // --
            Point { x: 1_002, y: 1_002 },
        ],

        9 => vec![
            //     #
            //    # #
            //   # . #
            //  #######
            // #       #
            Point { x: 1_004, y: 1_000 },
            Point { x: 1_003, y: 1_001 },
            Point { x: 1_005, y: 1_001 },
            Point { x: 1_002, y: 1_002 },
            Point { x: 1_006, y: 1_002 },
            // --
            Point { x: 1_001, y: 1_003 },
            Point { x: 1_002, y: 1_003 },
            Point { x: 1_003, y: 1_003 },
            Point { x: 1_004, y: 1_003 },
            Point { x: 1_005, y: 1_003 },
            Point { x: 1_006, y: 1_003 },
            Point { x: 1_007, y: 1_003 },
            // --
            Point { x: 1_000, y: 1_004 },
            Point { x: 1_008, y: 1_004 },
            // --
            Point { x: 1_004, y: 1_002 },
        ],

        10 => vec![
            //     #
            // ##   #
            //   .  #
            // ##   #
            //     #
            Point { x: 1_000, y: 1_001 },
            Point { x: 1_001, y: 1_001 },
            Point { x: 1_000, y: 1_003 },
            Point { x: 1_001, y: 1_003 },
            // --
            Point { x: 1_004, y: 1_000 },
            Point { x: 1_005, y: 1_001 },
            Point { x: 1_005, y: 1_002 },
            Point { x: 1_005, y: 1_003 },
            Point { x: 1_004, y: 1_004 },
            // --
            Point { x: 1_002, y: 1_002 },
        ],

        11 => vec![
            //    #
            //   ###
            // #######
            //   ###
            //    #
            Point { x: 1_003, y: 1_000 },
            // --
            Point { x: 1_002, y: 1_001 },
            Point { x: 1_003, y: 1_001 },
            Point { x: 1_004, y: 1_001 },
            // --
            Point { x: 1_000, y: 1_002 },
            Point { x: 1_001, y: 1_002 },
            Point { x: 1_002, y: 1_002 },
            Point { x: 1_003, y: 1_002 },
            Point { x: 1_004, y: 1_002 },
            Point { x: 1_005, y: 1_002 },
            Point { x: 1_006, y: 1_002 },
            // --
            Point { x: 1_002, y: 1_003 },
            Point { x: 1_003, y: 1_003 },
            Point { x: 1_004, y: 1_003 },
            // --
            Point { x: 1_003, y: 1_004 },
            // --
            Point { x: 1_003, y: 1_002 },
        ],

        _ => vec![Point { x: 3, y: 3 }],
    }
}
