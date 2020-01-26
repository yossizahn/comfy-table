use comfy_table::prelude::*;

fn get_max_min_table() -> Table {
    let mut table = Table::new();
    table
        .set_header(&vec!["smol", "Header2", "Header3"])
        .add_row(&vec![
            "smol",
            "This is another text",
            "This is the third text",
        ])
        .add_row(&vec![
            "smol",
            "Now\nadd some\nmulti line stuff",
            "This is awesome",
        ]);

    table
}

#[test]
/// Ensure max-, min- and fixed-width constraints are respected
fn fixed_max_min_constraints() {
    let mut table = get_max_min_table();

    table.set_constraints(vec![
        ColumnConstraint::MinWidth(10),
        ColumnConstraint::MaxWidth(8),
        ColumnConstraint::Width(10),
    ]);

    println!("{}", table.to_string());
    let expected = "
+----------+--------+----------+
| smol     | Header | Header3  |
|          | 2      |          |
+==============================+
| smol     | This   | This is  |
|          | is ano | the      |
|          | ther   | third    |
|          | text   | text     |
|----------+--------+----------|
| smol     | Now    | This is  |
|          | add    | awesome  |
|          | some   |          |
|          | multi  |          |
|          | line   |          |
|          | stuff  |          |
+----------+--------+----------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);

    // Now try this again when using dynamic content arrangement
    // The table tries to arrange to 28 characters,
    // but constraints enforce a width of at least 10+10+2+1+4 = 27
    // min_width + max_width + middle_padding + middle_min_width + borders
    // Since the left and right column are fixed, the middle column should only get a width of 2
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(28);

    println!("{}", table.to_string());
    let expected = "
+----------+----+----------+
| smol     | He | Header3  |
|          | ad |          |
|          | er |          |
|          | 2  |          |
+==========================+
| smol     | Th | This is  |
|          | is | the      |
|          | is | third    |
|          | an | text     |
|          | ot |          |
|          | he |          |
|          | r  |          |
|          | te |          |
|          | xt |          |
|----------+----+----------|
| smol     | No | This is  |
|          | w  | awesome  |
|          | ad |          |
|          | d  |          |
|          | so |          |
|          | me |          |
|          | mu |          |
|          | lt |          |
|          | i  |          |
|          | li |          |
|          | ne |          |
|          | st |          |
|          | uf |          |
|          | f  |          |
+----------+----+----------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
/// Max and Min constraints won't be considered, if they are unnecessary
/// This is true for normal and dynamic arrangement tables.
fn unnecessary_max_min_constraints() {
    let mut table = get_max_min_table();

    table.set_constraints(vec![
        ColumnConstraint::MinWidth(1),
        ColumnConstraint::MaxWidth(30),
    ]);

    println!("{}", table.to_string());
    let expected = "
+------+----------------------+------------------------+
| smol | Header2              | Header3                |
+======================================================+
| smol | This is another text | This is the third text |
|------+----------------------+------------------------|
| smol | Now                  | This is awesome        |
|      | add some             |                        |
|      | multi line stuff     |                        |
+------+----------------------+------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);

    // Now test for dynamic content arrangement
    table.set_content_arrangement(ContentArrangement::Dynamic);
    println!("{}", table.to_string());
    let expected = "
+------+----------------------+------------------------+
| smol | Header2              | Header3                |
+======================================================+
| smol | This is another text | This is the third text |
|------+----------------------+------------------------|
| smol | Now                  | This is awesome        |
|      | add some             |                        |
|      | multi line stuff     |                        |
+------+----------------------+------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
/// The user can specify constraints that result in bigger width than actually provided
/// This is allowed, but results in a wider table than acutally aimed for.
/// Anyway we still try to fit everything as good as possible, which of course breaks stuff.
fn constraints_bigger_than_table_width() {
    let mut table = get_max_min_table();

    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(28)
        .set_constraints(vec![
            ColumnConstraint::MaxWidth(50),
            ColumnConstraint::MinWidth(30),
            ColumnConstraint::ContentWidth,
        ]);

    println!("{}", table.to_string());
    let expected = "
+---+------------------------------+------------------------+
| s | Header2                      | Header3                |
| m |                              |                        |
| o |                              |                        |
| l |                              |                        |
+===========================================================+
| s | This is another text         | This is the third text |
| m |                              |                        |
| o |                              |                        |
| l |                              |                        |
|---+------------------------------+------------------------|
| s | Now                          | This is awesome        |
| m | add some                     |                        |
| o | multi line stuff             |                        |
| l |                              |                        |
+---+------------------------------+------------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}

#[test]
/// Test correct usage of the Hidden and Percentage constraint.
/// Hidden columns shouldn't be displayed and not be considered during arrangement
/// Percentage allows to set a fixed width
fn percentage_and_hidden() {
    let mut table = get_max_min_table();

    // Set a percentage of 20% for the first column
    // Hide the second
    // The third should get the remaining 70% automatically
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(30)
        .set_constraints(vec![
            ColumnConstraint::Percentage(20),
            ColumnConstraint::Hidden,
        ]);

    println!("{}", table.to_string());
    let expected = "
+------+---------------------+
| smol | Header2             |
+============================+
| smol | This is another     |
|      | text                |
|------+---------------------|
| smol | Now                 |
|      | add some            |
|      | multi line stuff    |
+------+---------------------+";
    assert_eq!("\n".to_string() + &table.to_string(), expected);
}
