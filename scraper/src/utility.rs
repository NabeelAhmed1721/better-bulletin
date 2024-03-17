use std::path::Path;

pub fn retrieve_document(url: &str) -> String {
    let cache_file = Path::new("./cache")
        .join(url.replace("https://", ""))
        .with_extension("html");
    // let filename = cache_file.file_name().unwrap().to_str().unwrap();

    // if file is older than 1 day, delete cache
    if let Ok(metadata) = std::fs::metadata(&cache_file) {
        let created_at = metadata.created().unwrap();

        if created_at
            < std::time::SystemTime::now() - std::time::Duration::from_secs(60 * 60 * 24 * 3)
        {
            // println!("{} file is older than 3 days, deleting...", filename);
            std::fs::remove_file(&cache_file).unwrap();
            // println!("{} file deleted.", filename);
        }
    }

    // check if index.html file exists, if not make request, and save as cache
    let body = match &cache_file.exists() {
        true => {
            // println!("Found cache [{}], reading file...", filename);
            std::fs::read_to_string(cache_file).unwrap()
        }
        false => {
            // println!("Making request...");
            let body = ureq::get(url).call().unwrap().into_string().unwrap();
            // println!("Got a response.");

            // save to file
            let mut cache_path = cache_file.to_path_buf();

            cache_path.pop();
            // println!("Saving as cache [{}]...", filename);
            std::fs::create_dir_all(cache_path).unwrap();
            std::fs::write(cache_file, body.clone()).unwrap();
            body
        }
    };

    // println!("Retrieved file.");
    body
}

pub trait TrimAll {
    fn trim_all(&self) -> String;
}

impl TrimAll for str {
    /// Returns a string with all unnecessary white space removed.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = "   Hello   world    ";
    ///
    /// assert_eq!("Hello world", s.trim_all());
    /// ```
    fn trim_all(&self) -> String {
        self.trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

pub trait ReplaceMany {
    fn replace_many(&self, replacements: &[(&str, &str)]) -> String;
}

impl ReplaceMany for str {
    /// Returns a string with all replacements made.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = "Hello world";
    ///
    /// assert_eq!("Goodbye world", s.replace_many(&[("Hello", "Goodbye")]));
    /// ```
    fn replace_many(&self, replacements: &[(&str, &str)]) -> String {
        let mut s = self.to_string();

        for (from, to) in replacements {
            s = s.replace(from, to);
        }

        s
    }
}
