小白从零开始学rust编写cloudflare woker教程！

## 一、基础

#### 1、使用worker-rust模板创建rust的cloudflare worker：

```
npx wrangler generate my-project https://github.com/cloudflare/workers-sdk/templates/experimental/worker-rust
```

其中`my-project`是wrangler项目的名字，不是上传到`cloudflare workers`的项目名，cloudflare workers的项目名是在生成的`wrangler.toml`文件中的`name`字段修改。其它编程语言编写模板[link](https://github.com/cloudflare/workers-sdk/tree/main/templates/experimental)。

注意：

创建失败可以在于目录`C:\Users\{用户名}\AppData\Local\Temp\ `，将文件夹`wrangler-generate-repo-`开头的文件夹（包含里面的文件）删除，就能解决。通常创建失败是，之前您使用这命令创建过一次了，如果再次创建，只改项目名称，会创建失败的。

- 创建成功，怎么样的？

<img src="images\1.png" />

成功创建项目first-cloudflare-worker-rust，目录情况如下：

<img src="images\2.png" />

#### 2、修改src/lib.rs的代码

由于使用worker-rust模板创建，在没有修改情况下，`src/lib.rs`中的req、env、ctx没有使用到，需要在前面添加“_”，或将括号里面的参数都删除。

```
use worker::*;

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::ok("Hello, World!")
}
```

#### 3、运行下面命令，检查代码是否有问题：

```
cargo install -q worker-build && worker-build --dev
```

<img src="images\3.png" />

这种情况，说明代码没有错误。

#### 4、本地调试(运行)：

```
wrangler dev --env dev
或
npm run dev
```

<img src="images\4-1.png" />

<img src="images\4-2.png" />

- 运行结果：

<img src="images\5.png" />

#### 5、将代码部署到cloudflare worker中，执行命令：

```
wrangler deploy
或：
npm run deploy
```

<img src="images\6.png" />

<img src="images\7.png" />

![](images\8.png)

## 二、进阶

怎么从GET请求的url中获取参数呢？下面以生成IPv4 CIDR范围内所有IPv4地址为例子。

#### 1、在Cargo.toml文件中添加第三方依赖库：

```
ipnetwork = "0.20.0"
```

<img src="images\a1.png" />

#### 2、修改src/lib.rs的代码：

```
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
```

#### 3、本地调试(运行)：

```
wrangler dev --env dev
或
npm run dev
```

![](C:\Users\JuerSon\Desktop\新建文件夹 (2)\images\a2.png)

#### 4、确定代码没有问题，就发布到cloudflare worker服务器中，执行命令：

```
wrangler deploy
```

![](C:\Users\JuerSon\Desktop\新建文件夹 (2)\images\a3.png)
