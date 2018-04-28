// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

#[derive(Clone, PartialEq, Debug)]
pub enum Period {
    // P1Y
    Duration(String),
    // 2018-10-11/2019-10-12
    Range(String, String),
    // 2018-10-11/P1Y
    RangeDateDuration(String, String),
    // P1Y/2018-10-11
    RangeDurationDate(String, String),
}
