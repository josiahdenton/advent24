Setting up file reading can be done either buffered

```rs
let fp = OpenOptions::new().read(true).open(path)?;
let reader = BufReader::new(fp);
let mut line = String::new();
let bytes_read = reader.read_line(&mut line);
// returning it...
return match bytes_read {
    Ok(0) => None,
    Ok(_) => Some(line),
    _ => None,
};
```

Or read the whole file

```rs
use std::fs;

let content = fs::read_to_string("path")?;  
```
