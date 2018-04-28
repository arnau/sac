// Copyright 2018 Arnau Siches

// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

// TODO: Probably better to have a struct of atoms
#[derive(Clone, PartialEq, Debug)]
pub enum Datetime {
    // 2018
    Year(String),
    // 2018-10
    YearMonth(String),
    // 2018-10-11
    Date(String),
    // 2018-10-11T12Z
    DateHour(String),
    // 2018-10-11T12:13Z
    DateHourMinute(String),
    // 2018-10-11T12:13:14Z
    Full(String),
}
