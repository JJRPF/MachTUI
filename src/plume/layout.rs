//! Advanced layout utilities for Plume.

use taffy::prelude::*;

/// Maps string property values to Taffy Flexbox properties.
pub fn map_justify_content(val: &str) -> JustifyContent {
    match val {
        "center" => JustifyContent::Center,
        "flex-start" => JustifyContent::FlexStart,
        "flex-end" => JustifyContent::FlexEnd,
        "space-between" => JustifyContent::SpaceBetween,
        "space-around" => JustifyContent::SpaceAround,
        _ => JustifyContent::Start,
    }
}

pub fn map_align_items(val: &str) -> AlignItems {
    match val {
        "center" => AlignItems::Center,
        "flex-start" => AlignItems::FlexStart,
        "flex-end" => AlignItems::FlexEnd,
        "stretch" => AlignItems::Stretch,
        _ => AlignItems::Start,
    }
}

pub fn map_flex_direction(val: &str) -> FlexDirection {
    match val {
        "row" => FlexDirection::Row,
        "row-reverse" => FlexDirection::RowReverse,
        "column-reverse" => FlexDirection::ColumnReverse,
        _ => FlexDirection::Column,
    }
}
