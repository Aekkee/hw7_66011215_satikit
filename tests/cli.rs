use assert_cmd::Command;

#[test]
fn sort_test_no_input() {
    let mut cmd1 = Command::cargo_bin("sort").unwrap();
    let mut cmd2 = Command::cargo_bin("sort_bubble").unwrap();

    // test if both sort programs work similarly
    assert_eq!(cmd1.assert().get_output(), cmd2.assert().get_output());

    // test each sort
    cmd1.assert().success().stdout("");
    cmd2.assert().success().stdout("");
}

#[test]
fn sort_test_1() {
    let mut cmd1 = Command::cargo_bin("sort").unwrap();
    let mut cmd2 = Command::cargo_bin("sort_bubble").unwrap();
    cmd1.arg("1").arg("2").arg("3").arg("4");
    cmd2.arg("1").arg("2").arg("3").arg("4");

    // test if both sort programs work similarly
    assert_eq!(cmd1.assert().get_output(), cmd2.assert().get_output());

    // test each sort
    cmd1.assert()
        .success()
        .stdout("[1.0, 2.0, 3.0, 4.0]\n[4.0, 3.0, 2.0, 1.0]\n");
    cmd2.assert()
        .success()
        .stdout("[1.0, 2.0, 3.0, 4.0]\n[4.0, 3.0, 2.0, 1.0]\n");
}

#[test]
fn sort_test_2() {
    let mut cmd1 = Command::cargo_bin("sort").unwrap();
    let mut cmd2 = Command::cargo_bin("sort_bubble").unwrap();
    cmd1.arg("-0.5").arg("-9").arg("45.2").arg("10").arg("20");
    cmd2.arg("-0.5").arg("-9").arg("45.2").arg("10").arg("20");

    // test if both sort programs work similarly
    assert_eq!(cmd1.assert().get_output(), cmd2.assert().get_output());

    // test each sort
    cmd1.assert()
        .success()
        .stdout("[-9.0, -0.5, 10.0, 20.0, 45.2]\n[45.2, 20.0, 10.0, -0.5, -9.0]\n");
    cmd2.assert()
        .success()
        .stdout("[-9.0, -0.5, 10.0, 20.0, 45.2]\n[45.2, 20.0, 10.0, -0.5, -9.0]\n");
}

#[test]
fn point_sort_test_no_input() {
    let mut cmd1 = Command::cargo_bin("point_sort").unwrap();
    let mut cmd2 = Command::cargo_bin("point_sort_bubble").unwrap();

    // test if both sort programs work similarly
    assert_eq!(cmd1.assert().get_output(), cmd2.assert().get_output());

    // test each sort
    cmd1.assert().success().stdout("");
    cmd2.assert().success().stdout("");
}

#[test]
fn point_sort_test_1() {
    let mut cmd1 = Command::cargo_bin("point_sort").unwrap();
    let mut cmd2 = Command::cargo_bin("point_sort_bubble").unwrap();
    cmd1.arg("1").arg("4").arg("3").arg("2");
    cmd2.arg("1").arg("4").arg("3").arg("2");

    // test if both sort programs work similarly
    assert_eq!(cmd1.assert().get_output(), cmd2.assert().get_output());

    // test each sort
    cmd1.assert().success().stdout("[(1.0, 4.0), (3.0, 2.0)]\n[(3.0, 2.0), (1.0, 4.0)]\n[(3.0, 2.0), (1.0, 4.0)]\n[(1.0, 4.0), (3.0, 2.0)]\n");
    cmd2.assert().success().stdout("[(1.0, 4.0), (3.0, 2.0)]\n[(3.0, 2.0), (1.0, 4.0)]\n[(3.0, 2.0), (1.0, 4.0)]\n[(1.0, 4.0), (3.0, 2.0)]\n");
}

#[test]
fn point_sort_test_2() {
    let mut cmd1 = Command::cargo_bin("point_sort").unwrap();
    let mut cmd2 = Command::cargo_bin("point_sort_bubble").unwrap();
    cmd1.arg("-0.5")
        .arg("-9")
        .arg("45.2")
        .arg("10")
        .arg("20")
        .arg("1")
        .arg("5");
    cmd2.arg("-0.5")
        .arg("-9")
        .arg("45.2")
        .arg("10")
        .arg("20")
        .arg("1")
        .arg("5");

    // test if both sort programs work similarly
    assert_eq!(cmd1.assert().get_output(), cmd2.assert().get_output());

    // test each sort
    cmd1.assert().success().stdout("[(-0.5, -9.0), (20.0, 1.0), (45.2, 10.0)]\n[(45.2, 10.0), (20.0, 1.0), (-0.5, -9.0)]\n[(-0.5, -9.0), (20.0, 1.0), (45.2, 10.0)]\n[(45.2, 10.0), (20.0, 1.0), (-0.5, -9.0)]\n");
    cmd2.assert().success().stdout("[(-0.5, -9.0), (20.0, 1.0), (45.2, 10.0)]\n[(45.2, 10.0), (20.0, 1.0), (-0.5, -9.0)]\n[(-0.5, -9.0), (20.0, 1.0), (45.2, 10.0)]\n[(45.2, 10.0), (20.0, 1.0), (-0.5, -9.0)]\n");
}

#[test]
fn point_sort_test_3() {
    let mut cmd1 = Command::cargo_bin("point_sort").unwrap();
    let mut cmd2 = Command::cargo_bin("point_sort_bubble").unwrap();
    cmd1.arg("-0.5");
    cmd2.arg("-0.5");

    // test if both sort programs work similarly
    assert_eq!(cmd1.assert().get_output(), cmd2.assert().get_output());

    // test each sort
    cmd1.assert().success().stdout("[]\n[]\n[]\n[]\n");
    cmd2.assert().success().stdout("[]\n[]\n[]\n[]\n");
}
