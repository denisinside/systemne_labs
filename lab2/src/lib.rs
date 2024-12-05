mod lexer;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use crate::lexer::Lexer;
use crate::ParseError::*;
use crate::RectangleProperties::{Area, Diagonal, Perimeter};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GrammarParser;

#[derive(Debug)]
pub enum ParseError {
    IncorrectName(String),
    IncorrectValues(String),
    IncorrectCoordinates(String),
    IncorrectInput(String),
    RectangleNotFound(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
enum RectangleProperties {
    Perimeter(f64),
    Area(f64),
    Diagonal(f64),
    IsIntersection(bool),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    x: i32,
    y: i32,
    name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rectangle {
    w: i32,
    h: i32,
    points: Vec<Point>,
    name: String,
    properties: Vec<RectangleProperties>,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, w: i32, h: i32, name: &String) -> Result<Rectangle, ParseError> {
        let point_names = Self::process_name(name)?;

        let p1 = Point {
            x,
            y,
            name: point_names[0].clone(),
        };
        let p2 = Point {
            x,
            y: y + h,
            name: point_names[1].clone(),
        };
        let p3 = Point {
            x: x + w,
            y: y + h,
            name: point_names[2].clone(),
        };
        let p4 = Point {
            x: x + w,
            y,
            name: point_names[3].clone(),
        };

        let points = vec![p1, p2, p3, p4];

        Ok(Rectangle {
            w,
            h,
            points,
            name: name.clone(),
            properties: vec![]
        })
    }

    pub fn new_from_coords(name: &String, coords: &[(i32, i32)]) -> Result<Rectangle, ParseError> {
        let point_names = Self::process_name(name)?;

        if coords.len() != 4 {
            return Err(IncorrectInput(
                "Exactly 4 coordinates are required to define a rectangle.".to_string(),
            ));
        }

        let p1 = Point { x: coords[0].0, y: coords[0].1, name: point_names[0].clone() };
        let p2 = Point { x: coords[1].0, y: coords[1].1, name: point_names[1].clone() };
        let p3 = Point { x: coords[2].0, y: coords[2].1, name: point_names[2].clone() };
        let p4 = Point { x: coords[3].0, y: coords[3].1, name: point_names[3].clone() };

        let side1 = ((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f64;
        let side2 = ((p2.x - p3.x).pow(2) + (p2.y - p3.y).pow(2)) as f64;
        let side3 = ((p3.x - p4.x).pow(2) + (p3.y - p4.y).pow(2)) as f64;
        let side4 = ((p4.x - p1.x).pow(2) + (p4.y - p1.y).pow(2)) as f64;

        if (side1 - side3).abs() > 0.001 || (side2 - side4).abs() > 0.001 {
            return Err(IncorrectInput(
                "Provided coordinates do not form a rectangle.".to_string(),
            ));
        }

        let w = (p2.x - p1.x).abs().max((p4.x - p1.x).abs());
        let h = (p3.y - p1.y).abs().max((p4.y - p1.y).abs());

        Ok(Rectangle {
            w,
            h,
            points: vec![p1, p2, p3, p4],
            name: name.clone(),
            properties: vec![]
        })
    }


    pub fn new_with_ratio(name: &String, ratio: (u32, u32), length: u32, x: i32, y: i32) -> Result<Rectangle, ParseError> {
        Self::process_name(name)?;

        if ratio.0 == 0 || ratio.1 == 0 {
            return Err(IncorrectInput(
                "Incorrect ratio.".to_string()
            ));
        }

        let total_ratio = ratio.0 + ratio.1;
        let unit_length = length as f32 / total_ratio as f32;
        let w = (unit_length * ratio.0 as f32) as i32;
        let h = (unit_length * ratio.1 as f32) as i32;

        Rectangle::new(x, y, w, h, name)
    }

    fn process_name(name: &String) -> Result<Vec<String>, ParseError> {
        let re = Regex::new(r"[A-Z][0-9]*").unwrap();
        let point_names: Vec<String> = re
            .find_iter(name)
            .map(|mat| mat.as_str().to_string())
            .collect();

        if point_names.len() != 4 {
            return Err(IncorrectName(format!(
                "Incorrect name format: {}. Must contain exactly 4 unique point names.",
                name
            )));
        }

        let unique_names: HashSet<_> = point_names.iter().cloned().collect();
        if unique_names.len() != 4 {
            return Err(IncorrectName(format!(
                "Duplicate point names in: {}. Points must be unique.",
                name
            )));
        }

        Ok(point_names)
    }

    pub fn has_point(&self, name: &String) -> bool {
        self.points.iter().any(|p| p.name == *name)
    }

    pub fn area(&mut self) -> i32 {
        self.properties.retain(|p| !matches!(p, Area(_)));
        let area = (self.w.abs() * self.h.abs()) as f64;
        self.properties.push(Area(area));
        area as i32
    }

    pub fn perimeter(&mut self) -> i32 {
        self.properties.retain(|p| !matches!(p, Perimeter(_)));
        let perimeter = 2.0 * (self.w.abs() as f64 + self.h.abs() as f64);
        self.properties.push(Perimeter(perimeter));
        perimeter as i32
    }

    pub fn diagonal(&mut self) -> i32 {
        self.properties.retain(|p| !matches!(p, Diagonal(_)));
        let diagonal = ((self.w.pow(2) + self.h.pow(2)) as f64).sqrt();
        self.properties.push(Diagonal(diagonal));
        diagonal as i32
    }

    fn update_calculations(&mut self) {
        if let Some(_index) = self.properties.iter().position(|prop| matches!(prop, Area(_))) {
            self.area();
        }
        if let Some(_index) = self.properties.iter().position(|prop| matches!(prop, Perimeter(_))) {
            self.perimeter();
        }
        if let Some(_index) = self.properties.iter().position(|prop| matches!(prop, Diagonal(_))) {
            self.diagonal();
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.points[0].x, self.points[0].y)
    }

    pub fn move_rectangle(&mut self, dx: i32, dy: i32) {
        for point in &mut self.points {
            point.x += dx;
            point.y += dy;
        }
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.points[0].x = x;
        self.points[0].y = y;

        self.points[1].x = x;
        self.points[1].y = y + self.h;

        self.points[2].x = x + self.w;
        self.points[2].y = y + self.h;

        self.points[3].x = x + self.w;
        self.points[3].y = y;
    }

    pub fn resize_rectangle(&mut self, factor: f32) {
        self.w = (self.w as f32 * factor) as i32;
        self.h = (self.h as f32 * factor) as i32;
        let pos = self.get_position();
        for point in &mut self.points {
            point.x = (pos.0 as f32 + (point.x - pos.0) as f32 * factor) as i32;
            point.y = (pos.1 as f32 + (point.y - pos.1) as f32 * factor) as i32;
        }
        self.update_calculations();
    }

    pub fn rotate_rectangle(&mut self, angle: f32, center_x: i32, center_y: i32) {
        let angle_rad = angle.to_radians();
        for point in &mut self.points {
            let translated_x = (point.x - center_x) as f32;
            let translated_y = (point.y - center_y) as f32;
            point.x = center_x
                + (translated_x * angle_rad.cos() - translated_y * angle_rad.sin()).round() as i32;
            point.y = center_y
                + (translated_x * angle_rad.sin() + translated_y * angle_rad.cos()).round() as i32;
        }
    }

    pub fn reflect_rectangle(&mut self, axis: char) {
        let pos = self.get_position();
        for point in &mut self.points {
            match axis {
                'X' => point.y = pos.1 - (point.y - pos.1),
                'Y' => point.x = pos.0 - (point.x - pos.0),
                _ => {}
            }
        }
    }

    pub fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        let x1 = self.points[0].x.max(other.points[0].x);
        let y1 = self.points[0].y.max(other.points[0].y);
        let x2 = self.points[2].x.min(other.points[2].x);
        let y2 = self.points[1].y.min(other.points[1].y);

        if x1 < x2 && y1 < y2 {
            let intersection_name = format!(
                "I{}{}",
                self.name.chars().next().unwrap_or('X'),
                other.name.chars().next().unwrap_or('Y')
            );

            let points = vec![
                Point { x: x1, y: y1, name: format!("{}1", intersection_name) },
                Point { x: x1, y: y2, name: format!("{}2", intersection_name) },
                Point { x: x2, y: y2, name: format!("{}3", intersection_name) },
                Point { x: x2, y: y1, name: format!("{}4", intersection_name) },
            ];

            let w = x2 - x1;
            let h = y2 - y1;

            Some(Rectangle {
                w,
                h,
                points,
                name: intersection_name,
                properties: vec![RectangleProperties::IsIntersection(true)],
            })
        } else {
            None
        }
    }

    pub fn rename_point(&mut self, old_name: &str, new_name: &str) -> Result<(), ParseError> {
        let re = Regex::new(r"^[A-Z][0-9]*$").unwrap();
        if !re.is_match(new_name) {
            return Err(IncorrectName(
                format!("Incorrect new name format: {}.Must be a capital letter and optional digits.", new_name)
            ));
        }

        if self.points.iter().any(|p| p.name == new_name) {
            return Err(IncorrectName(
                format!("The point name {} is already used in the rectangle.", new_name)
            ));
        }

        if let Some(point) = self.points.iter_mut().find(|p| p.name == old_name) {
            point.name = new_name.to_string();
            self.name = self.points.iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join("");
            Ok(())
        } else {
            Err(IncorrectName(
                format!("Point {} not found.", old_name)
            ))
        }
    }
}

pub fn parse(source: &str, path: &PathBuf) -> Result<(), ParseError> {
    //println!("source: {:?}", source);
    let processed = Lexer::new(source).process();
    //println!("processed: {:?}", processed);

    let mut rectangles = HashMap::new();
    let mut last_rectangle: Option<String> = None;

    let pairs = GrammarParser::parse(Rule::statement_list, &processed)
        .map_err(|e| IncorrectInput(format!("Parse error: {}", e)))?;
    //println!("Parsed pairs: {:?}", pairs);

    fn get_rectangle_name(
        pairs: &mut Pairs<Rule>,
        last_rectangle: &Option<String>,
    ) -> Result<String, ParseError> {
        match pairs.clone().next() {
            Some(p) if p.as_rule() == Rule::rectangle_name => {
                pairs.next();
                Ok(p.as_str().to_string())
            },
            Some(_) => last_rectangle.clone().ok_or_else(|| {
                RectangleNotFound("No rectangle specified.".to_string())
            }),
            None => last_rectangle.clone().ok_or_else(|| {
                RectangleNotFound("No rectangle specified.".to_string())
            }),
        }
    }

    fn parse_coordinate_pair(pair: Pair<Rule>) -> Result<(i32, i32), ParseError> {
        if pair.as_rule() != Rule::coordinate_pair && pair.as_rule() != Rule::vector {
            return Err(IncorrectInput(
                format!(
                    "Expected {:?}, but found a {:?}",
                    Rule::coordinate_pair,
                    pair.as_rule()
                )
                .to_string(),
            ));
        }
        let mut coords = pair.into_inner();
        let x = coords
            .next()
            .and_then(|p| p.as_str().parse::<i32>().ok())
            .ok_or_else(|| {
                IncorrectInput("Incorrect X coordinate format.".to_string())
            })?;
        let y = coords
            .next()
            .and_then(|p| p.as_str().parse::<i32>().ok())
            .ok_or_else(|| {
                IncorrectInput("Incorrect Y coordinate format.".to_string())
            })?;
        Ok((x, y))
    }
    fn parse_coordinate_list(coordinate_list: Pair<Rule>) -> Result<Vec<(i32, i32)>, ParseError> {
        if coordinate_list.as_rule() != Rule::coordinate_list {
            return Err(IncorrectInput(
                format!(
                    "Expected {:?}, but was found a {:?}",
                    Rule::coordinate_list,
                    coordinate_list.as_rule()
                )
                .to_string(),
            ));
        }
        let mut coordinates = Vec::new();
        for pair in coordinate_list.into_inner() {
            match pair.as_rule() {
                Rule::coordinate_pair => {
                    coordinates.push(parse_coordinate_pair(pair)?);
                }
                _ => {
                    return Err(IncorrectInput(
                        "Incorrect coordinate format.".to_string(),
                    ))
                }
            }
        }
        Ok(coordinates)
    }
    display_parse_tree(pairs.clone(), 0);
    for pair in pairs {
        //println!("{:?}", pair.as_rule());
        match pair.as_rule() {
            Rule::define_rectangle => {
                let mut inner = pair.into_inner();
                let name = get_rectangle_name(&mut inner, &last_rectangle)?;
                if let Some(coords_list) = inner.next() {
                    match coords_list.as_rule() {
                        Rule::coordinate_list => {
                            let coords = parse_coordinate_list(coords_list)?;
                            let rect = Rectangle::new_from_coords(&name, &coords)?;
                            rectangles.insert(name.clone(), rect);
                            last_rectangle = Some(name.clone());
                        }
                        Rule::size_define => {
                            let mut params_inner = coords_list.into_inner();
                            let height = params_inner
                                .find(|p| p.as_rule() == Rule::height)
                                .and_then(|p| p.into_inner().next())
                                .and_then(|p| p.as_str().parse::<i32>().ok())
                                .ok_or_else(|| IncorrectInput("Incorrect height".to_string()))?;

                            let width = params_inner
                                .find(|p| p.as_rule() == Rule::width)
                                .and_then(|p| p.into_inner().next())
                                .and_then(|p| p.as_str().parse::<i32>().ok())
                                .ok_or_else(|| IncorrectInput("Incorrect width".to_string()))?;

                            let (x, y) = if let Some(coord_pair) = inner.next() {
                                parse_coordinate_pair(coord_pair)?
                            } else {
                                (0, 0)
                            };

                            let rect = Rectangle::new(x, y, width, height, &name)?;
                            rectangles.insert(name.clone(), rect);
                            last_rectangle = Some(name.clone());
                        }
                        _ => {},
                    }
                } else {
                    let rect = Rectangle::new(0, 0, 10, 20, &name)?;
                    rectangles.insert(name.clone(), rect);
                    last_rectangle = Some(name.clone());
                }
            }
            Rule::calculate_area => {
                let name = get_rectangle_name(&mut pair.into_inner(), &last_rectangle)?;
                let rect = rectangles
                    .get_mut(&name)
                    .ok_or(RectangleNotFound(format!("Rectangle {} not found", name.clone())))?;
                last_rectangle = Some(name.clone());
                println!("Area of rectangle {}: {}", name, rect.area());
            }
            Rule::calculate_perimeter => {
                let name = get_rectangle_name(&mut pair.into_inner(), &last_rectangle)?;
                let rect = rectangles
                    .get_mut(&name)
                    .ok_or(RectangleNotFound(format!("Rectangle {} not found", name.clone())))?;
                last_rectangle = Some(name.clone());
                println!("Perimeter of rectangle {}: {}", name, rect.perimeter());
            }
            Rule::build_diagonal => {
                let name = get_rectangle_name(&mut pair.into_inner(), &last_rectangle)?;
                let rect = rectangles
                    .get_mut(&name)
                    .ok_or(RectangleNotFound(format!("Rectangle {} not found", name.clone())))?;
                last_rectangle = Some(name.clone());
                println!("Diagonal length of rectangle {}: {}", name, rect.diagonal());
            }
            Rule::move_by_rectangle => {
                let mut inner = pair.into_inner();

                let name = get_rectangle_name(&mut inner, &last_rectangle)?;
                let vector = inner
                    .next()
                    .filter(|p| p.as_rule() == Rule::vector)
                    .ok_or_else(|| IncorrectInput("The movement vector not found.".to_string()))?;

                let (dx, dy) = parse_coordinate_pair(vector)?;
                let rect = rectangles
                    .get_mut(&name)
                    .ok_or(RectangleNotFound(format!("Rectangle {} not found", name.clone())))?;
                last_rectangle = Some(name.clone());
                rect.move_rectangle(dx, dy);
            }
            Rule::move_to_rectangle => {
                let mut inner = pair.into_inner();

                let name = get_rectangle_name(&mut inner, &last_rectangle)?;

                let vector = inner
                    .next()
                    .filter(|p| p.as_rule() == Rule::vector)
                    .ok_or_else(|| IncorrectInput("The movement vector not found.".to_string()))?;

                let (dx, dy) = parse_coordinate_pair(vector)?;
                let rect = rectangles
                    .get_mut(&name)
                    .ok_or(RectangleNotFound(format!("Rectangle {} not found", name.clone())))?;
                last_rectangle = Some(name.clone());
                rect.move_to(dx, dy);
            }
            Rule::resize_rectangle => {
                let mut inner = pair.into_inner();
                let name = get_rectangle_name(&mut inner, &last_rectangle)?;
                let factor = inner
                    .next()
                    .filter(|p| p.as_rule() == Rule::coefficient)
                    .and_then(|p| p.as_str().parse::<f32>().ok())
                    .ok_or_else(|| IncorrectInput("The coefficient factor not found.".to_string()))?;

                let rect = rectangles
                    .get_mut(&name)
                    .ok_or(RectangleNotFound(format!("Rectangle {} not found", name.clone())))?;
                last_rectangle = Some(name.clone());
                rect.resize_rectangle(factor);
            }
            Rule::rotate_rectangle => {
                let mut inner = pair.into_inner();
                let name = get_rectangle_name(&mut inner, &last_rectangle)?;
                let angle = inner
                    .next()
                    .and_then(|p| p.into_inner().next().unwrap().as_str().parse::<f32>().ok())
                    .ok_or_else(|| IncorrectInput("The angle not found or incorrect.".to_string()))?;

                let center = inner
                    .next()
                    .filter(|p| p.as_rule() == Rule::center_point)
                    .ok_or_else(|| IncorrectInput("The rotate center not found.".to_string()))?;

                let center_value = center.into_inner().next().unwrap();

                let rect = rectangles
                    .get_mut(&name)
                    .ok_or(RectangleNotFound(format!("Rectangle {} not found", name.clone())))?;

                let (center_x, center_y) = match center_value.as_rule() {
                    Rule::coordinate_pair => parse_coordinate_pair(center_value)?,
                    Rule::point_name => {
                        let point = rect.points.iter().find(|p| p.name.eq(center_value.as_str()));
                        match point {
                            Some(point) => (point.x, point.y),
                            None => return Err(IncorrectInput("Point not found.".to_string()))
                        }
                    },
                    _ => return Err(IncorrectInput("Неправильний формат центру обертання".to_string()))
                };

                last_rectangle = Some(name.clone());
                rect.rotate_rectangle(angle, center_x, center_y);

            }
            Rule::reflect_rectangle => {
                let mut inner = pair.into_inner();
                let name = get_rectangle_name(&mut inner, &last_rectangle)?;

                let axis = inner
                    .next()
                    .filter(|p| p.as_rule() == Rule::axis)
                    .and_then(|p| p.as_str().chars().next())
                    .ok_or_else(|| IncorrectInput("The axis not found or incorrect.".to_string()))?;

                let rect = rectangles
                    .get_mut(&name)
                    .ok_or(RectangleNotFound(format!("Rectangle {} not found", name.clone())))?;
                last_rectangle = Some(name.clone());
                rect.reflect_rectangle(axis);
            },
            Rule::build_rectangle_with_ratio => {
                let mut inner = pair.into_inner();
                let name = get_rectangle_name(&mut inner, &last_rectangle)?;

                let mut ratio_pair = inner
                    .next()
                    .ok_or(IncorrectInput("The ratio not found or incorrect.".to_string()))?.into_inner();
                let ratio = (ratio_pair.next().unwrap().as_str().parse::<u32>().unwrap(), ratio_pair.next().unwrap().as_str().parse::<u32>().unwrap());

                let mut length_pair = inner
                    .next()
                    .filter(|p| p.as_rule() == Rule::length)
                    .ok_or(IncorrectInput("The length not found or incorrect.".to_string()))?.into_inner();
                let length = length_pair.next().unwrap().as_str().parse::<u32>().unwrap();

                let coordinate_pair = inner
                    .next()
                    .filter(|p| p.as_rule() == Rule::coordinate_pair)
                    .ok_or_else(|| IncorrectInput("The coordinate pair not found.".to_string()))?;
                let (dx, dy) = parse_coordinate_pair(coordinate_pair)?;

                let rect = Rectangle::new_with_ratio(&name, ratio, length, dx, dy)?;
                last_rectangle = Some(name.clone());
                rectangles.insert(name.clone(), rect);
            },
            Rule::mark_intersection => {
                let mut inner = pair.into_inner();

                let name1 = match inner.next() {
                    Some(p) if p.as_rule() == Rule::rectangle_name => p.as_str().to_string(),
                    Some(p) => Err(IncorrectInput(format!("The rectangle name was expected, but found {:?}..", p.as_rule()).to_string()))?,
                    None => return Err(IncorrectInput("The rectangle name not found.".to_string()))
                };
                let name2 = get_rectangle_name(&mut inner, &last_rectangle)?;
                let intersection = Rectangle::intersection(rectangles.get(&name1).unwrap(), rectangles.get(&name2).unwrap());
                match intersection {
                    None => {
                        println!("Intersection of rectangles {} and {} not found", name1, name2);
                    }
                    Some(i) => {
                        rectangles.insert(i.name.clone(), i);
                    }
                }
            },
            Rule::rename_point => {
                let mut inner = pair.into_inner();
                let name = get_rectangle_name(&mut inner, &last_rectangle)?;

                let mut rect = rectangles.remove(&name)
                    .ok_or_else(|| IncorrectInput(format!("Rectangle {} not found.", name)))?;

                let p1 = inner
                    .next()
                    .filter(|p| p.as_rule() == Rule::point_name)
                    .ok_or_else(|| IncorrectInput("The point name was not found.".to_string()))?.as_str().to_string();

                if !rect.has_point(&p1) {
                    return Err(IncorrectInput(format!("The point {} was not found in rectangle.", p1).to_string()));
                }

                let p2 = inner
                    .next()
                    .filter(|p| p.as_rule() == Rule::point_name)
                    .ok_or_else(|| IncorrectInput("The point name was not found.".to_string()))?.as_str().to_string();

                match rect.rename_point(&p1, &p2) {
                    Ok(()) => {
                        rectangles.insert(rect.name.clone(), rect);
                        println!("Successfully renamed the point.")
                    },
                    Err(e) => println!("Rename error: {:?}", e),
                }
            }

            _ => {}
        }
    }
    let rectangles_vec: Vec<Rectangle> = rectangles.values().cloned().collect();
    save_rectangles_to_json(&rectangles_vec, path).unwrap();
    Ok(())
}
fn save_rectangles_to_json(rectangles: &Vec<Rectangle>, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::create(path.join("rectangles.json"))?;
    to_writer(file, &rectangles)?;
    Ok(())
}
fn display_parse_tree(pairs: Pairs<Rule>, depth: usize) {
    for pair in pairs {
        let indent = "  ".repeat(depth);
        println!("{} {:?}", indent, pair.as_rule());

        if pair.clone().into_inner().count() > 0 {
            display_parse_tree(pair.into_inner(), depth + 2);
        } else {
            println!("{}  {:?}", indent, pair.as_str());
        }
    }
}