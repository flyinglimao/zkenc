fn main() {
    // 解析命令行參數
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("使用方法: {} <參數>", args[0]);
        std::process::exit(1);
    }

    // 調用核心演算法
    let result = zkenc_core::run_algorithm(&args[1]);
    
    // 輸出結果
    println!("結果: {:?}", result);
}