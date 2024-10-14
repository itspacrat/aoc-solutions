//
/// alias  the type we'll use to count scores in
/// case it needs to be changed later
type ScoreType = u16;
//
/// return the sum of scores for an input
fn out_total(input: Vec<Vec<ScoreType>>) -> ScoreType {
    let scores = input.iter().map(|g| game(g[1], g[0])).collect::<Vec<ScoreType>>();
    let total: ScoreType = {
        let mut total_sum = 0;
        for i in scores.iter() {
            total_sum += *i
        }
        total_sum
    };
    total
}
//
/// converts an &str to RPS ScoreType (use with `map()`)
fn to_scoretype<'a>(game_shape: &'a str) -> ScoreType {
    match game_shape {
        "A" | "X" => 1 as ScoreType,
        "B" | "Y" => 2 as ScoreType,
        "C" | "Z" => 3 as ScoreType,
        _ => 0 as ScoreType,
    }
}
/// convert the player's shape to the proper shape
/// to meet the desired outcome (part 2)
fn game_2<'a>(you: &'a str, them: &'a str) -> Vec<&'a str> {
    let mut out = vec![them];
    match you {
        "X" => match them {
            "A" => out.push("Z"),
            "B" => out.push("X"),
            "C" => out.push("Y"),
            _ => out.push(you),
        },
        "Y" => match them {
            "A" => out.push("X"),
            "B" => out.push("Y"),
            "C" => out.push("Z"),
            _ => out.push(you),
        },
        "Z" => match them {
            "A" => out.push("Y"),
            "B" => out.push("Z"),
            "C" => out.push("X"),
            _ => out.push(you),
        },
        _ => out.push(you),
    }
    out
}
//
/// compare 2 shape scores and return the
/// resulting final score as a ScoreType
fn game(you: ScoreType, them: ScoreType) -> ScoreType {
    if you == them {
        you + 3
    } else {
        match you {
            1 => {
                if them == 2 {
                    you
                } else {
                    you + 6
                }
            }
            2 => {
                if them == 3 {
                    you
                } else {
                    you + 6
                }
            }
            3 => {
                if them == 1 {
                    you
                } else {
                    you + 6
                }
            }
            _ => you,
        }
    }
}
fn main() {
    //
    // store input as a String
    let input_s = std::fs::read_to_string("in.txt").unwrap().to_owned();
    //
    // convert input to a Vec<Vec<ScoreType>>
    let mut input = input_s
        .split("\n")
        .collect::<Vec<_>>()
        .to_owned()
        .iter()
        .map(|g| g.split(" ").map(|s| to_scoretype(s)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .to_owned();
    //
    // remove malformed end line
    input.pop();

    //
    // fix the shapes you play on part 2
    let mut input_2 = input_s
        .split("\n")
        .collect::<Vec<_>>()
        .to_owned()
        .iter()
        .map(|s| s.split(" ").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    input_2.pop();
    let input_2 = input_2.iter().map(|game| game_2(game[1],game[0])).collect::<Vec<_>>()
    .iter().map(|g| g.iter().map(|gs|
        to_scoretype(gs)
    ).collect::<Vec<_>>()).collect::<Vec<_>>();

    

    println!("\npart 1: {},\npart 2: {}\n",out_total(input),out_total(input_2));
}
