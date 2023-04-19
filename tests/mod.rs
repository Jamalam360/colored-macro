// TODO: Use real tests somehow

#[test]
fn basic() {
    let name = "James";
    println!("{}", colored_macro::colored!("hi <yellow>H<red>e<bold>l</bold></red>lo!</yellow> <green>My name is</green> <blue>{name}</blue> <bold>{}</bold> no style!", name));
    println!("{}", colored_macro::colored!("<red>all red <yellow>yellow <blue>now to blue</blue> back to yellow</yellow> back to red</red>"));
    println!(
        "{}",
        colored_macro::colored!("<blink>blinking! <underline>underlined!</underline></blink>")
    );
}
