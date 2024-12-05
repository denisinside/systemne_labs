use crate::rectangle::{RectTraitKey, RectTraitValue, Rectangle};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde_json::{json, Value};

#[derive(Debug)]
pub enum RectData {
    SideX(f64),
    SideY(f64),
    Perimeter(f64),
    Area(f64),
    Diagonal(f64),
    DiagonalDiagonalAngle(f64),
    SideXDiagonalAngle(f64),
    SideYDiagonalAngle(f64),
    SmallerSide(f64),
    BiggerSide(f64),
    Ratio(f64, f64),
    SideDistances(f64, f64),
    CircumscribedCircleRadius(f64),
    CircumscribedCircleDiameter(f64),
    CircumscribedCircleArea(f64),
    CircumscribedCirclePerimeter(f64),
}

#[derive(Debug, Clone)]
pub enum RectTarget {
    Sides,
    Perimeter,
    Area,
    Diagonal,
    DiagonalDiagonalAngle,
    SideXDiagonalAngle,
    SideYDiagonalAngle,
    SmallerSide,
    BiggerSide,
    SideDistances,
    CircumscribedCircleRadius,
    CircumscribedCircleDiameter,
    CircumscribedCircleArea,
    CircumscribedCirclePerimeter,
}

static KEYWORDS: &[(&str, &str)] = &[
    ("периметр", "perimeter"),
    ("площа", "area"),
    ("діагональ", "diagonal"),
    ("кут", "angle"),
    ("менша сторона", "smaller_side"),
    ("більша сторона", "bigger_side"),
];

pub struct GeometryTaskAnalyser;

