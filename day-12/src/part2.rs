use std::{str::FromStr, collections::HashMap, iter::repeat};

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone)]
struct ConditionRecord {
    row: String,
    count: Vec<usize>
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    row_position: usize,
    count_position: usize,
    num_of_consecutive_broken_springs: usize, // what is this???
    expecting_working: bool
}

pub fn process<'a>(
    input: &'a str,
) -> usize {

    parse_input(input).map(count_possible).sum()
}

fn count_possible(condition_record: ConditionRecord) -> usize {
    let mut total_possible_count = 0;

    // construct a finite state machine which can take multiple states simultaneously
    let mut current_states = HashMap::from([(State{row_position: 0, count_position: 0, num_of_consecutive_broken_springs: 0, expecting_working: false}, 1)]);
    let mut next_states = HashMap::<State, usize>::new();


    while current_states.len() > 0 {
        // loop around the current states
        for (state, possible_count) in current_states {
            // if we're at the end of the row then we're done with this state
            if at_end_of_row(&condition_record, &state) {
                if at_end_of_count(&condition_record, &state) {
                    // if we're also at the end of the counts then we're onto a winner
                    total_possible_count += possible_count;
                }
                continue;
            }
            
            if looking_for_damaged_springs(&condition_record, &state) {
                // we are still looking for broken springs

                if in_run_of_damaged_springs(&condition_record, &state) {
                    // we are not in a run of broken springs, so this '?' might be working
                    let mut next_state = state.clone();
                    next_state.row_position += 1;
                    insert_next_state(&mut next_states, next_state, possible_count);
                }

                // it might also be broken
                let mut next_state = state.clone();
                next_state.num_of_consecutive_broken_springs += 1;
                next_state.row_position += 1;
            
                if at_end_of_run_of_damaged_springs(&condition_record, &next_state) {
                    // we have found the end of the run of broken springs
                    // move to next count
                    next_state.count_position += 1;
                    // reset the broken spring counter
                    next_state.num_of_consecutive_broken_springs = 0;
                    // the next spring has to be working
                    next_state.expecting_working = true;
                }

                insert_next_state(&mut next_states, next_state, possible_count);
            } else if not_at_end_of_run_of_damaged_springs(&condition_record, &state) {
                let mut next_state = state.clone();
                next_state.row_position += 1;
                next_state.expecting_working = false;

                insert_next_state(&mut next_states, next_state, possible_count);
            }

        }
        current_states = next_states;
        next_states = HashMap::<State, usize>::new();
    }
    total_possible_count
}

fn at_end_of_row(condition_record: &ConditionRecord, state: &State) -> bool {
    state.row_position == condition_record.row.len()
}

fn at_end_of_count(condition_record: &ConditionRecord, state: &State) -> bool {
    state.count_position == condition_record.count.len()
}

fn looking_for_damaged_springs(condition_record: &ConditionRecord, state: &State) -> bool {
    (condition_record.row.chars().nth(state.row_position).unwrap() == '#' || condition_record.row.chars().nth(state.row_position).unwrap() == '?') 
        && state.count_position < condition_record.count.len() && ! state.expecting_working 
}

fn insert_next_state(next_states: &mut HashMap<State, usize>, next_state: State, possible_count: usize) {
    let count = next_states.get(&next_state).or(Some(&0)).unwrap();
    next_states.insert(next_state, count + possible_count);
}

fn in_run_of_damaged_springs(condition_record: &ConditionRecord, state: &State) -> bool {
    condition_record.row.chars().nth(state.row_position).unwrap() == '?' && state.num_of_consecutive_broken_springs == 0
}

fn at_end_of_run_of_damaged_springs(condition_record: &ConditionRecord, state: &State) -> bool {
    state.num_of_consecutive_broken_springs == condition_record.count[state.count_position]
}

fn not_at_end_of_run_of_damaged_springs(condition_record: &ConditionRecord, state: &State) -> bool {

    (condition_record.row.chars().nth(state.row_position).unwrap() ==  '.' || condition_record.row.chars().nth(state.row_position).unwrap() == '?' )
        && state.num_of_consecutive_broken_springs == 0 
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = ConditionRecord> + 'a {
    input.lines().map(parse_line)
}

fn parse_line(line: &str) -> ConditionRecord {
    let (row, count_string) = line.split_once(' ').expect("the input to be correct");
    ConditionRecord {
        row: repeat(row).take(5).join("?"),
        count: repeat(count_string.split(',').map(usize::from_str).map(Result::unwrap)).take(5).flatten().collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    #[test]
    fn test_line(
        #[case] input: &str,
        #[case] expected: usize
    ) {

        assert_eq!(count_possible(parse_line(input)), expected);
    }

    #[test]
    fn test_process() {
        let lines = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let result = process(lines);
        assert_eq!(result, 525152);
    }
}
