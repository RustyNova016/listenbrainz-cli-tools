use rust_decimal::Decimal;

use crate::core::user_compatibility::builder::UserCompatibilityBuilder;
use crate::core::user_compatibility::TargetUser;
use crate::utils::cli::await_next;

pub async fn compatibility_command(user_a: &str, user_b: &str) -> color_eyre::Result<()> {
    let compatibility = UserCompatibilityBuilder::new(user_a.to_string(), user_b.to_string())
        .build()
        .await?;

    display_results(
        compatibility.user_a(),
        compatibility.user_b(),
        compatibility.get_shared_recordings().await?.len(),
        compatibility
            .get_user_shared_percent(TargetUser::UserA)
            .await?,
        compatibility
            .get_user_shared_percent(TargetUser::UserB)
            .await?,
        compatibility.get_shared_ratio().await? * Decimal::ONE_HUNDRED,
    );

    Ok(())
}

fn display_results(
    user_a: &str,
    user_b: &str,
    shared_recordings: usize,
    percent_shared_a: Decimal,
    percent_shared_b: Decimal,
    compatibility_score: Decimal,
) {
    println!("Compatibility results:");
    println!();
    println!("[Shared Recordings]");
    println!("  There is {shared_recordings} recordings both listened by {user_a} and {user_b}");
    println!(
        "   > This is {}% of {user_a}'s listened recordings",
        percent_shared_a.trunc_with_scale(2)
    );
    println!(
        "   > This is {}% of {user_b}'s listened recordings",
        percent_shared_b.trunc_with_scale(2)
    );
    println!();
    println!("[Compatibility]");
    println!(
        "  The compatibilty score between {user_a} and {user_b} is {}%",
        compatibility_score.trunc_with_scale(2)
    );

    await_next();
}

#[tokio::test]
#[serial_test::serial]
async fn compatibility() {
    compatibility_command("RustyNova", "backhdlp")
        .await
        .unwrap();

    assert!(true)
}
