use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process;
use std::{env, fs};
#[derive(Debug)]
pub enum Bread<'a> {
    Bean(&'a String),
    //File(Result<Vec<u8>, std::io::Error>),
    File(Result<File, std::io::Error>),
}
impl Bread<'_> {
    pub fn open(&self) -> Bread {
        //let mut file: Result<Vec<u8>, std::io::Error>;
        if let Bread::Bean(file) = self {
            println!("打开 Bean: {}", file);
            Bread::File(File::open(file.clone()))
        } else {
            println!("Error: 打开文件错误(18)");
            process::exit(1);
        }
    }
    pub fn read(&mut self) -> Vec<u8> {
        if let Bread::File(file) = self {
            //println!("{:#?}", file);
            if let Ok(file) = file {
                let mut buf: Vec<u8> = Vec::new();
                file.read_to_end(&mut buf).unwrap();
                //println!("{:#?}", file);
                buf
            } else {
                println!("Error: 读取文件错误(31)");
                process::exit(1);
            }
        } else {
            println!("Error: 读取文件错误(35)");
            process::exit(1);
        }
    }
}
pub fn bargs() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Tips: 请输入运算符号和文件名称");
        quit(1);
    }
    args
}
pub fn write(file: &String, buf: &[u8]) -> Result<(), std::io::Error> {
    if let Ok(()) = fs::remove_file(file) {
        println!("Bread: brean 很好吃");
    } else {
        println!("Angry Bread: brean 死掉了");
        //quit(1);
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file.to_owned()); // + &String::from("_new"));
    if let Err(s) = file {
        println!("Error: {}", s);
        process::exit(1);
    } else {
        file?.write_all(buf)
    }
}
pub fn open(file: &String) -> Vec<u8> {
    let buf = Bread::Bean(file);
    let mut buf = buf.open();
    buf.read()
}
pub fn quit(code: i32) {
    process::exit(code)
}
pub fn cmd(args: &Vec<String>) {
    if let Err(s) = process::Command::new("chmod")
        .arg("+x")
        .arg(args[0].as_str())
        .spawn()
    {
        println!("Warn: {}", s);
        //quit(1);
    }
}
pub fn add(file: &String, buf: &[u8], bean: &[u8]) -> Vec<u8> {
    let s = format!("[s-cyear-{}]", file).into_bytes();
    let e = format!("[e-cyear-{}]", file).into_bytes();
    let mut buf = buf.to_vec();
    buf.extend(s);
    buf.extend(bean.to_vec());
    buf.extend(e);
    buf
}
pub fn eat(args: &Vec<String>) -> Vec<u8> {
    let buf = open(&args[0]);
    let buf_len = buf.len();
    let bean = open(&args[2]);
    let buf = add(&args[2], &buf, &bean);
    if let Ok(()) = fs::remove_file(&args[2]) {
        println!("Bread: brean 很好吃");
    } else {
        println!("Angry Bread: brean 死掉了");
        quit(1);
    }
    println!(
        "[Write]\nBread len: {}\nBean len: {}\nBrean len : {}",
        buf_len,
        bean.len(),
        buf.len(),
    );
    if let Err(s) = write(&args[0], &buf) {
        println!("Error: {}", s);
        quit(1);
    }
    cmd(&args);
    println!("[End]");
    buf
}
pub fn spit(args: &Vec<String>) -> Vec<u8> {
    println!("[Splt]\nBrean: 初始化中...");
    let s = format!("[s-cyear-{}]", args[2]).into_bytes();
    let e = format!("[e-cyear-{}]", args[2]).into_bytes();
    let buf_ = open(&args[0]);
    let buf = buf_.as_slice();
    let s_cache: &[u8] = s.as_slice();
    let e_cache: &[u8] = e.as_slice();
    let buf_len = buf.len();
    let s_cache_len = s_cache.len();
    let e_cache_len = e_cache.len();
    let s_place: usize;
    let e_place: usize;
    println!("Brean: 寻找食物...");
    for i in 0..buf_len {
        let len = i + s_cache_len;
        if len > buf_len {
            println!(r"Brean: 没有食物(T^T)\[End]");
            break;
        }
        if s_cache == &buf[i..len] {
            //println!("{:#?}", cache);
            println!("Brean: 找到食物了୧( ॑ധ ॑)୨");
            s_place = len;
            for o in i..buf_len {
                let len = o + e_cache_len;
                if len > buf_len {
                    break;
                }
                if e_cache == &buf[o..len] {
                    println!("Brean: 酝酿中(｡•ˇ‸ˇ•｡)…");
                    e_place = o;
                    let b = buf[s_place..e_place].to_vec();
                    println!("Bean len: {}", b.len());
                    //old
                    //for _ in i..len {
                    //    buf_.remove(i);
                    //}
                    let buf_1 = &buf_[..i];
                    let buf_2 = &buf_[len..];
                    let mut buf_ = Vec::new();
                    buf_.extend(&mut buf_1.iter());
                    buf_.extend(&mut buf_2.iter());
                    println!("Brean: 在这里！(write)");
                    if let Err(s) = write(&args[0], buf_.as_slice()) {
                        println!("Error: {}", s);
                        quit(1);
                    }
                    cmd(&args);
                    if let Err(s) = write(&args[2], &b.as_slice()) {
                        println!("Error: {}", s);
                        quit(1);
                    }
                    println!("[End]");
                    return b;
                }
            }
            break;
        }
    }
    vec![]
}
pub fn brean(args: &Vec<String>) -> Vec<u8> {
    if args[1] == "+" {
        return eat(args);
    }
    return spit(args);
}
