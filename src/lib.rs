use ipnetwork::IpNetwork;
use std::net::IpAddr;
use worker::*;

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    // 获取 URL 对象
    let url = req.url().unwrap();

    // 获取查询参数
    let query_string = url.query().unwrap_or("");

    // 解析查询参数为键值对
    let query_params: Vec<(&str, &str)> = query_string
        .split('&')
        .map(|param| {
            let mut iter = param.splitn(2, '=');
            (iter.next().unwrap_or(""), iter.next().unwrap_or(""))
        })
        .collect();

    // 获取名为 "cidr_str" 的查询参数值，如果不存在则使用默认值
    let cidr_str = query_params.iter()
        .find(|&&(key, _)| key == "cidr")
        .map(|&(_, value)| value)
        .unwrap_or("192.168.1.0/24");
    console_log!("CIDR: {}", cidr_str); // 调试日志
    // 输入CIDR范围字符串
    // let cidr_str = "192.168.1.0/24";

    // 解析CIDR字符串为IpNetwork
    let ip_network: IpNetwork = cidr_str.parse().expect("Invalid CIDR");

    // 获取CIDR范围内的所有IP地址
    let ip_addresses: Vec<IpAddr> = ip_network.iter().map(|ip| ip).collect();

    let result = ip_addresses
        .iter()
        .map(|ip| ip.to_string())
        .collect::<Vec<String>>();
    Response::ok(result.join("\n"))
}