use std::env;
use std::process;

pub mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Get the command name (either from argv[0] or argv[1])
    let cmd_name = if args.len() > 1 {
        // Called as: nullbox <command> [args...]
        &args[1]
    } else {
        // Called as symlink: <command> [args...]
        let exe_name = args[0].split(['/', '\\']).last().unwrap_or("nullbox");
        let name = exe_name.trim_end_matches(".exe");
        if name == "nullbox" {
            print_usage();
            process::exit(0);
        }
        name
    };

    let cmd_args = if args.len() > 1 && &args[1] == cmd_name {
        &args[2..]
    } else {
        &args[1..]
    };

    let result = match cmd_name {
        "arch" => command::arch::execute(cmd_args),
        "base64" => command::base64::execute(cmd_args),
        "basename" => command::basename::execute(cmd_args),
        "cat" => command::cat::execute(cmd_args),
        "cd" => command::cd::execute(cmd_args),
        "cksum" => command::cksum::execute(cmd_args),
        "clear" | "clr" | "cls" => command::clear::execute(cmd_args),
        "comm" => command::comm::execute(cmd_args),
        "cp" => command::cp::execute(cmd_args),
        "cut" => command::cut::execute(cmd_args),
        "date" => command::date::execute(cmd_args),
        "df" => command::df::execute(cmd_args),
        "diff" => command::diff::execute(cmd_args),
        "dirname" => command::dirname::execute(cmd_args),
        "du" => command::du::execute(cmd_args),
        "echo" => command::echo::execute(cmd_args),
        "env" => command::env::execute(cmd_args),
        "expand" => command::expand::execute(cmd_args),
        "factor" => command::factor::execute(cmd_args),
        "false" => command::false_cmd::execute(cmd_args),
        "find" => command::find::execute(cmd_args),
        "fold" => command::fold::execute(cmd_args),
        "grep" => command::grep::execute(cmd_args),
        "head" => command::head::execute(cmd_args),
        "hostname" => command::hostname::execute(cmd_args),
        "join" => command::join::execute(cmd_args),
        "ln" => command::ln::execute(cmd_args),
        "del" | "rm" => command::rm::execute(cmd_args),
        "dir" | "ls" => command::ls::execute(cmd_args),
        "md5" => command::md5::execute(cmd_args),
        "mkdir" => command::mkdir::execute(cmd_args),
        "mv" => command::mv::execute(cmd_args),
        "nl" => command::nl::execute(cmd_args),
        "nproc" => command::nproc::execute(cmd_args),
        "od" => command::od::execute(cmd_args),
        "paste" => command::paste::execute(cmd_args),
        "printenv" => command::printenv::execute(cmd_args),
        "pwd" => command::pwd::execute(cmd_args),
        "readlink" => command::readlink::execute(cmd_args),
        "realpath" => command::realpath::execute(cmd_args),
        "rx" | "regex" => command::regex::execute(cmd_args),
        "rev" => command::rev::execute(cmd_args),
        "rmdir" => command::rmdir::execute(cmd_args),
        "seq" => command::seq::execute(cmd_args),
        "sha256" => command::sha256::execute(cmd_args),
        "sleep" => command::sleep::execute(cmd_args),
        "sort" => command::sort::execute(cmd_args),
        "split" => command::split::execute(cmd_args),
        "stat" => command::stat::execute(cmd_args),
        "strings" => command::strings::execute(cmd_args),
        "sum" => command::sum::execute(cmd_args),
        "tac" => command::tac::execute(cmd_args),
        "tail" => command::tail::execute(cmd_args),
        "tee" => command::tee::execute(cmd_args),
        "touch" => command::touch::execute(cmd_args),
        "tr" => command::tr::execute(cmd_args),
        "tree" => command::tree::execute(cmd_args),
        "true" => command::true_cmd::execute(cmd_args),
        "truncate" => command::truncate::execute(cmd_args),
        "uname" => command::uname::execute(cmd_args),
        "unexpand" => command::unexpand::execute(cmd_args),
        "uniq" => command::uniq::execute(cmd_args),
        "uptime" => command::uptime::execute(cmd_args),
        "wc" => command::wc::execute(cmd_args),
        "which" => command::which::execute(cmd_args),
        "whoami" => command::whoami::execute(cmd_args),
        "xxd" => command::xxd::execute(cmd_args),
        "yes" => command::yes::execute(cmd_args),
        _ => {
            eprintln!("nullbox: unknown command '{}'", cmd_name);
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("{}: {}", cmd_name, e);
        process::exit(1);
    }
}

fn print_usage() {
    println!("nullbox - A BusyBox-like utility for Windows and FreeBSD");
    println!("\nUsage: nullbox <command> [args...]");
    println!("\n64 Available commands:");
    println!("  arch basename base64 cat cd cksum clear/clr/cls comm cp cut");
    println!("  date df diff dirname du echo env expand factor false find");
    println!("  fold grep head hostname join ln ls/dir md5 mkdir mv nl");
    println!("  nproc od paste printenv pwd readlink realpath regex/rx rev");
    println!("  rm/del rmdir seq sha256 sleep sort split stat strings sum");
    println!("  tac tail tee touch tr tree true truncate uname unexpand");
    println!("  uniq uptime wc which whoami xxd yes");
}
