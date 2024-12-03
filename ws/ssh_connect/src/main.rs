use std::env;


fn main() {
 // 获取命令行参数，不包括程序本身的路径
    let args: Vec<String> = env::args().collect();
    let username = args.get(1).map(|s| s.as_ref()).unwrap();
    let password = args.get(2).map(|s| s.as_ref()).unwrap();
    let cennect = args.get(3).map(|s| s.as_str()).unwrap();

    let mut session = ssh::create_session()
        .username(username)
        .password(password)
        .connect(cennect)
        .unwrap()
        .run_local();
    let exec = session.open_exec().unwrap();
	
    const CMD: &str = "sh /webapps/intermediate-bond/call-details/check_usage.sh";

    let vec2: Vec<u8> = exec.send_command(CMD).unwrap();
    let str = String::from_utf8(vec2).unwrap();
    println!("{}", str);
    // 解析字符串并创建MachineInfo实例的列表
    let infos = parse_info(&str);

    // 打印MachineInfo实例的列表
    for info in &infos {
        println!(
            "部件: {}, 总量: {}, 占用率: {}",
            info.part, info.total, info.usage_rate
        );
    }

    session.close();
}
struct MachineInfo {
    part: String,
    total: String,
    usage_rate: String,
}

// 解析字符串并创建MachineInfo实例的函数
fn parse_info(str: &str) -> Vec<MachineInfo> {
    // 使用换行符分割字符串
    let lines: Vec<&str> = str.split('\n').collect();

    // 创建MachineInfo实例的向量
    let mut infos = Vec::new();

    // 遍历每一行
    for line in lines {
        // 跳过空行
        if line.trim().is_empty() {
            continue;
        }

        // 使用空格分割行
        let parts: Vec<&str> = line.split_whitespace().collect();

        // // 检查分割后的部分是否足够
        // if parts.len() < 3 {
        //     continue;
        // }

        // // 创建MachineInfo实例并添加到向量中
        // let info = MachineInfo {
        //     part: parts[0].trim().to_string(),
        //     total: parts[1].trim().to_string(),
        //     usage_rate: parts[2..].join(" ").trim().to_string(),
        // };

        let info = match parts.len() {
            2 => MachineInfo {
                part: parts[0].trim().to_string(),
                total: "".to_string(),
                usage_rate: parts[1].trim().to_string(), // 默认值
            },
            _ => MachineInfo {
                part: parts[0].trim().to_string(),
                total: parts[1].trim().to_string(),
                usage_rate: parts[2..].join(" ").trim().to_string(),
            },
        };
        infos.push(info);
    }

    // 返回MachineInfo实例的向量
    infos
}