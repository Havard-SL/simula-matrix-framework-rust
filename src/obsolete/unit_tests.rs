// Unit tests.
// Not currently being used.
#[cfg(test)]
mod tests {
    use super::super::abelian::*;
    use super::super::abelian::legacy_abelian::*;

    #[test]
    fn test_compare_all_group_generation_old_and_new() {
        let n = 6;

        let groups_old = generate_all_groups(n);
        let groups_new = generate_all_groups_new(n);

        assert_eq!(groups_old, groups_new);
    }

    #[test]
    fn test_compare_sudocurity_group_generation_old_and_new() {
        let n = 6;

        let groups_old = generate_all_sudocurity_groups(n);
        let groups_new = generate_all_sudocurity_groups_new(n);

        assert_eq!(groups_old, groups_new);
    }

    #[test]
    fn test_apply_permutation_to_group() {
        let table = generate_all_sudocurity_groups_new(5);

        let permutation = vec![0, 2, 3, 4, 1];

        let new_t = apply_permutation_to_group(&table[3], &permutation);

        assert_eq!(new_t, table[2]);
    }

    #[test]
    fn test_apply_permutation_to_group_2() {
        let n = 7;

        let groups = generate_all_sudocurity_groups_new(n);
        let permutations = generate_sudocurity_permutations(n);

        for p in permutations.iter() {
            for g in &groups {
                let test = apply_permutation_to_group(g, p);

                assert!(groups.contains(&test))
            }
        }
    }
}