use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Game {
    Win,
    Draw,
    Loss,
}

fn update_table<'a>(
    teams: &[&'a str],
    res: &[&'a str],
    data: &mut HashMap<&'a str, Vec<u8>>,
    vals: &[&[(u8, u8)]],
) {
    for (idx, team) in teams.iter().enumerate() {
        let cols_vals = vals.get(idx).unwrap();
        let mut v = Vec::from([0, 0, 0, 0, 0]);
        if let Some(t) = data.get_mut(team) {
            t[cols_vals.first().unwrap().0 as usize] += cols_vals.first().unwrap().1;
            t[cols_vals.get(1).unwrap().0 as usize] += cols_vals.get(1).unwrap().1;
            t[cols_vals.get(2).unwrap().0 as usize] += cols_vals.get(2).unwrap().1;
            t[cols_vals.get(3).unwrap().0 as usize] += cols_vals.get(3).unwrap().1;
        } else {
            match *res.get(idx).unwrap() {
                "win" => {
                    v[0] = 1;
                    v[1] = 1;
                    v[4] = 3;
                }
                "draw" => {
                    v[0] = 1;
                    v[2] = 1;
                    v[4] = 1;
                }
                "loss" => {
                    v[0] = 1;
                    v[3] = 1;
                    v[4] = 0;
                }
                _ => {}
            }
            data.insert(team, v);
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut ftable: Vec<String> = Vec::new();
    ftable.push(format!("{:<31}| MP |  W |  D |  L |  P", "Team"));
    if match_results.is_empty() {
        return ftable.join("\n");
    }
    let mut table: HashMap<&str, Vec<u8>> = HashMap::new();
    let draw_vec: &[(u8, u8)] = &[(0, 1), (2, 1), (3, 0), (4, 1)];
    let loss_vec: &[(u8, u8)] = &[(0, 1), (1, 0), (3, 1), (4, 0)];
    let win_vec: &[(u8, u8)] = &[(0, 1), (1, 1), (3, 0), (4, 3)];
    match_results
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .for_each(|mp| {
            let list = mp.split(";").collect::<Vec<_>>();
            match *list.last().unwrap() {
                "win" => {
                    update_table(
                        &list[..2],
                        &["win", "loss"],
                        &mut table,
                        &[win_vec, loss_vec],
                    );
                }
                "draw" => {
                    update_table(
                        &list[..2],
                        &["draw", "draw"],
                        &mut table,
                        &[draw_vec, draw_vec],
                    );
                }
                "loss" => {
                    update_table(
                        &list[..2],
                        &["loss", "win"],
                        &mut table,
                        &[loss_vec, win_vec],
                    );
                }
                _ => {}
            }
        });
    table
        .iter()
        .map(|val| (*val.0, val.1))
        .collect::<Vec<(&str, &Vec<u8>)>>()
        .sort_by(|(key1, vec1), (key2, vec2)| {
            vec2.last().cmp(&vec1.last()).then_with(|| key1.cmp(key2))
        });
    for line in table {
        ftable.push(format!(
            "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            line.0, line.1[0], line.1[1], line.1[2], line.1[3], line.1[4]
        ));
    }
    ftable.join("\n")
}
