use libm::{atan2, cos, fmax, fmin, pow, sin, sqrt, sqrtf};
use std::any::Any;
use std::cmp::max;
use std::collections::HashMap;
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RectTraitKey {
    Perimeter,
    Area,
    Diagonal,
    DiagonalDiagonalAngle,
    SideXDiagonalAngle,
    SideYDiagonalAngle,
    SmallerSide,
    BiggerSide,
    Ratio,
    SideDistances,
    CircumscribedCircleRadius,
    CircumscribedCircleDiameter,
    CircumscribedCircleArea,
    CircumscribedCirclePerimeter,
    CircleRectRatio,
}

#[derive(Debug, Clone)]
pub enum RectTraitValue {
    Single(f64),
    Pair(f64, f64),
}


#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub traits: HashMap<RectTraitKey, RectTraitValue>,
}
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle {
            width,
            height,
            traits: HashMap::new(),
        }
    }

    pub fn has_trait(&self, key: RectTraitKey) -> bool {
        self.traits.contains_key(&key)
    }

    pub fn get_trait_value(&self, key: &RectTraitKey) -> Option<&RectTraitValue> {
        self.traits.get(key)
    }

    pub fn perimeter(&mut self) -> f64 {
        let value = 2.0 * (self.width + self.height);
        self.traits
            .insert(RectTraitKey::Perimeter, RectTraitValue::Single(value));
        value
    }

    pub fn area(&mut self) -> f64 {
        let value = self.width * self.height;
        self.traits
            .insert(RectTraitKey::Area, RectTraitValue::Single(value));
        value
    }

    pub fn diagonal(&mut self) -> f64 {
        let value = (self.width.powi(2) + self.height.powi(2)).sqrt();
        self.traits
            .insert(RectTraitKey::Diagonal, RectTraitValue::Single(value));
        value
    }

    pub fn find_diagonal_by_area(&mut self) {
        if let Some(RectTraitValue::Single(s)) = self.traits.get(&RectTraitKey::Area) {
            if self.width <= 0.0 && self.height <= 0.0 {
                return;
            }
            let side = if self.width > 0.0 {
                self.width
            } else {
                self.height
            };
            let d = (sqrt(pow(*s, 2.) + pow(side, 4.))) / side;
            self.traits
                .insert(RectTraitKey::Diagonal, RectTraitValue::Single(d));
        }
    }

    pub fn find_diagonal_by_perimeter(&mut self) {
        if let Some(RectTraitValue::Single(p)) = self.traits.get(&RectTraitKey::Perimeter) {
            if self.width <= 0.0 && self.height <= 0.0 {
                return;
            }
            let side = if self.width > 0.0 {
                self.width
            } else {
                self.height
            };
            let d = (sqrt(pow(*p, 2.) - (4. * p * side) + 8. * pow(side, 2.))) / 2.;
            self.traits
                .insert(RectTraitKey::Diagonal, RectTraitValue::Single(d));
        }
    }

    pub fn find_perimeter_by_area(&mut self) {
        if let Some(RectTraitValue::Single(s)) = self.traits.get(&RectTraitKey::Area) {
            if self.width <= 0.0 && self.height <= 0.0 {
                return;
            }
            let side = if self.width > 0.0 {
                self.width
            } else {
                self.height
            };
            let p = (2. * s + 2. * pow(side, 2.)) / side;
            self.traits
                .insert(RectTraitKey::Perimeter, RectTraitValue::Single(p));
        }
    }

    pub fn find_perimeter_by_diagonal(&mut self) {
        if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::Diagonal) {
            if self.width <= 0.0 && self.height <= 0.0 {
                return;
            }
            let side = if self.width > 0.0 {
                self.width
            } else {
                self.height
            };
            let p = 2. * (side + sqrt(pow(*d, 2.) - pow(side, 2.)));
            self.traits
                .insert(RectTraitKey::Perimeter, RectTraitValue::Single(p));
        }
    }

    pub fn find_area_by_perimeter(&mut self) {
        if let Some(RectTraitValue::Single(p)) = self.traits.get(&RectTraitKey::Perimeter) {
            if self.width <= 0.0 && self.height <= 0.0 {
                return;
            }
            let side = if self.width > 0.0 {
                self.width
            } else {
                self.height
            };
            let s = (p * side - (2. * pow(side, 2.))) / 2.;
            self.traits
                .insert(RectTraitKey::Area, RectTraitValue::Single(s));
        }
    }

    pub fn find_area_by_diagonal(&mut self) {
        if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::Diagonal) {
            if self.width <= 0.0 && self.height <= 0.0 {
                return;
            }
            let side = if self.width > 0.0 {
                self.width
            } else {
                self.height
            };
            let s = side * sqrt(pow(*d, 2.) - pow(side, 2.));
            self.traits
                .insert(RectTraitKey::Area, RectTraitValue::Single(s));
        }
    }

    pub fn find_side_by_perimeter(&mut self) {
        let p = match self.traits.get(&RectTraitKey::Perimeter) {
            Some(RectTraitValue::Single(p)) => *p,
            _ => return,
        };
        if self.width > 0.0 && self.height > 0.0 {
            return;
        }
        if self.width > 0.0 {
            self.height = (p - 2. * self.width) / 2.;
        }
        if self.height > 0.0 {
            self.width = (p - 2. * self.height) / 2.;
        }
    }
    pub fn find_side_by_area(&mut self) {
        if let Some(RectTraitValue::Single(s)) = self.traits.get(&RectTraitKey::Area) {
            if self.width > 0.0 && self.height > 0.0 {
                return;
            }
            if self.width > 0.0 {
                self.height = s / self.width;
            }
            if self.height > 0.0 {
                self.width = s / self.height;
            }
        }
    }
    pub fn find_side_by_diagonal(&mut self) {
        if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::Diagonal) {
            if self.width > 0.0 && self.height > 0.0 {
                return;
            }
            if self.width > 0.0 {
                self.height = sqrt(pow(*d, 2.) - pow(self.width, 2.));
            }
            if self.height > 0.0 {
                self.width = sqrt(pow(*d, 2.) - pow(self.height, 2.));
            }
        }
    }
    pub fn find_sides_from_area_and_diagonal(&mut self) {
        let area = match self.traits.get(&RectTraitKey::Area) {
            Some(RectTraitValue::Single(s)) => *s,
            _ => return,
        };

        let diagonal = match self.traits.get(&RectTraitKey::Diagonal) {
            Some(RectTraitValue::Single(d)) => *d,
            _ => return,
        };
        let a = 1.0;
        let b = -diagonal.powi(2);
        let c = area.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        println!("{:?}", discriminant);
        if discriminant >= 0.0 {
            let width = ((-b + discriminant.sqrt()) / (2.0 * a)).sqrt();
            let height = area / width;

            self.width = width;
            self.height = height;
        }
    }
    pub fn find_diagonal_by_angle_and_side(&mut self) {
        if let Some(RectTraitValue::Single(angle)) = self.traits.get(&RectTraitKey::DiagonalDiagonalAngle) {
            let angle_rad = angle.to_radians();

            if let Some(RectTraitValue::Single(side)) = self.traits.get(&RectTraitKey::SmallerSide) {
                let diagonal = side / sin(angle_rad / 2.0);
                self.traits.insert(RectTraitKey::Diagonal, RectTraitValue::Single(diagonal));
            }
            else if let Some(RectTraitValue::Single(side)) = self.traits.get(&RectTraitKey::BiggerSide) {
                let diagonal = side / cos(angle_rad / 2.0);
                self.traits.insert(RectTraitKey::Diagonal, RectTraitValue::Single(diagonal));
            }
        }
    }
    pub fn find_diagonal_by_diagonals_angle_and_area(&mut self) {
        let area = match self.traits.get(&RectTraitKey::Area) {
            Some(RectTraitValue::Single(s)) => *s,
            _ => return,
        };

        let angle = match self.traits.get(&RectTraitKey::DiagonalDiagonalAngle) {
            Some(RectTraitValue::Single(a)) => *a,
            _ => return,
        };

        let angle_rad = angle.to_radians();
        let diagonal = (2.0 * area * angle_rad.sin()).sqrt();
        self.traits.insert(RectTraitKey::Diagonal, RectTraitValue::Single(diagonal));
    }

    pub fn calculate_side_distances(&mut self) {
        let d1 = self.width / 2.0;
        let d2 = self.height / 2.0;
        self.traits
            .insert(RectTraitKey::SideDistances, RectTraitValue::Pair(d1, d2));
    }

    pub fn find_sides_by_side_distances(&mut self) {
        if let Some(RectTraitValue::Pair(d1, d2)) = self.traits.get(&RectTraitKey::SideDistances) {
            self.width = d1 * 2.0;
            self.height = d2 * 2.0;
        }
    }

    pub fn angle_between_side_and_diagonal(&mut self) {
        if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::Diagonal) {
            let angle_x = (self.width / d).asin().to_degrees();
            let angle_y = (self.height / d).asin().to_degrees();

            self.traits.insert(
                RectTraitKey::SideXDiagonalAngle,
                RectTraitValue::Single(angle_x),
            );
            self.traits.insert(
                RectTraitKey::SideYDiagonalAngle,
                RectTraitValue::Single(angle_y),
            );
        }
    }

    pub fn angle_between_diagonals(&mut self) {
        if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::Diagonal) {
            if let Some(RectTraitValue::Single(s)) = self.traits.get(&RectTraitKey::Area) {
                let tan_angle = (2.*s)/pow(*d,2.);
                let angle = atan2(tan_angle, 1.0).to_degrees();
                self.traits.insert(
                    RectTraitKey::DiagonalDiagonalAngle,
                    RectTraitValue::Single(angle),
                );
            }
        }
    }

    pub fn find_sides_from_diagonal_and_ratio(&mut self, ratio: (f64, f64)) {
        let scale = (ratio.0.powi(2) + ratio.1.powi(2)).sqrt();
        if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::Diagonal) {
            self.width = d * (ratio.0 / scale);
            self.height = d * (ratio.1 / scale);
        }
    }

    pub fn find_sides_from_perimeter_and_ratio(&mut self, ratio: (f64, f64)) {
        let scale = ratio.0 + ratio.1;
        if let Some(RectTraitValue::Single(p)) = self.traits.get(&RectTraitKey::Perimeter) {
            self.width = p * (ratio.0 / scale) / 2.0;
            self.height = p * (ratio.1 / scale) / 2.0;
        }
    }

    pub fn find_sides_from_one_side_and_ratio(&mut self, ratio: (f64, f64)) {
        let mut fraction_value = 0.;
        if let Some(RectTraitValue::Single(x)) = self.traits.get(&RectTraitKey::BiggerSide) {
            fraction_value = x / fmax(ratio.0, ratio.1);
        }
        if let Some(RectTraitValue::Single(x)) = self.traits.get(&RectTraitKey::SmallerSide) {
            fraction_value = x / fmin(ratio.0, ratio.1);
        }
        self.width = ratio.0 * fraction_value;
        self.height = ratio.1 * fraction_value;
    }
    pub fn find_bigger_smaller_side(&mut self) {
        self.traits.insert(
            RectTraitKey::SmallerSide,
            RectTraitValue::Single(fmin(self.width, self.height)),
        );
        self.traits.insert(
            RectTraitKey::BiggerSide,
            RectTraitValue::Single(fmax(self.width, self.height)),
        );
    }

    pub fn circumscribed_circle_radius(&mut self) {
        if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::Diagonal) {
            let radius = d / 2.0;
            self.traits.insert(
                RectTraitKey::CircumscribedCircleRadius,
                RectTraitValue::Single(radius)
            );
        } else if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::Diagonal) {
            let radius = d / 2.0;
            self.traits.insert(
                RectTraitKey::CircumscribedCircleRadius,
                RectTraitValue::Single(radius)
            );
        }
    }
    pub fn circumscribed_circle_diameter(&mut self) {
        if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::Diagonal) {
            self.traits.insert(
                RectTraitKey::CircumscribedCircleDiameter,
                RectTraitValue::Single(*d)
            );
        } else if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::CircumscribedCircleRadius) {
            self.traits.insert(
                RectTraitKey::CircumscribedCircleDiameter,
                RectTraitValue::Single(*d*2.)
            );
        }
    }

    pub fn diagonal_from_radius_or_diameter(&mut self) {
        if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::CircumscribedCircleDiameter) {
            self.traits.insert(
                RectTraitKey::Diagonal,
                RectTraitValue::Single(*d)
            );
        } else if let Some(RectTraitValue::Single(d)) = self.traits.get(&RectTraitKey::CircumscribedCircleRadius) {
            self.traits.insert(
                RectTraitKey::Diagonal,
                RectTraitValue::Single(*d*2.)
            );
        }
    }

    pub fn circumscribed_circle_area(&mut self) {
        if let Some(RectTraitValue::Single(r)) = self.traits.get(&RectTraitKey::CircumscribedCircleRadius) {
            let area = std::f64::consts::PI * r.powi(2);
            self.traits.insert(
                RectTraitKey::CircumscribedCircleArea,
                RectTraitValue::Single(area)
            );
        }
    }

    pub fn circumscribed_circle_perimeter(&mut self) {
        if let Some(RectTraitValue::Single(r)) = self.traits.get(&RectTraitKey::CircumscribedCircleRadius) {
            let perimeter = 2.0 * std::f64::consts::PI * r;
            self.traits.insert(
                RectTraitKey::CircumscribedCirclePerimeter,
                RectTraitValue::Single(perimeter)
            );
        }
    }
    pub fn find_radius_by_circle_area(&mut self) {
        if let Some(RectTraitValue::Single(area)) = self.traits.get(&RectTraitKey::CircumscribedCircleArea) {
            let radius = (area / std::f64::consts::PI).sqrt();
            self.traits.insert(
                RectTraitKey::CircumscribedCircleRadius,
                RectTraitValue::Single(radius)
            );
        }
    }

    pub fn find_radius_by_circle_perimeter(&mut self) {
        if let Some(RectTraitValue::Single(perimeter)) = self.traits.get(&RectTraitKey::CircumscribedCirclePerimeter) {
            let radius = perimeter / (2.0 * std::f64::consts::PI);
            self.traits.insert(
                RectTraitKey::CircumscribedCircleRadius,
                RectTraitValue::Single(radius)
            );
        }
    }
    fn trait_to_json_value(&self, key: &RectTraitKey, value: &RectTraitValue) -> (String, Value) {
        let key_str = match key {
            RectTraitKey::Perimeter => "Perimeter",
            RectTraitKey::Area => "Area",
            RectTraitKey::Diagonal => "Diagonal",
            RectTraitKey::DiagonalDiagonalAngle => "DiagonalDiagonalAngle",
            RectTraitKey::SideXDiagonalAngle => "SideXDiagonalAngle",
            RectTraitKey::SideYDiagonalAngle => "SideYDiagonalAngle",
            RectTraitKey::SmallerSide => "SmallerSide",
            RectTraitKey::BiggerSide => "BiggerSide",
            RectTraitKey::Ratio => "Ratio",
            RectTraitKey::SideDistances => "SideDistances",
            RectTraitKey::CircumscribedCircleRadius => "CircumscribedCircleRadius",
            RectTraitKey::CircumscribedCircleDiameter => "CircumscribedCircleDiameter",
            RectTraitKey::CircumscribedCircleArea => "CircumscribedCircleArea",
            RectTraitKey::CircumscribedCirclePerimeter => "CircumscribedCirclePerimeter",
            RectTraitKey::CircleRectRatio => "CircleRectRatio",
        }.to_string();

        let value_json = match value {
            RectTraitValue::Single(v) => json!(v),
            RectTraitValue::Pair(v1, v2) => json!([v1, v2]),
        };

        (key_str, value_json)
    }

    pub fn to_json_value(&self) -> Value {
        let mut traits_map = serde_json::Map::new();

        for (key, value) in &self.traits {
            let (key_str, value_json) = self.trait_to_json_value(key, value);
            traits_map.insert(key_str, value_json);
        }

        json!({
            "width": self.width,
            "height": self.height,
            "traits": traits_map
        })
    }
}
