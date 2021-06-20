use siderunner::{parse, Runner};
use std::fs::File;
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::test;

async fn testing(path: &str) {
    let mut file = File::open(path).expect("Failed to read a file");
    let side_file = parse(&mut file).expect("Failed to parse a file");

    let mut cops = DesiredCapabilities::chrome();
    cops.set_headless()
        .expect("Failed to set a headless setting");
    let wb = WebDriver::new("http://localhost:4444/wd/hub", cops)
        .await
        .expect("Failed to create a webdriver");

    let mut runner = Runner::new(&wb);
    match runner.run(&side_file).await {
        Ok(()) => {}
        Err(err) => {
            wb.quit().await.expect("Failed to stop a webdriver");
            panic!("Failed to run a file {:?} test: {:?}", path, err);
        }
    }

    wb.quit().await.expect("Failed to stop a webdriver");
}

macro_rules! test_file {
    ( $test_file:expr, $test_name:ident ) => {
        #[test]
        async fn $test_name() {
            testing($test_file).await;
        }
    };
}

test_file!("tests/resources/basic/test.side.json", basic);
test_file!(
    "tests/resources/open relative url/test.side.json",
    open_relative_url
);
test_file!(
    "tests/resources/commands/assert/test.side.json",
    command_assert
);
test_file!(
    "tests/resources/commands/click/test.side.json",
    command_click
);
test_file!(
    "tests/resources/commands/execute/test.side.json",
    command_execute
);
test_file!(
    "tests/resources/commands/execute async/test.side.json",
    command_execute_async
);
test_file!(
    "tests/resources/commands/run script/test.side.json",
    command_run_script
);
test_file!(
    "tests/resources/commands/for each/test.side.json",
    command_for_each
);
test_file!(
    "tests/resources/commands/add selection/test.side.json",
    command_add_selection
);
