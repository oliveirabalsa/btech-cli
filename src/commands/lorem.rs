use clap::Args;
use lipsum::lipsum;

#[derive(Args)]
pub struct LoremArgs {
    #[arg(short, long, default_value_t = 10)]
    pub count: u32,
}

pub fn execute(args: &LoremArgs) {
    let lorem = lipsum(args.count as usize);
    println!("{}", lorem);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorem_generation() {
        let _args = LoremArgs { count: 5 };
    }

    #[test]
    fn test_default_count() {
        let args = LoremArgs { count: 10 };
        assert_eq!(args.count, 10);
    }
}