impl GeometryTaskAnalyser {
    pub fn get_task_data(
        significant_words: Vec<String>,
    ) -> (HashMap<String, Option<RectData>>, Vec<RectTarget>) {
        let mut task_data: HashMap<String, Option<RectData>> = HashMap::new();
        task_data.insert("bigger_side".to_string(), None);
        task_data.insert("smaller_side".to_string(), None);
        task_data.insert("diagonal".to_string(), None);
        task_data.insert("perimeter".to_string(), None);
        task_data.insert("area".to_string(), None);
        task_data.insert("angle".to_string(), None);
        task_data.insert("ratio".to_string(), None);
        task_data.insert("sideX".to_string(), None);
        task_data.insert("sideY".to_string(), None);
        task_data.insert("side_distances".to_string(), None);
        task_data.insert("diagonal_angle".to_string(), None);
        task_data.insert("side_diagonal_angle".to_string(), None);
        task_data.insert("radius".to_string(), None);
        task_data.insert("diameter".to_string(), None);
        task_data.insert("circle_area".to_string(), None);
        task_data.insert("circle_perimeter".to_string(), None);

        let mut target_data = Vec::new();
        let mut in_target = false;

        let mut i = 0;
        while i < significant_words.len() {
            match significant_words[i].as_str() {
                "периметр" => {
                    if in_target {
                        target_data.push(RectTarget::Perimeter);
                    } else if !Self::keyword_before_number(&significant_words, i + 1) {
                        if let Some(value) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                        {
                            task_data.insert(
                                "perimeter".to_string(),
                                Some(RectData::Perimeter(value.0)),
                            );
                        }
                    }
                }
                "площа" => {
                    if Self::word_before_next_data("коло", &significant_words, i + 1) {
                        if in_target {
                            target_data.push(RectTarget::CircumscribedCircleArea);
                            i += 2;
                        } else if let Some(s) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                        {
                            task_data.insert(
                                "circle_area".to_string(),
                                Some(RectData::CircumscribedCircleArea(s.0)),
                            );
                            i = s.1;
                        }
                    } else if in_target {
                        target_data.push(RectTarget::Area);
                    } else if !Self::keyword_before_number(&significant_words, i + 1) {
                        if let Some(value) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                        {
                            task_data.insert("area".to_string(), Some(RectData::Area(value.0)));
                        }
                    }
                }
                "діагональ" => {
                    if !Self::keyword_before_number(&significant_words, i + 1) {
                        if Self::word_before_next_data("сторона", &significant_words, i + 1)
                            || Self::word_before_next_data("вона", &significant_words, i + 1)
                        {
                            if Self::word_before_next_data("кут", &significant_words, i + 2) {
                                if in_target {
                                    target_data.push(RectTarget::SideXDiagonalAngle);
                                } else if let Some(value) = GeometryTaskAnalyser::find_next_number(
                                    &significant_words,
                                    i + 1,
                                ) {
                                    task_data.insert(
                                        "side_diagonal_angle".to_string(),
                                        Some(RectData::SideXDiagonalAngle(value.0)),
                                    );
                                }
                            }
                        } else if Self::word_before_next_data(
                            "перетинатися",
                            &significant_words,
                            i + 1,
                        ) {
                            if Self::word_before_next_data("кут", &significant_words, i + 2) {
                                if in_target {
                                    target_data.push(RectTarget::DiagonalDiagonalAngle);
                                } else if let Some(value) = GeometryTaskAnalyser::find_next_number(
                                    &significant_words,
                                    i + 1,
                                ) {
                                    task_data.insert(
                                        "diagonal_angle".to_string(),
                                        Some(RectData::DiagonalDiagonalAngle(value.0)),
                                    );
                                    i += 1;
                                }
                            }
                        } else if in_target {
                            target_data.push(RectTarget::Diagonal);
                        } else if let Some(value) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                        {
                            task_data
                                .insert("diagonal".to_string(), Some(RectData::Diagonal(value.0)));
                        }
                    }
                }
                "відноситися" | "співвідношення" | "відношення" => {
                    if !Self::keyword_before_number(&significant_words, i + 1) {
                        if let Some(x) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                        {
                            if !Self::keyword_before_number(&significant_words, x.1 + 1) {
                                if let Some(y) = GeometryTaskAnalyser::find_next_number(
                                    &significant_words,
                                    x.1 + 1,
                                ) {
                                    task_data.insert(
                                        "ratio".to_string(),
                                        Some(RectData::Ratio(x.0, y.0)),
                                    );
                                    i += 1;
                                }
                            }
                        }
                    }
                }
                "кут" => {
                    if Self::word_before_next_data("діагональ", &significant_words, i)
                    {
                        if in_target {
                            target_data.push(RectTarget::DiagonalDiagonalAngle);
                        } else if let Some(value) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                        {
                            task_data.insert(
                                "diagonal_angle".to_string(),
                                Some(RectData::DiagonalDiagonalAngle(value.0)),
                            );
                            i += 1;
                        }
                    }
                }
                "менший" => {
                    if in_target {
                        target_data.push(RectTarget::SmallerSide);
                    } else if !Self::keyword_before_number(&significant_words, i + 1) {
                        if let Some(value) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 2)
                        {
                            match task_data.get("sideX").unwrap() {
                                Some(_) => match task_data.get("sideY").unwrap() {
                                    None => {
                                        task_data.insert(
                                            "sideY".to_string(),
                                            Some(RectData::SideY(value.0)),
                                        );
                                    }
                                    _ => {}
                                },
                                None => {
                                    task_data.insert(
                                        "sideX".to_string(),
                                        Some(RectData::SideX(value.0)),
                                    );
                                }
                            }
                            task_data.insert(
                                "smaller_side".to_string(),
                                Some(RectData::SmallerSide(value.0)),
                            );
                            i += 1;
                        }
                    }
                }
                "більший" => {
                    if in_target {
                        target_data.push(RectTarget::BiggerSide);
                    } else if !Self::keyword_before_number(&significant_words, i + 1) {
                        if let Some(value) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 2)
                        {
                            match task_data.get("sideX").unwrap() {
                                Some(_) => match task_data.get("sideY").unwrap() {
                                    None => {
                                        task_data.insert(
                                            "sideY".to_string(),
                                            Some(RectData::SideY(value.0)),
                                        );
                                    }
                                    _ => {}
                                },
                                None => {
                                    task_data.insert(
                                        "sideX".to_string(),
                                        Some(RectData::SideX(value.0)),
                                    );
                                }
                            }
                            task_data.insert(
                                "bigger_side".to_string(),
                                Some(RectData::BiggerSide(value.0)),
                            );
                            i += 1;
                        }
                    }
                }
                "сторона" => {
                    if in_target {
                        target_data.push(RectTarget::Sides);
                    } else if !Self::keyword_before_number(&significant_words, i + 1) {
                        if let Some(x) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                        {
                            task_data.insert("sideX".to_string(), Some(RectData::SideX(x.0)));
                            if !Self::keyword_before_number(&significant_words, x.1 + 1) {
                                if let Some(y) = GeometryTaskAnalyser::find_next_number(
                                    &significant_words,
                                    x.1 + 1,
                                ) {
                                    task_data
                                        .insert("sideY".to_string(), Some(RectData::SideY(y.0)));
                                    i += 1;
                                }
                            }
                        }
                    }
                }
                "відстань" => {
                    if in_target {
                        target_data.push(RectTarget::SideDistances);
                    } else if let Some(d1) =
                        GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                    {
                        if let Some(d2) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, d1.1 + 1)
                        {
                            task_data.insert(
                                "side_distances".to_string(),
                                Some(RectData::SideDistances(d1.0, d2.0)),
                            );
                            i = d1.1;
                        }
                    }
                }
                "радіус" => {
                    if in_target {
                        target_data.push(RectTarget::CircumscribedCircleRadius);
                    } else if let Some(r) =
                        GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                    {
                        task_data.insert(
                            "radius".to_string(),
                            Some(RectData::CircumscribedCircleRadius(r.0)),
                        );
                    }
                }
                "діаметер" => {
                    if in_target {
                        target_data.push(RectTarget::CircumscribedCircleDiameter);
                    } else if let Some(r) =
                        GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                    {
                        task_data.insert(
                            "diameter".to_string(),
                            Some(RectData::CircumscribedCircleRadius(r.0)),
                        );
                    }
                }
                "довжина" => {
                    if in_target {
                        target_data.push(RectTarget::CircumscribedCirclePerimeter);
                    } else if let Some(r) =
                        GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                    {
                        task_data.insert(
                            "circle_radius".to_string(),
                            Some(RectData::CircumscribedCircleRadius(r.0)),
                        );
                    }
                }
                "коло" => {
                    if Self::word_before_next_data("площа", &significant_words, i + 1) {
                        if in_target {
                            target_data.push(RectTarget::CircumscribedCircleArea);
                            i += 2;
                        } else if let Some(s) =
                            GeometryTaskAnalyser::find_next_number(&significant_words, i + 1)
                        {
                            task_data.insert(
                                "circle_area".to_string(),
                                Some(RectData::CircumscribedCircleArea(s.0)),
                            );
                            i = s.1;
                        }
                    }
                }
                "обчислити" | "знайти" => {
                    in_target = true;
                }
                _ => {}
            }
            i += 1;
        }

        (task_data, target_data)
    }
    fn find_next_number(words: &Vec<String>, start_idx: usize) -> Option<(f64, usize)> {
        for i in start_idx..std::cmp::min(start_idx + 10, words.len()) {
            if let Ok(value) = words[i].parse::<f64>() {
                return Some((value, i));
            }
        }
        None
    }

    fn word_before_next_data(word: &str, words: &Vec<String>, start_idx: usize) -> bool {
        for i in start_idx..std::cmp::min(start_idx + 5, words.len()) {
            if *&words[i].eq(word) {
                return true;
            }
            if let Ok(_) = words[i].parse::<f64>() {
                return false;
            }
        }
        false
    }

    fn keyword_before_number(words: &Vec<String>, start_idx: usize) -> bool {
        for i in start_idx..std::cmp::min(start_idx + 3, words.len()) {
            if KEYWORDS
                .iter()
                .filter(|k| k.0.eq(&words[i]) || k.1.eq(&words[i]))
                .count()
                > 1
            {
                return true;
            }
            if let Ok(_) = words[i].parse::<f64>() {
                return false;
            }
        }
        false
    }
}

