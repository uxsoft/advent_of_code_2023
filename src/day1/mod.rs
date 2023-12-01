pub fn process(input: String) {
    let result : u32 = 
        input.lines()
        .map(|line|{
            let numbers = line
                .replace("one", "o1ne")
                .replace("two", "t2wo")
                .replace("three", "th3ree")
                .replace("four", "fo4ur")
                .replace("five", "fi5ve")
                .replace("six", "s6ix")
                .replace("seven", "se7ven")
                .replace("eight", "ei8ght")
                .replace("nine", "ni9ne")
                .chars()
                .filter(|char| char.is_ascii_digit())
                .collect::<Vec<_>>();
            let number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
            println!("number: {number}");
            return number.parse::<u32>().unwrap();
        }).sum();

    println!("{result}");
}
