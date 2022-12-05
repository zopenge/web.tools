// use futures::future::join_all;
// use scraper::{Html, Selector};
// use std::env;

// async fn fetch_path(path: String) -> surf::Result<String> {
//     let mut back_string = String::new();
//     match surf::get(&path).await {
//         Ok(mut response) => {
//             match response.body_string().await {
//                 Ok(text) => back_string = format!("{}", text),
//                 Err(_) => {
//                     println!("Read response text Error!")
//                 }
//             };
//         }
//         Err(_) => {
//             println!("reqwest get Error!")
//         }
//     }
//     Ok(back_string)
// }

// #[tokio::main]
// async fn main() -> surf::Result<()> {
//     let stdin = env::args().nth(1).take().unwrap();
//     let paths = vec![stdin.to_string()];
//     let result_list = join_all(paths.into_iter().map(|path| fetch_path(path))).await;

//     let mut list_string: Vec<String> = vec![];
//     for ele in result_list.into_iter() {
//         if ele.is_ok() {
//             list_string.push(ele.unwrap())
//         } else {
//             return Err(ele.unwrap_err());
//         }
//     }
//     let v = list_string.get(0).take().unwrap();
//     //  println!("{}",v);
//     let fragment = Html::parse_fragment(v);
//     let ul_selector = Selector::parse("script").unwrap();

//     for element in fragment.select(&ul_selector) {
//         println!("{}", element.inner_html());
//     }
//     // println!("请求输出：{:?}",list_string);
//     Ok(())
// }

use futures::future::join_all;
use std::env;

async fn fetch_path(path: String) -> surf::Result<String> {
    let mut back_string = String::new();
    back_string = "hahah".to_string();
    // match surf::get(&path).await {
    //     Ok(mut response) => {
    //         match response.body_string().await {
    //             Ok(text) => back_string = format!("{}", text),
    //             Err(_) => {
    //                 println!("Read response text Error!")
    //             }
    //         };
    //     }
    //     Err(_) => {
    //         println!("reqwest get Error!")
    //     }
    // }
    Ok(back_string)
}

#[tokio::main]
async fn main() -> surf::Result<()> {
    let args = env::args();
    for arg in args {
        println!("{}", arg);
    }

    let stdin = env::args().nth(1).take().unwrap();
    let paths = vec![stdin.to_string()];
    // for ele in paths {
    //     println!("{}", ele);
    // }

    println!("Hello, world!111");

    // let result_list = join_all(paths.into_iter().map(|path| fetch_path(path))).await;

    let reuslt_list = join_all(paths.into_iter().map(
        |path| {
            println!("{}", path);

            return fetch_path(path);
        }, //        fetch_path(path);
    ))
    .await;

    for ele in reuslt_list {
        let v = ele.unwrap();
        println!("{}", v);
    }

    Ok(())
}
