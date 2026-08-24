#![cfg(feature = "engine")]

use std::{error::Error, panic, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::engine::{ExternalEngineAnalysisOptions, ExternalEngineOptions},
    models::engine::{SearchMethod, UciVariant},
};

// Connect to test clients
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_engine_url("http://localhost:9666")
        .unwrap()
        .with_authentication("lip_li")
        .unwrap()
        .build()
});

static BOT0: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_engine_url("http://localhost:9666")
        .unwrap()
        .with_authentication("lip_bot0")
        .unwrap()
        .build()
});

static DEFAULT: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_engine_url("http://localhost:9666")
        .unwrap()
        .build()
});

#[tokio::test]
async fn external_engine_list() {
    // Create an external engine for testing
    let options = ExternalEngineOptions::new(64, 1, "Perch", "afullysecuresecrettoken")
        .provider_data("arbitrarydata")
        .variants(&[UciVariant::Chess]);
    LI.external_engine().create(&options).await.unwrap();

    // Run some test cases
    let result = LI.external_engine().list().await;
    assert!(
        result.is_ok(),
        "Failed to list external engines: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.external_engine().list().await;
    assert!(
        result.is_ok(),
        "Failed to list external engines: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.external_engine().list().await;
    assert!(
        result.is_err(),
        "Listing external engines did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn external_engine_create() {
    // Create options for testing
    let options1 = ExternalEngineOptions::new(64, 1, "Perch", "afullysecuresecrettoken");
    let options2 = ExternalEngineOptions::new(64, 1, "Perch", "afullysecuresecrettoken")
        .provider_data("arbitrarydata")
        .variants(&[UciVariant::Chess]);
    let options3 = ExternalEngineOptions::new(0, 0, "", "");

    // Run some tests
    let result = LI.external_engine().create(&options1).await;
    assert!(
        result.is_ok(),
        "Failed to create external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.external_engine().create(&options2).await;
    assert!(
        result.is_ok(),
        "Failed to create external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.external_engine().create(&options3).await;
    assert!(
        result.is_err(),
        "Creating external engine did not fail: {:?}",
        result.unwrap()
    );

    let result = DEFAULT.external_engine().create(&options1).await;
    assert!(
        result.is_err(),
        "Creating external engine did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn external_engine_show() {
    // Get engine lists for testing
    let list1 = LI.external_engine().list().await.unwrap();
    let list2 = BOT0.external_engine().list().await.unwrap();

    // Run some test cases
    let result = LI.external_engine().show(&list1[0].id).await;
    assert!(
        result.is_ok(),
        "Failed to get external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.external_engine().show(&list2[0].id).await;
    assert!(
        result.is_ok(),
        "Failed to get external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.external_engine().show("notvalid").await;
    assert!(
        result.is_err(),
        "Getting external engine did not fail: {:?}",
        result.unwrap()
    );

    let result = DEFAULT.external_engine().show(&list1[0].id).await;
    assert!(
        result.is_err(),
        "Getting external engine did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn external_engine_update() {
    // Get engine lists and create options for testing
    let list1 = LI.external_engine().list().await.unwrap();
    let list2 = BOT0.external_engine().list().await.unwrap();
    let options1 = ExternalEngineOptions::new(32, 2, "PerchUpdate", "afullysecuresecrettoken")
        .provider_data("morearbitrarydata")
        .variants(&[UciVariant::Chess]);
    let options2 = ExternalEngineOptions::new(0, 0, "", "");

    // Run some test cases
    let result = LI.external_engine().update(&list1[0].id, &options1).await;
    assert!(
        result.is_ok(),
        "Failed to update external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.external_engine().update(&list2[0].id, &options1).await;
    assert!(
        result.is_ok(),
        "Failed to update external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.external_engine().update(&list1[0].id, &options2).await;
    assert!(
        result.is_err(),
        "Updating external engine did not fail: {:?}",
        result.unwrap()
    );

    let result = LI.external_engine().update("notvalid", &options1).await;
    assert!(
        result.is_err(),
        "Updating external engine did not fail: {:?}",
        result.unwrap()
    );

    let result = DEFAULT
        .external_engine()
        .update(&list1[0].id, &options2)
        .await;
    assert!(
        result.is_err(),
        "Updating external engine did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn external_engine_delete() {
    // Create engines for testing
    let options = ExternalEngineOptions::new(1, 1, "Test", "afullysecuresecrettoken");
    let engine1 = LI.external_engine().create(&options).await.unwrap();
    let engine2 = BOT0.external_engine().create(&options).await.unwrap();

    // Run some test cases
    let result = LI.external_engine().delete(&engine1.id).await;
    assert!(
        result.is_ok(),
        "Failed to delete external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.external_engine().delete(&engine2.id).await;
    assert!(
        result.is_ok(),
        "Failed to delete external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.external_engine().delete(&engine1.id).await;
    assert!(
        result.is_err(),
        "Deleting external engine did not fail: {:?}",
        result.unwrap()
    );

    let result = LI.external_engine().delete("notvalid").await;
    assert!(
        result.is_err(),
        "Deleting external engine did not fail: {:?}",
        result.unwrap()
    );

    let result = DEFAULT.external_engine().delete(&engine1.id).await;
    assert!(
        result.is_err(),
        "Deleting external engine did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn external_engine_analysis() {
    // Create engines for testing
    let engine_options = ExternalEngineOptions::new(128, 4, "Stockfish", "secretstockfishtoken");
    let engine1 = LI.external_engine().create(&engine_options).await.unwrap();
    let engine2 = BOT0
        .external_engine()
        .create(&engine_options)
        .await
        .unwrap();
    let engine3 = engine1.clone();
    let engine4 = engine2.clone();

    // Create options for testing
    let options1 = ExternalEngineAnalysisOptions::new(
        SearchMethod::Depth(5),
        64,
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &[],
        1,
        "",
        1,
        UciVariant::Chess,
    );
    let options2 = ExternalEngineAnalysisOptions::new(
        SearchMethod::Movetime(1000),
        64,
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &["e2e4"],
        1,
        "testing",
        1,
        UciVariant::Chess,
    );
    let options3 = options1.clone();
    let options4 = options2.clone();

    // Run some test cases
    let client_task = tokio::spawn(async move {
        let mut request_stream = LI
            .external_engine()
            .analysis_request(&engine1.id, &engine1.client_secret, &options1)
            .await
            .unwrap();
        if let Some(request_result) = request_stream.next().await {
            assert!(
                request_result.is_ok(),
                "Failed to analyse with external engine: {:?}",
                request_result.unwrap_err().source().unwrap()
            );
        }
    });

    let provider_task = tokio::spawn(async move {
        let acquire_result = LI
            .external_engine()
            .analysis_acquire("secretstockfishtoken")
            .await;
        assert!(
            acquire_result.as_ref().is_ok_and(|option| option.is_some()),
            "Failed to analyse with external engine: {:?}",
            acquire_result.unwrap_err().source().unwrap()
        );

        let submit_result = LI
            .external_engine()
            .analysis_submit(&acquire_result.unwrap().unwrap().id,
                "info depth 1 seldepth 2 multipv 1 score cp -1 nodes 20 nps 20000 hashfull 0 tbhits 0 time 1 pv e2e4"
            )
            .await;
        assert!(
            submit_result.is_ok(),
            "Failed to submit external engine analysis data: {:?}",
            submit_result.unwrap_err().source().unwrap()
        );
    });

    let provider_result = provider_task.await;
    let client_result = client_task.await;
    if provider_result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(provider_result.unwrap_err().into_panic());
    }
    if client_result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(client_result.unwrap_err().into_panic());
    }

    let client_task = tokio::spawn(async move {
        let mut request_stream = BOT0
            .external_engine()
            .analysis_request(&engine2.id, &engine2.client_secret, &options2)
            .await
            .unwrap();
        if let Some(request_result) = request_stream.next().await {
            assert!(
                request_result.is_ok(),
                "Failed to analyse with external engine: {:?}",
                request_result.unwrap_err().source().unwrap()
            );
        }
    });

    let provider_task = tokio::spawn(async move {
        let acquire_result = BOT0
            .external_engine()
            .analysis_acquire("secretstockfishtoken")
            .await;
        assert!(
            acquire_result.as_ref().is_ok_and(|option| option.is_some()),
            "Failed to analyse with external engine: {:?}",
            acquire_result.unwrap_err().source().unwrap()
        );

        let submit_result = BOT0
            .external_engine()
            .analysis_submit(&acquire_result.unwrap().unwrap().id,
                "info depth 1 seldepth 2 multipv 1 score cp -29 nodes 22 nps 22000 hashfull 0 tbhits 0 time 1 pv e7e5"
            )
            .await;
        assert!(
            submit_result.is_ok(),
            "Failed to submit external engine analysis data: {:?}",
            submit_result.unwrap_err().source().unwrap()
        );
    });

    let provider_result = provider_task.await;
    let client_result = client_task.await;
    if provider_result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(provider_result.unwrap_err().into_panic());
    }
    if client_result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(client_result.unwrap_err().into_panic());
    }

    let client_task = tokio::spawn(async move {
        let mut request_stream = DEFAULT
            .external_engine()
            .analysis_request(&engine3.id, &engine3.client_secret, &options3)
            .await
            .unwrap();
        if let Some(request_result) = request_stream.next().await {
            assert!(
                request_result.is_ok(),
                "Failed to analyse with external engine: {:?}",
                request_result.unwrap_err().source().unwrap()
            );
        }
    });

    let provider_task = tokio::spawn(async move {
        let acquire_result = DEFAULT
            .external_engine()
            .analysis_acquire("secretstockfishtoken")
            .await;
        assert!(
            acquire_result.as_ref().is_ok_and(|option| option.is_some()),
            "Failed to analyse with external engine: {:?}",
            acquire_result.unwrap_err().source().unwrap()
        );

        let submit_result = DEFAULT
            .external_engine()
            .analysis_submit(&acquire_result.unwrap().unwrap().id,
                "info depth 1 seldepth 2 multipv 1 score cp -29 nodes 22 nps 22000 hashfull 0 tbhits 0 time 1 pv e7e5"
            )
            .await;
        assert!(
            submit_result.is_ok(),
            "Failed to submit external engine analysis data: {:?}",
            submit_result.unwrap_err().source().unwrap()
        );
    });

    let provider_result = provider_task.await;
    let client_result = client_task.await;
    if provider_result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(provider_result.unwrap_err().into_panic());
    }
    if client_result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(client_result.unwrap_err().into_panic());
    }

    let request_stream = LI
        .external_engine()
        .analysis_request("notvalid", &engine4.client_secret, &options4)
        .await;
    assert!(
        request_stream.is_err(),
        "Analysing with external engine did not fail"
    );

    let acquire_result = LI.external_engine().analysis_acquire("nosuchtoken").await;
    assert!(
        acquire_result.as_ref().is_ok_and(|option| option.is_none())
            || acquire_result.as_ref().is_err(),
        "Analysing with external engine did not fail: {:?}",
        acquire_result.as_ref().err().and_then(|e| e.source())
    );

    let submit_result = LI
        .external_engine()
        .analysis_submit("nosuchid",
            "info depth 1 seldepth 2 multipv 1 score cp -29 nodes 22 nps 22000 hashfull 0 tbhits 0 time 1 pv e7e5"
        )
        .await;
    assert!(
        submit_result.is_err(),
        "Submitting external engine analysis data did not fail: {:?}",
        submit_result.unwrap()
    );
}
