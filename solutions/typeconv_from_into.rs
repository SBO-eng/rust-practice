use std::fs;
use std::io;
use std::num;

fn main() {
    // 1
    {
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);

        // FIX #1: char -> u32 has From<char>
        let i3: u32 = 'a'.into();
        assert_eq!(i3, 97);

        // FIX #2: char -> String (две нормальные опции)
        let s: String = 'a'.to_string();
        assert_eq!(s, "a");

        println!("from/into #1 OK");
    }

    // 2
    {
        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(value: i32) -> Self {
                Number { value }
            }
        }

        let num = Number::from(30);
        assert_eq!(num.value, 30);

        let num: Number = 30.into();
        assert_eq!(num.value, 30);

        println!("from/into #2 OK");
    }

    // 3 (From for errors)
    {
        enum CliError {
            IoError(io::Error),
            ParseError(num::ParseIntError),
        }

        impl From<io::Error> for CliError {
            fn from(e: io::Error) -> Self {
                CliError::IoError(e)
            }
        }
        impl From<num::ParseIntError> for CliError {
            fn from(e: num::ParseIntError) -> Self {
                CliError::ParseError(e)
            }
        }

        fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
            let contents = fs::read_to_string(&file_name)?;
            let num: i32 = contents.trim().parse()?;
            Ok(num)
        }

        let _ = open_and_parse_file; // просто чтобы функция “использовалась”
        println!("from/into #3 OK");
    }

    // 4 (TryInto)
    {
        let n: i16 = 256;
        let n: u8 = match n.try_into() {
            Ok(n) => n,
            Err(e) => {
                println!(
                    "there is an error when converting: {:?}, but we catch it",
                    e.to_string()
                );
                0
            }
        };

        assert_eq!(n, 0);
        println!("from/into #4 OK");
    }

    // 5 (TryFrom / TryInto)
    {
        use std::convert::TryFrom;

        #[derive(Debug, PartialEq)]
        struct EvenNum(i32);

        impl TryFrom<i32> for EvenNum {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNum(value))
                } else {
                    Err(())
                }
            }
        }

        assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
        assert_eq!(EvenNum::try_from(5), Err(()));

        let result: Result<EvenNum, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNum(8)));

        let result: Result<EvenNum, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));

        println!("from/into #5 OK");
    }

    println!("typeconv_from_into: ALL OK ✅");
}
