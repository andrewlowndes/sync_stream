use async_stream::stream;
use futures::StreamExt;
use rand::random;
use std::{cmp::Ordering, time::Duration};
use sync_stream::sync_stream;
use tokio::time::sleep;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Item<T> {
    id: u32,
    value: T,
}

impl<T> Eq for Item<T> {}

//implement ordering for our item
impl<T, B> PartialEq<Item<B>> for Item<T> {
    fn eq(&self, other: &Item<B>) -> bool {
        self.id == other.id
    }
}

impl<T, B> PartialOrd<Item<B>> for Item<T> {
    fn partial_cmp(&self, other: &Item<B>) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

async fn delay() {
    sleep(Duration::from_millis(random::<u8>().into())).await;
}

#[tokio::main]
async fn main() {
    let a = stream! {
        delay().await;
        yield Item { id: 1, value: 100 };
        delay().await;
        yield Item { id: 6, value: 200 };
        delay().await;
        yield Item { id: 8, value: 100 };
        delay().await;
        yield Item { id: 9, value: 300 };
        delay().await;
        yield Item { id: 10, value: 100 };
        delay().await;
        yield Item { id: 18, value: 900 };
        delay().await;
    };
    let b = stream! {
        delay().await;
        yield Item { id: 2, value: "a" };
        delay().await;
        yield Item { id: 4, value: "z" };
        delay().await;
        yield Item { id: 14, value: "r" };
        delay().await;
        yield Item { id: 23, value: "c" };
        delay().await;
    };
    let c = stream! {
        delay().await;
        yield Item { id: 3, value: 'p' };
        delay().await;
        yield Item { id: 5, value: 'c' };
        delay().await;
        yield Item { id: 17, value: 'd' };
        delay().await;
        yield Item { id: 19, value: 'w' };
        delay().await;
    };

    //our three stream items will be emitted ordered by the id in our stream items
    sync_stream!(a, b, c)
        .for_each(|(a, b, c)| async move {
            println!("{a:?},{b:?},{c:?}");
        })
        .await;
}
