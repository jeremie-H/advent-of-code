
use std::{error::Error};
use md5::{self, Md5, Digest};

/**
 * Part 1
 */
pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    
    let mut hasher = Md5::new();
    let mut position = 0;
    let mut workingstring = input.trim_end().to_string();
    let indexnonce=workingstring.len();
    let mut output: [u8;16]= [0;16];
    loop {
        workingstring.replace_range(indexnonce.., &position.to_string());

        hasher.update(&workingstring);
        hasher.finalize_into_reset((&mut output).into());
        
        if output[0] | output[1] | output[2]>>4 == 0 {
            return Ok(position);
        }
        
        position += 1;
    }
    
}

/**
 * Part 2
 */
pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    
    let mut hasher = Md5::new();
    let mut position = 0;
    let mut workingstring = input.trim_end().to_string();

    let indexnonce=workingstring.len();
    let mut output: [u8;16]= [0;16];
    loop {
        workingstring.replace_range(indexnonce.., &position.to_string());
        
        //hasher.update(&workingstring);
        hasher.update(&workingstring);
        hasher.finalize_into_reset((&mut output).into());
        

        if output[0] | output[1] | output[2] == 0 {
            return Ok(position);
        }
        position += 1;
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // avec les données d'essai
        assert_eq!(part1("abcdef").unwrap(), 609043);
        assert_eq!(part1("pqrstuv").unwrap(), 1048970);
    }

    #[test]
    fn test_part2() {
        // avec les données d'essai
        assert_eq!(part2("").unwrap(), 0);
    }

    #[test]
    fn real_tests() {
        // avec les vraies données
        assert_eq!(part1(include_str!("input.txt")).unwrap(), 346386);
        assert_eq!(part2(include_str!("input.txt")).unwrap(), 9958218);
    }
}
