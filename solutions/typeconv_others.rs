use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    // 1) Display -> ToString
    {
        struct Point {
            x: i32,
            y: i32,
        }

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "The point is ({}, {})", self.x, self.y)
            }
        }

        let origin = Point { x: 0, y: 0 };
        assert_eq!(origin.to_string(), "The point is (0, 0)");
        assert_eq!(format!("{}", origin), "The point is (0, 0)");

        println!("others #1 OK");
    }

    // 2) parse + turbofish + from_str
    {
        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();
        let from_str = i32::from_str("20").unwrap();

        let sum = parsed + turbo_parsed + from_str;
        assert_eq!(sum, 35);

        println!("others #2 OK");
    }

    // 3) FromStr for custom type
    {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl FromStr for Point {
            type Err = ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let coords: Vec<&str> = s
                    .trim_matches(|p| p == '(' || p == ')')
                    .split(',')
                    .map(|x| x.trim())
                    .collect();

                let x_fromstr = coords[0].parse::<i32>()?;
                let y_fromstr = coords[1].parse::<i32>()?;

                Ok(Point {
                    x: x_fromstr,
                    y: y_fromstr,
                })
            }
        }

        let p = "(3, 4)".parse::<Point>();
        assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

        println!("others #3 OK");
    }

    println!("typeconv_others: ALL OK ✅");
}
