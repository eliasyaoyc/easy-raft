mod test_raft;

use test_raft::TestHardness;
use raftkv::Role;
use tokio::time::Duration;

#[tokio::test]
async fn dynamic_member() {
    let test = TestHardness::default();
    test.add_node(1);
    test.initialize().await.unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;
    assert_eq!(test.metrics(1).await.unwrap().leader, Some(1));
    assert_eq!(test.metrics(1).await.unwrap().role, Role::Leader);

    test.write(Action::put("a", 1)).await.unwrap();
    test.write(Action::put("b", 2)).await.unwrap();

    test.add_node(2);
    test.add_non_voter(2).await.unwrap();

    tokio::time::sleep(Duration::from_secs(1)).await;

    assert_eq!(test.read_from(1, "a").await.unwrap(), Some(1));
    assert_eq!(test.read_from(1, "b").await.unwrap(), Some(2));

    assert_eq!(test.read_from(2, "a").await.unwrap(), Some(1));
    assert_eq!(test.read_from(2, "b").await.unwrap(), Some(2));
}