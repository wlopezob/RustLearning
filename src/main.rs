pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

mod error;

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub done: bool,
    pub desc: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            done: false,
            desc: None,
        }
    }
}

impl Task {
    fn new(title: impl Into<String>) -> Task {
        Task {
            title: title.into(),
            done: false,
            desc: None,
        }
    }
}

#[derive(Debug)]
pub struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Default, Clone)]
pub struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,
     body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url.insert(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method.insert(method.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) ->  Self {
        self.body.insert(body.into());
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<Request> {
        let Some(url) = self.url else {
            return Err(Error::Static("No URL"));
        };
        let method = self
            .method
            .unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            url,
            method,
            headers: self.headers,
            body: self.body,
        })
    }
}
fn main() -> Result<()> {
    // let task = Task {
    //     title: "demo".to_string(),
    //     done: false,
    //     desc: Some("desc".to_string()),
    // };

    // let task = Task::new("Task 01");

    // let task: Option<Task> = None;
    // let task = task.unwrap_or_default();

    // let task = Task {
    //     done: true,
    //     ..Default::default()
    // };

    // let task = Task {
    //     done: true,
    //     ..Task::new("Cool title")
    // };

    let mut req_builder = RequestBuilder::new().url("http://localhost").method("GET");
    let req_builder = req_builder.header("HOST", "localhost");
    let req = req_builder.clone().build()?;

    print!("{req:#?}");

    let req = req_builder.header("HOST", "localhost").build()?;
    print!("{req:#?}");
    
    Ok(())
}