pub struct Solver;

impl Solver {
    pub fn solve_geometry_task(
        task_data: &HashMap<String, Option<RectData>>,
        target_data: &Vec<RectTarget>,
    ) -> Vec<(String, Rectangle)> {
        let mut rect = Rectangle::new(0.0, 0.0);
        let mut steps: Vec<(String, Rectangle)> = Vec::new();

        if let Some(RectData::SideX(width)) = task_data.get("sideX").unwrap() {
            rect.width = *width;
        }
        if let Some(RectData::SideY(height)) = task_data.get("sideY").unwrap() {
            rect.height = *height;
        }
        if let Some(RectData::Perimeter(value)) = task_data.get("perimeter").unwrap() {
            rect.traits
                .insert(RectTraitKey::Perimeter, RectTraitValue::Single(*value));
        }
        if let Some(RectData::Area(value)) = task_data.get("area").unwrap() {
            rect.traits
                .insert(RectTraitKey::Area, RectTraitValue::Single(*value));
        }
        if let Some(RectData::Diagonal(value)) = task_data.get("diagonal").unwrap() {
            rect.traits
                .insert(RectTraitKey::Diagonal, RectTraitValue::Single(*value));
        }
        if let Some(RectData::SideDistances(value1, value2)) =
            task_data.get("side_distances").unwrap()
        {
            rect.traits.insert(
                RectTraitKey::SideDistances,
                RectTraitValue::Pair(*value1, *value2),
            );
            rect.find_sides_by_side_distances();
        }
        if let Some(RectData::BiggerSide(value)) = task_data.get("bigger_side").unwrap() {
            rect.traits
                .insert(RectTraitKey::BiggerSide, RectTraitValue::Single(*value));
        }
        if let Some(RectData::SmallerSide(value)) = task_data.get("smaller_side").unwrap() {
            rect.traits
                .insert(RectTraitKey::SmallerSide, RectTraitValue::Single(*value));
        }
        if let Some(RectData::CircumscribedCircleDiameter(value)) = task_data.get("diameter").unwrap() {
            rect.traits
                .insert(RectTraitKey::CircumscribedCircleDiameter, RectTraitValue::Single(*value));
        }
        if let Some(RectData::CircumscribedCircleArea(value)) = task_data.get("circle_area").unwrap() {
            rect.traits
                .insert(RectTraitKey::CircumscribedCircleArea, RectTraitValue::Single(*value));
        }
        if let Some(RectData::CircumscribedCirclePerimeter(value)) = task_data.get("circle_perimeter").unwrap() {
            rect.traits
                .insert(RectTraitKey::CircumscribedCirclePerimeter, RectTraitValue::Single(*value));
        }
        if let Some(RectData::CircumscribedCircleRadius(value)) = task_data.get("radius").unwrap() {
            rect.traits
                .insert(RectTraitKey::CircumscribedCircleRadius, RectTraitValue::Single(*value));
        }
        if let Some(RectData::DiagonalDiagonalAngle(value)) =
            task_data.get("diagonal_angle").unwrap()
        {
            rect.traits.insert(
                RectTraitKey::DiagonalDiagonalAngle,
                RectTraitValue::Single(*value),
            );
        }

        let mut target_list = target_data.clone();

        if let Some(RectData::Ratio(r1, r2)) = task_data.get("ratio").unwrap() {
            if rect.has_trait(RectTraitKey::Perimeter) {
                rect.find_sides_from_perimeter_and_ratio((*r1, *r2));
            } else if rect.has_trait(RectTraitKey::Diagonal) {
                rect.find_sides_from_diagonal_and_ratio((*r1, *r2));
            } else {
                rect.find_sides_from_one_side_and_ratio((*r1, *r2));
            }
        }

        steps.push(("Init".to_string(), rect.clone()));

        while target_list.len() > 0 {
            let mut action = "";
            match target_list.get(0) {
                Some(RectTarget::Perimeter) => {
                    if rect.has_trait(RectTraitKey::Perimeter) {
                        target_list.remove(0);
                        action = "Perimeter is already known";
                    } else if rect.width != 0.0 && rect.height != 0.0 {
                        rect.perimeter();
                        target_list.remove(0);
                        action = "Found perimeter using sides";
                    } else if rect.width > 0.0 || rect.height > 0.0 {
                        if rect.has_trait(RectTraitKey::Area) {
                            rect.find_perimeter_by_area();
                            target_list.remove(0);
                            action = "Found perimeter using side and area";
                        } else if rect.has_trait(RectTraitKey::Diagonal) {
                            rect.find_perimeter_by_diagonal();
                            target_list.remove(0);
                            action = "Found perimeter using side and diagonal";
                        } else {
                            target_list.insert(0, RectTarget::Sides);
                        }
                    } else {
                        target_list.insert(0, RectTarget::Sides);
                    }
                }
                Some(RectTarget::Area) => {
                    if rect.has_trait(RectTraitKey::Area) {
                        target_list.remove(0);
                        action = "Area is already known";
                    } else if rect.width != 0.0 && rect.height != 0.0 {
                        rect.area();
                        target_list.remove(0);
                        action = "Found area using sides";
                    } else if rect.width > 0.0 || rect.height > 0.0 {
                        if rect.has_trait(RectTraitKey::Perimeter) {
                            rect.find_area_by_perimeter();
                            target_list.remove(0);
                            action = "Found area using side and perimeter";
                        } else if rect.has_trait(RectTraitKey::Diagonal) {
                            rect.find_area_by_diagonal();
                            target_list.remove(0);
                            action = "Found area using side and diagonal";
                        } else {
                            target_list.insert(0, RectTarget::Sides);
                        }
                    } else {
                        target_list.insert(0, RectTarget::Sides);
                    }
                }
                Some(RectTarget::Diagonal) => {
                    if rect.has_trait(RectTraitKey::Diagonal) {
                        target_list.remove(0);
                        action = "Diagonal is already known";
                    } else if rect.width != 0.0 && rect.height != 0.0 {
                        rect.diagonal();
                        target_list.remove(0);
                        action = "Found diagonal using sides";
                    } else if rect.has_trait(RectTraitKey::CircumscribedCircleRadius)
                        || rect.has_trait(RectTraitKey::CircumscribedCircleDiameter)
                    {
                        rect.diagonal_from_radius_or_diameter();
                        target_list.remove(0);
                        action = "Found diagonal using circumscribed circle radius";
                    } else if rect.has_trait(RectTraitKey::CircumscribedCircleArea)
                        || rect.has_trait(RectTraitKey::CircumscribedCirclePerimeter)
                    {
                        target_list.insert(0, RectTarget::CircumscribedCircleRadius);
                    } else if rect.has_trait(RectTraitKey::DiagonalDiagonalAngle)
                        && rect.has_trait(RectTraitKey::Area)
                    {
                        rect.find_diagonal_by_diagonals_angle_and_area();
                        target_list.remove(0);
                        action = "Found diagonal using angle between diagonals and area";
                    } else if rect.width > 0.0 || rect.height > 0.0 {
                        if rect.has_trait(RectTraitKey::Perimeter) {
                            rect.find_diagonal_by_perimeter();
                            target_list.remove(0);
                            action = "Found diagonal using side and perimeter";
                        } else if rect.has_trait(RectTraitKey::Area) {
                            rect.find_diagonal_by_area();
                            target_list.remove(0);
                            action = "Found diagonal using side and area";
                        } else {
                            target_list.insert(0, RectTarget::Sides);
                        }
                    } else {
                        target_list.insert(0, RectTarget::Sides);
                    }
                }
                Some(RectTarget::Sides) => {
                    if rect.has_trait(RectTraitKey::SideDistances) {
                        rect.find_sides_by_side_distances();
                        target_list.remove(0);
                        action = "Found sides using distances from diagonal intersection point";
                    } else if rect.has_trait(RectTraitKey::Perimeter) {
                        rect.find_side_by_perimeter();
                        target_list.remove(0);
                        action = "Found sides using side and perimeter";
                    } else if rect.has_trait(RectTraitKey::Area) {
                        if rect.width <= 0. && rect.height <= 0. {
                            if rect.has_trait(RectTraitKey::Diagonal) {
                                rect.find_sides_from_area_and_diagonal();
                                target_list.remove(0);
                                action = "Found sides using area and diagonal";
                            }
                        } else {
                            rect.find_side_by_area();
                            target_list.remove(0);
                            action = "Found sides using side and area";
                        }
                    } else if rect.has_trait(RectTraitKey::Diagonal) {
                        rect.find_side_by_diagonal();
                        target_list.remove(0);
                        action = "Found sides using side and diagonal";
                    } else if rect.has_trait(RectTraitKey::DiagonalDiagonalAngle)
                        && (rect.has_trait(RectTraitKey::SmallerSide)
                            || rect.has_trait(RectTraitKey::BiggerSide))
                    {
                        rect.find_diagonal_by_angle_and_side();
                        target_list.remove(0);
                        action = "Found sides using side and angle between diagonal and side";
                    } else if rect.has_trait(RectTraitKey::CircumscribedCircleRadius)
                        || rect.has_trait(RectTraitKey::CircumscribedCircleDiameter)
                    {
                        target_list.insert(0, RectTarget::Diagonal);
                    } else {
                        break;
                    }
                }
                Some(RectTarget::SideDistances) => {
                    if rect.width != 0.0 && rect.height != 0.0 {
                        rect.calculate_side_distances();
                        target_list.remove(0);
                        action = "Found distances from intersection point using sides";
                    } else {
                        target_list.insert(0, RectTarget::Sides);
                    }
                }
                Some(RectTarget::SideXDiagonalAngle) => {
                    if rect.has_trait(RectTraitKey::Diagonal) {
                        if rect.width != 0.0 && rect.height != 0.0 {
                            rect.angle_between_side_and_diagonal();
                            target_list.remove(0);
                            action = "Found angle between diagonal and side";
                        } else {
                            target_list.insert(0, RectTarget::Sides);
                        }
                    } else {
                        target_list.insert(0, RectTarget::Diagonal);
                    }
                }
                Some(RectTarget::DiagonalDiagonalAngle) => {
                    if rect.has_trait(RectTraitKey::Diagonal) {
                        if rect.has_trait(RectTraitKey::Area) {
                            rect.angle_between_diagonals();
                            target_list.remove(0);
                            action = "Found intersection angle between diagonals";
                        } else {
                            target_list.insert(0, RectTarget::Area);
                        }
                    } else {
                        target_list.insert(0, RectTarget::Diagonal);
                    }
                }
                Some(RectTarget::SmallerSide) => {
                    if rect.has_trait(RectTraitKey::SmallerSide) {
                        target_list.remove(0);
                    } else if rect.width != 0.0 && rect.height != 0.0 {
                        rect.find_bigger_smaller_side();
                        target_list.remove(0);
                        action = "Found smaller side";
                    } else {
                        target_list.insert(0, RectTarget::Sides);
                    }
                }
                Some(RectTarget::BiggerSide) => {
                    if rect.has_trait(RectTraitKey::BiggerSide) {
                        target_list.remove(0);
                    } else if rect.width != 0.0 && rect.height != 0.0 {
                        rect.find_bigger_smaller_side();
                        target_list.remove(0);
                        action = "Found bigger side";
                    } else {
                        target_list.insert(0, RectTarget::Sides);
                    }
                }
                Some(RectTarget::CircumscribedCircleRadius) => {
                    if rect.has_trait(RectTraitKey::CircumscribedCircleRadius) {
                        target_list.remove(0);
                    } else if rect.has_trait(RectTraitKey::Diagonal)
                        || rect.has_trait(RectTraitKey::CircumscribedCircleDiameter)
                    {
                        rect.circumscribed_circle_radius();
                        target_list.remove(0);
                        action = "Found circumscribed circle radius using diagonal/diameter";
                    } else if rect.has_trait(RectTraitKey::CircumscribedCirclePerimeter) {
                        rect.find_radius_by_circle_perimeter();
                        target_list.remove(0);
                        action = "Found circumscribed circle radius using its perimeter";
                    } else if rect.has_trait(RectTraitKey::CircumscribedCircleArea) {
                        rect.find_radius_by_circle_area();
                        target_list.remove(0);
                        action = "Found circumscribed circle radius using its area";
                    } else {
                        target_list.insert(0, RectTarget::Diagonal);
                    }
                }
                Some(RectTarget::CircumscribedCircleDiameter) => {
                    if rect.has_trait(RectTraitKey::CircumscribedCircleDiameter) {
                        target_list.remove(0);
                    } else if rect.has_trait(RectTraitKey::Diagonal)
                        || rect.has_trait(RectTraitKey::CircumscribedCircleRadius)
                    {
                        rect.circumscribed_circle_diameter();
                        target_list.remove(0);
                        action = "Found circumscribed circle diameter using its radius/rectangle diagonal";
                    } else {
                        target_list.insert(0, RectTarget::Diagonal);
                    }
                }
                Some(RectTarget::CircumscribedCircleArea) => {
                    if rect.has_trait(RectTraitKey::CircumscribedCircleArea) {
                        target_list.remove(0);
                    } else if rect.has_trait(RectTraitKey::CircumscribedCircleRadius) {
                        rect.circumscribed_circle_area();
                        target_list.remove(0);
                        action = "Found circumscribed circle area using its radius";
                    } else {
                        target_list.insert(0, RectTarget::CircumscribedCircleRadius);
                    }
                }
                Some(RectTarget::CircumscribedCirclePerimeter) => {
                    if rect.has_trait(RectTraitKey::CircumscribedCirclePerimeter) {
                        target_list.remove(0);
                    } else if rect.has_trait(RectTraitKey::CircumscribedCircleRadius) {
                        rect.circumscribed_circle_perimeter();
                        target_list.remove(0);
                        action = "Found circumscribed circle perimeter using its radius";
                    } else {
                        target_list.insert(0, RectTarget::CircumscribedCircleRadius);
                    }
                }
                _ => {}
            }
            if target_list.len() == 0 && (rect.width == 0.0 || rect.height == 0.0) {
                target_list.insert(0, RectTarget::Sides);
            }

            if !action.is_empty() {
                steps.push((action.to_string(), rect.clone()));
            }
        }
        steps.push(("Final result".to_string(), rect.clone()));
        steps
    }
    pub fn save_steps_to_json(steps: Vec<(String, Rectangle)>, file_path: PathBuf) -> std::io::Result<()> {
        let json_steps: Vec<Value> = steps
            .into_iter()
            .map(|(description, rectangle)| {
                json!([description, rectangle.to_json_value()])
            })
            .collect();

        let json_string = serde_json::to_string_pretty(&json_steps)?;
        let mut file = File::create(file_path)?;
        file.write_all(json_string.as_bytes())?;

        Ok(())
    }
}
