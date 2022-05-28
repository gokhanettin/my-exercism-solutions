use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt;

struct TableEntry<'a> {
    team_name: &'a str,
    matches_played: i32,
    matches_won: i32,
    matches_drawn: i32,
    matches_lost: i32,
    points: i32,
}

struct Table<'a> {
    entries: Vec<TableEntry<'a>>,
}

impl<'a> Table<'a> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    fn add_entry(&mut self, entry: TableEntry<'a>) {
        self.entries.push(entry);
    }

    fn sort(&mut self) {
        self.entries
            .sort_unstable_by_key(|e| (Reverse(e.points), e.team_name));
    }
}

impl<'a> fmt::Display for Table<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}",
            "Team", "MP", "W", "D", "L", "P"
        )?;
        for e in self.entries.iter() {
            write!(
                f,
                "\n{:<31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}",
                e.team_name,
                e.matches_played,
                e.matches_won,
                e.matches_drawn,
                e.matches_lost,
                e.points
            )?;
        }
        Ok(())
    }
}

pub fn tally(match_results: &str) -> String {
    let mut map = HashMap::new();

    for line in match_results.lines() {
        let tokens: Vec<_> = line.split(';').collect();

        if let Some(&result) = tokens.get(2) {
            if result == "win" {
                let (win, _, _) = map.entry(tokens[0]).or_insert((0, 0, 0));
                *win += 1;
                let (_, loss, _) = map.entry(tokens[1]).or_insert((0, 0, 0));
                *loss += 1;
            } else if result == "loss" {
                let (_, loss, _) = map.entry(tokens[0]).or_insert((0, 0, 0));
                *loss += 1;
                let (win, _, _) = map.entry(tokens[1]).or_insert((0, 0, 0));
                *win += 1;
            } else if result == "draw" {
                let (_, _, draw) = map.entry(tokens[0]).or_insert((0, 0, 0));
                *draw += 1;
                let (_, _, draw) = map.entry(tokens[1]).or_insert((0, 0, 0));
                *draw += 1;
            }
        }
    }

    let mut table = Table::with_capacity(map.len());

    for (key, (win, loss, draw)) in map {
        table.add_entry(TableEntry {
            team_name: key,
            matches_played: win + loss + draw,
            matches_won: win,
            matches_drawn: draw,
            matches_lost: loss,
            points: 3 * win + draw,
        });
    }

    table.sort();
    table.to_string()
}
