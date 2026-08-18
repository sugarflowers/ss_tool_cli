use std::path::PathBuf;
use std::fs;
use anyhow::{Result, anyhow};
use screenshots::Screen;
use glob::glob;
use clap::Parser;


#[derive(Default, Debug)]
struct Capture {
    path: PathBuf,
    count: Vec<i64>,
}

#[derive(Parser, Debug)]
#[command(
    name = "ss_tool",
    version = "0.0.1",
    author = "sugarflower",
)]
struct Args {
    #[arg(
        value_name="path",
        help = "Image save path"
    )]
    path: String,

    #[arg(
        value_name = "interval",
        help = "Save interval (seconds)"
    )]
    interval: u64,
    //target: Option<usize>,
}


impl Capture {
    fn new() -> Result<Capture> {
        let screens = Screen::all()?;
        Ok(Capture {
            count: vec![0; screens.len()],
            ..Default::default()
        })
    }

    fn path(&mut self, path: &str) -> Result<()> {
        self.path = path.into();
        self.check()?;
        Ok(())

    }

    fn file_count(&mut self, idx: usize) -> Result<()> {
        let buf = self.path.clone();
        let target = buf.to_str().unwrap();
        let pattern = format!("{target}/**/*.png");
        let mut count = 0;
        for entry in glob(&pattern)? {
            match entry {
                Ok(_) => count += 1,
                Err(e) => return Err(anyhow!(e)),
            }
        }
        self.count[idx] = count;
        Ok(())
    }

    fn check(&mut self) -> Result<()> {
        let screens = Screen::all()?;
        for idx in 0..screens.len() {
            self.file_count(idx)?;
            let mut path = self.path.clone();
            path.push(format!("{idx}"));
            fs::create_dir_all(&path)?;
        }
        Ok(())
    }

    fn capture(&self) -> Result<()> {
        let screens = Screen::all()?;
        for (idx, screen) in screens.into_iter().enumerate() {
            let mut path = self.path.clone();
            let count = self.count[idx];
            path.push(format!("{idx}"));
            path.push(format!("{count:06}.png"));
            screen.capture()?.save(&path)?;
        }
        Ok(())
    }
}


fn run() -> Result<()> {
    let args = Args::parse();

    let mut cap = Capture::new()?;

    loop {
        cap.path(&args.path)?;
        cap.capture()?;
        std::thread::sleep(std::time::Duration::from_secs(args.interval));
    }
    //Ok(())

}

fn main() {
	if let Err(e) = run() {
        
        let msg = e.chain()
        	.map(|cause| cause.to_string())
        	.collect::<Vec<_>>()
        	.join("\n");
        
        println!("Error: {}", msg);
    }
}
